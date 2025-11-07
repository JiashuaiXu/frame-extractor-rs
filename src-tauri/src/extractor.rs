use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tokio::fs;
use tokio::process::Command as TokioCommand;
use tokio::time::{timeout, Duration};
use tauri::{AppHandle, Emitter};
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessResult {
    pub video_path: String,
    pub output_dir: String,
    pub frames_extracted: usize,
    pub success: bool,
    pub error: Option<String>,
}

/// 获取 FFmpeg 可执行文件路径
fn get_ffmpeg_path() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        // 开发环境：先尝试系统 PATH 中的 ffmpeg
        if let Ok(output) = Command::new("ffmpeg").arg("-version").output() {
            if output.status.success() {
                return PathBuf::from("ffmpeg");
            }
        }
        // 开发环境：尝试从 src-tauri/bin 目录
        let dev_path = PathBuf::from("src-tauri/bin/ffmpeg.exe");
        if dev_path.exists() {
            return dev_path;
        }
        // 最后尝试当前目录的 bin 目录
        let dev_path2 = PathBuf::from("bin/ffmpeg.exe");
        if dev_path2.exists() {
            return dev_path2;
        }
    }
    
    #[cfg(not(debug_assertions))]
    {
        // 生产环境：从 exe 目录查找
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Tauri 会将 externalBin 中的文件复制到应用目录
                // 注意：Tauri 可能会自动添加 .exe 后缀，导致文件名变成 ffmpeg.exe.exe
                let possible_paths = vec![
                    exe_dir.join("ffmpeg.exe"),           // 标准名称
                    exe_dir.join("ffmpeg.exe.exe"),      // Tauri 可能添加的 .exe 后缀
                    exe_dir.join("bin").join("ffmpeg.exe"),
                    exe_dir.join("bin").join("ffmpeg.exe.exe"),
                    exe_dir.join("resources").join("bin").join("ffmpeg.exe"),
                    exe_dir.join("resources").join("bin").join("ffmpeg.exe.exe"),
                    // Tauri 打包后的常见位置
                    exe_dir.parent()
                        .map(|p| p.join("resources").join("bin").join("ffmpeg.exe"))
                        .unwrap_or_default(),
                    exe_dir.parent()
                        .map(|p| p.join("resources").join("bin").join("ffmpeg.exe.exe"))
                        .unwrap_or_default(),
                ];
                
                for path in possible_paths {
                    if path.exists() {
                        return path;
                    }
                }
            }
        }
    }
    
    // 最后尝试系统 PATH 中的 ffmpeg
    PathBuf::from("ffmpeg")
}

