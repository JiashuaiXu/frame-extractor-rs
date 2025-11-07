use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;

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

/// 检查 FFmpeg 是否可用
fn check_ffmpeg() -> Result<()> {
    let ffmpeg_path = get_ffmpeg_path();
    let output = Command::new(&ffmpeg_path)
        .arg("-version")
        .output()
        .with_context(|| format!("无法执行 FFmpeg: {}", ffmpeg_path.display()))?;
    
    if !output.status.success() {
        anyhow::bail!("FFmpeg 执行失败");
    }
    
    Ok(())
}

pub async fn extract_frames(
    input_dir: &str,
    output_dir: &str,
    skip_start_sec: u64,
    frame_interval_sec: u64,
    preserve_dir_structure: bool,
    create_video_subdir: bool,
) -> Result<Vec<ProcessResult>> {
    // 检查 FFmpeg 是否可用
    check_ffmpeg().context("FFmpeg 未找到或不可用。请确保 FFmpeg 已安装或在应用目录中")?;

    let input_path = Path::new(input_dir);
    if !input_path.exists() {
        anyhow::bail!("输入目录不存在: {}", input_dir);
    }

    // 递归查找所有 .mp4 文件
    let video_files = find_mp4_files(input_path).await?;
    if video_files.is_empty() {
        anyhow::bail!("未找到任何 MP4 视频文件");
    }

    let mut results = Vec::new();

    for video_path in video_files {
        let output_dir_path = get_output_dir_for_video(
            &video_path,
            input_path,
            Path::new(output_dir),
            preserve_dir_structure,
            create_video_subdir,
        );

        match extract_frames_from_video(
            &video_path,
            &output_dir_path,
            skip_start_sec,
            frame_interval_sec,
        )
        .await
        {
            Ok(count) => {
                results.push(ProcessResult {
                    video_path: video_path.to_string_lossy().to_string(),
                    output_dir: output_dir_path.to_string_lossy().to_string(),
                    frames_extracted: count,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
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

    // 首先获取视频时长和 FPS
    let (duration, _fps) = get_video_info(&ffmpeg_path, video_path)?;
    
    // 计算需要提取的帧数
    let total_frames = ((duration - skip_start_sec as f64) / frame_interval_sec as f64).ceil() as usize;
    let total_frames = total_frames.max(0);
    
    if total_frames == 0 {
        return Ok(0);
    }

    let mut saved_count = 0;

    // 使用 FFmpeg 的 select 过滤器来提取帧
    // 每隔 frame_interval_sec 秒提取一帧，从 skip_start_sec 开始
    for i in 0..total_frames {
        let timestamp = skip_start_sec as f64 + (i as f64 * frame_interval_sec as f64);
        if timestamp >= duration {
            break;
        }

        saved_count += 1;
        let img_name = format!("{}_{}.jpg", base_name, saved_count);
        let img_path = output_dir.join(&img_name);

        // 使用 FFmpeg 提取指定时间点的帧
        let output = Command::new(&ffmpeg_path)
            .args(&[
                "-i", video_path.to_string_lossy().as_ref(),
                "-ss", &format!("{:.3}", timestamp),
                "-vframes", "1",
                "-q:v", "2", // 高质量 JPEG
                "-y", // 覆盖已存在的文件
                img_path.to_string_lossy().as_ref(),
            ])
            .output()
            .with_context(|| format!("提取帧失败: {}", img_path.display()))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("FFmpeg 执行失败: {}", error_msg);
        }
    }

    Ok(saved_count)
}

/// 获取视频信息（时长和 FPS）
fn get_video_info(ffmpeg_path: &Path, video_path: &Path) -> Result<(f64, f64)> {
    // 获取视频时长
    let duration_output = Command::new(ffmpeg_path)
        .args(&[
            "-i", video_path.to_string_lossy().as_ref(),
            "-f", "null",
            "-",
        ])
        .stderr(std::process::Stdio::piped())
        .output()
        .context("无法执行 FFmpeg 获取视频信息")?;

    let stderr = String::from_utf8_lossy(&duration_output.stderr);
    
    // 解析时长 (格式: Duration: HH:MM:SS.mm)
    let duration = parse_duration(&stderr).unwrap_or(0.0);
    
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