/// 配置 FFmpeg 命令以隐藏窗口（Windows）
fn configure_ffmpeg_command(cmd: &mut Command) {
    #[cfg(windows)]
    {
        // CREATE_NO_WINDOW: 0x08000000 - 不创建新窗口
        // CREATE_NEW_PROCESS_GROUP: 0x00000200 - 创建新的进程组
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    // 重定向标准输出和错误，避免显示在控制台
    cmd.stdout(Stdio::piped())
       .stderr(Stdio::piped());
}

/// 检查 FFmpeg 是否可用
fn check_ffmpeg() -> Result<()> {
    let ffmpeg_path = get_ffmpeg_path();
    let mut cmd = Command::new(&ffmpeg_path);
    cmd.arg("-version");
    configure_ffmpeg_command(&mut cmd);
    
    let output = cmd
        .output()
        .with_context(|| format!("无法执行 FFmpeg: {}", ffmpeg_path.display()))?;
    
    if !output.status.success() {
        anyhow::bail!("FFmpeg 执行失败");
    }
    
    Ok(())
}

/// 发送日志事件到前端
fn emit_log(app: &AppHandle, level: &str, message: &str) {
    let _ = app.emit("log", serde_json::json!({
        "level": level,
        "message": message,
        "timestamp": chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
    }));
}

/// 发送进度事件到前端
fn emit_progress(app: &AppHandle, current: usize, total: usize, message: &str) {
    let percentage = if total > 0 {
        (current as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };
    let _ = app.emit("progress", serde_json::json!({
        "current": current,
        "total": total,
        "percentage": percentage,
        "message": message,
    }));
}

pub async fn extract_frames(
    app: &AppHandle,
    input_dir: &str,
    output_dir: &str,
    skip_start_sec: u64,
    frame_interval_sec: u64,
    preserve_dir_structure: bool,
    create_video_subdir: bool,
) -> Result<Vec<ProcessResult>> {
    emit_log(app, "info", &format!("开始处理视频抽帧任务"));
    emit_log(app, "info", &format!("输入目录: {}", input_dir));
    emit_log(app, "info", &format!("输出目录: {}", output_dir));
    emit_log(app, "info", &format!("跳过开头: {} 秒", skip_start_sec));
    emit_log(app, "info", &format!("抽帧间隔: {} 秒", frame_interval_sec));

    // 检查 FFmpeg 是否可用
    emit_log(app, "info", "检查 FFmpeg 是否可用...");
    check_ffmpeg().context("FFmpeg 未找到或不可用。请确保 FFmpeg 已安装或在应用目录中")?;
    emit_log(app, "success", "FFmpeg 检查通过");

    let input_path = Path::new(input_dir);
    if !input_path.exists() {
        emit_log(app, "error", &format!("输入目录不存在: {}", input_dir));
        anyhow::bail!("输入目录不存在: {}", input_dir);
    }

    // 递归查找所有 .mp4 文件
    emit_log(app, "info", "正在扫描视频文件...");
    let video_files = find_mp4_files(input_path).await?;
    if video_files.is_empty() {
        emit_log(app, "error", "未找到任何 MP4 视频文件");
        anyhow::bail!("未找到任何 MP4 视频文件");
    }
    emit_log(app, "success", &format!("找到 {} 个视频文件", video_files.len()));

    let mut results = Vec::new();
    let total_videos = video_files.len();

    for (index, video_path) in video_files.iter().enumerate() {
        let video_name = video_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("未知视频");
        emit_log(app, "info", &format!("[{}/{}] 处理视频: {}", index + 1, total_videos, video_name));
        emit_progress(app, index, total_videos, &format!("处理中: {}", video_name));
        let output_dir_path = get_output_dir_for_video(
            &video_path,
            input_path,
            Path::new(output_dir),
            preserve_dir_structure,
            create_video_subdir,
        );

        match extract_frames_from_video(
            app,
            &video_path,
            &output_dir_path,
            skip_start_sec,
            frame_interval_sec,
        )
        .await
        {
            Ok(count) => {
                emit_log(app, "success", &format!("✓ 完成: {} (提取了 {} 帧)", video_name, count));
                results.push(ProcessResult {
                    video_path: video_path.to_string_lossy().to_string(),
                    output_dir: output_dir_path.to_string_lossy().to_string(),
                    frames_extracted: count,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                emit_log(app, "error", &format!("✗ 失败: {} - {}", video_name, e));
                results.push(ProcessResult {
                    video_path: video_path.to_string_lossy().to_string(),
                    output_dir: output_dir_path.to_string_lossy().to_string(),
                    frames_extracted: 0,
                    success: false,
                    error: Some(e.to_string()),
                });
            }
        }
    }

    let total_frames: usize = results.iter().map(|r| r.frames_extracted).sum();
    let successful = results.iter().filter(|r| r.success).count();
    emit_log(app, "success", &format!("所有视频处理完成！成功: {}/{}，共提取 {} 帧", successful, total_videos, total_frames));
    emit_progress(app, total_videos, total_videos, "处理完成");

    Ok(results)
}

async fn find_mp4_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut entries = fs::read_dir(&dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|s| s.to_str()) == Some("mp4") {
                files.push(path);
            }
        }
    }

    Ok(files)
}

fn get_output_dir_for_video(
    video_path: &Path,
    input_root: &Path,
    output_root: &Path,
    preserve_dir_structure: bool,
    create_video_subdir: bool,
) -> PathBuf {
    let mut output_dir = output_root.to_path_buf();

    // 1. 是否保留原始子目录结构？
    if preserve_dir_structure {
        if let Some(parent) = video_path.parent() {
            if let Ok(rel_path) = parent.strip_prefix(input_root) {
                output_dir = output_dir.join(rel_path);
            }
        }
    }

    // 2. 是否为每个视频建独立子目录？
    if create_video_subdir {
        if let Some(stem) = video_path.file_stem().and_then(|s| s.to_str()) {
            output_dir = output_dir.join(stem);
        }
    }

    output_dir
}

async fn extract_frames_from_video(
    app: &AppHandle,
    video_path: &Path,
    output_dir: &Path,
    skip_start_sec: u64,
    frame_interval_sec: u64,
) -> Result<usize> {
    // 确保输出目录存在
    fs::create_dir_all(output_dir).await?;

    let ffmpeg_path = get_ffmpeg_path();
    let base_name = video_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("video");

    // 首先获取视频时长和 FPS（使用异步版本，支持超时）
    emit_log(app, "info", "正在获取视频信息...");
    let (duration, _fps) = get_video_info_async(app, &ffmpeg_path, video_path).await?;
    emit_log(app, "info", &format!("视频时长: {:.2} 秒", duration));
    
    // 计算需要提取的帧数
    let total_frames = ((duration - skip_start_sec as f64) / frame_interval_sec as f64).ceil() as usize;
    let total_frames = total_frames.max(0);
    
    if total_frames == 0 {
        emit_log(app, "warn", "无需提取帧（视频时长不足或跳过时间过长）");
        return Ok(0);
    }
    
    emit_log(app, "info", &format!("预计提取 {} 帧", total_frames));
    emit_log(app, "info", "开始提取帧...");

    // 优化：使用单次 FFmpeg 调用提取所有帧，而不是每帧调用一次
    // 使用 select 过滤器：选择满足条件的时间点
    // 这样可以一次性提取所有需要的帧，大大提高性能（从 N 次调用减少到 1 次）
    
    let output_pattern = output_dir.join(format!("{}_%04d.jpg", base_name));
    
    // 构建 FFmpeg 命令
    // -ss: 从指定时间开始（优化：先定位到开始时间，再处理）
    // -i: 输入文件
    // -vf select: 使用 select 过滤器选择帧
    //   - gte(t, start): 从开始时间之后
    //   - mod(t-start, interval) < 0.1: 每隔 interval 秒选择一帧（容差 0.1 秒）
    // -vsync vfr: 可变帧率输出
    // -q:v 2: 高质量 JPEG
    let filter = format!(
        "select='gte(t,{})*lt(mod(t-{},{}),0.1)'",
        skip_start_sec,
        skip_start_sec,
        frame_interval_sec
    );
    
    let mut cmd = Command::new(&ffmpeg_path);
    cmd.args(&[
        "-ss", &format!("{}", skip_start_sec),  // 先定位到开始时间（优化）
        "-i", video_path.to_string_lossy().as_ref(),
        "-vf", &filter,
        "-vsync", "vfr",  // 可变帧率，只输出选中的帧
        "-q:v", "2",      // 高质量 JPEG
        "-y",             // 覆盖已存在的文件
        output_pattern.to_string_lossy().as_ref(),
    ]);
    
    configure_ffmpeg_command(&mut cmd);
    
    emit_log(app, "debug", "执行 FFmpeg 命令...");
    
    let output = cmd
        .output()
        .with_context(|| format!("提取帧失败: {}", video_path.display()))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        emit_log(app, "error", &format!("FFmpeg 执行失败: {}", error_msg));
        anyhow::bail!("FFmpeg 执行失败: {}", error_msg);
    }
    
    // 输出 FFmpeg 的 stderr（包含处理信息）
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.trim().is_empty() {
        // 解析并显示 FFmpeg 输出中的关键信息
        for line in stderr.lines() {
            if line.contains("frame=") || line.contains("fps=") || line.contains("time=") {
                emit_log(app, "debug", &format!("FFmpeg: {}", line.trim()));
            }
        }
    }

    // 统计实际提取的帧数
    let mut saved_count = 0;
    let mut frame_num = 1;
    loop {
        let img_path = output_dir.join(format!("{}_{:04}.jpg", base_name, frame_num));
        if img_path.exists() {
            saved_count += 1;
            frame_num += 1;
        } else {
            break;
        }
    }

    Ok(saved_count)
}

/// 检查是否为网络路径（UNC 路径）
fn is_network_path(path: &Path) -> bool {
    if let Some(path_str) = path.to_str() {
        path_str.starts_with("\\\\") || path_str.starts_with("//")
    } else {
        false
    }
}

/// 获取视频信息（时长和 FPS）- 异步版本，支持超时
async fn get_video_info_async(
    app: &AppHandle,
    ffmpeg_path: &Path,
    video_path: &Path,
) -> Result<(f64, f64)> {
    let is_network = is_network_path(video_path);
    
    if is_network {
        emit_log(app, "info", "检测到网络路径，使用优化的 FFmpeg 参数...");
    }
    
    // 构建 FFmpeg 命令
    let mut cmd = TokioCommand::new(ffmpeg_path);
    
    // 对于网络文件，使用更激进的参数限制探测，避免长时间等待
    // 注意：即使限制了探测大小，如果网络很慢，读取几MB也可能需要很长时间
    if is_network {
        cmd.args(&[
            "-analyzeduration", "5000000",   // 限制分析时长为 5 秒（微秒）- 更短
            "-probesize", "2097152",        // 限制探测大小为 2MB（字节）- 更小，只读取文件头
            "-readrate", "10M",             // 限制读取速度为 10MB/s，避免网络拥塞
            "-i", video_path.to_string_lossy().as_ref(),
            "-f", "null",
            "-",
        ]);
    } else {
        cmd.args(&[
            "-i", video_path.to_string_lossy().as_ref(),
            "-f", "null",
            "-",
        ]);
    }
    
    // 配置命令（隐藏窗口、重定向输出）
    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd.stdout(Stdio::piped())
       .stderr(Stdio::piped());
    
    emit_log(app, "debug", "执行 FFmpeg 获取视频信息...");
    
    // 设置超时：网络文件 30 秒（更短，因为已经限制了探测大小），本地文件 15 秒
    // 如果网络很慢，30秒内无法读取2MB，说明网络连接有问题
    let timeout_duration = if is_network {
        Duration::from_secs(30)
    } else {
        Duration::from_secs(15)
    };
    
    let output_result = timeout(timeout_duration, cmd.output()).await;
    
    let duration_output = match output_result {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => {
            emit_log(app, "error", &format!("FFmpeg 执行失败: {}", e));
            anyhow::bail!("无法执行 FFmpeg 获取视频信息: {}", e);
        }
        Err(_) => {
            let timeout_msg = if is_network {
                "获取网络视频信息超时（30秒）。网络可能太慢或连接不稳定。建议：1) 检查网络连接 2) 将文件复制到本地处理 3) 或跳过视频信息获取直接处理（可能影响帧数计算）"
            } else {
                "获取视频信息超时（15秒）。文件可能损坏或格式不支持。"
            };
            emit_log(app, "error", timeout_msg);
            
            // 对于网络文件，提供降级方案：跳过信息获取，使用默认值
            if is_network {
                emit_log(app, "warn", "尝试使用降级方案：跳过视频信息获取，使用估算值...");
                // 返回默认值，让处理继续（虽然可能不准确）
                return Ok((3600.0, 30.0)); // 默认1小时，30fps
            }
            
            anyhow::bail!("{}", timeout_msg);
        }
    };

    let stderr = String::from_utf8_lossy(&duration_output.stderr);
    
    // 输出 FFmpeg 的错误信息（如果有）
    if !duration_output.status.success() {
        let error_preview: String = stderr.lines().take(5).collect::<Vec<_>>().join("\n");
        emit_log(app, "warn", &format!("FFmpeg 警告/错误: {}", error_preview));
    }
    
    // 解析时长 (格式: Duration: HH:MM:SS.mm)
    let duration = parse_duration(&stderr).unwrap_or(0.0);
    
    if duration == 0.0 {
        emit_log(app, "warn", "无法解析视频时长，使用默认值 0");
    } else {
        emit_log(app, "debug", &format!("解析到视频时长: {:.2} 秒", duration));
    }
    
    // 解析 FPS (格式: fps, 25 fps, 等)
    let fps = parse_fps(&stderr).unwrap_or(30.0);

    Ok((duration, fps))
}


/// 从 FFmpeg 输出中解析时长
fn parse_duration(output: &str) -> Option<f64> {
    use regex::Regex;
    let re = Regex::new(r"Duration:\s*(\d{2}):(\d{2}):(\d{2})\.(\d{2})").ok()?;
    let caps = re.captures(output)?;
    
    let hours: f64 = caps.get(1)?.as_str().parse().ok()?;
    let minutes: f64 = caps.get(2)?.as_str().parse().ok()?;
    let seconds: f64 = caps.get(3)?.as_str().parse().ok()?;
    let centiseconds: f64 = caps.get(4)?.as_str().parse().ok()?;
    
    Some(hours * 3600.0 + minutes * 60.0 + seconds + centiseconds / 100.0)
}

/// 从 FFmpeg 输出中解析 FPS
fn parse_fps(output: &str) -> Option<f64> {
    use regex::Regex;
    // 匹配各种 FPS 格式: 25 fps, 29.97 fps, 30fps 等
    let re = Regex::new(r"(\d+\.?\d*)\s*fps").ok()?;
    let caps = re.captures(output)?;
    caps.get(1)?.as_str().parse().ok()
}
