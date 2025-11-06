# FFmpeg 集成方案

## 概述

本项目使用 **FFmpeg** 作为视频处理引擎，通过命令行调用进行视频帧提取。

---

## FFmpeg 简介

### 什么是 FFmpeg？

**FFmpeg** 是一个跨平台的多媒体处理框架：

- **功能**: 视频/音频编解码、转换、处理
- **格式**: 支持几乎所有视频/音频格式
- **许可证**: LGPL/GPL
- **平台**: Windows/macOS/Linux

### 为什么选择 FFmpeg？

1. **功能强大**: 支持所有主流视频格式
2. **性能优异**: C 语言编写，性能优秀
3. **跨平台**: 支持所有主要平台
4. **命令行接口**: 易于集成到 Rust 应用中

---

## 集成方案

### 方案选择

本项目采用 **外部二进制文件** 方案：

- ✅ FFmpeg 作为独立可执行文件
- ✅ 打包到应用中
- ✅ 运行时动态查找和调用

### 替代方案

| 方案 | 优点 | 缺点 |
|------|------|------|
| **外部二进制** (当前) | 简单、灵活 | 文件较大 |
| **静态链接** | 单文件 | 编译复杂 |
| **动态库** | 可共享 | 依赖管理复杂 |
| **Rust 绑定** | 类型安全 | 维护成本高 |

---

## 配置

### Tauri 配置

```json
{
  "bundle": {
    "externalBin": ["bin/ffmpeg.exe"]
  }
}
```

**说明**:
- 文件会被复制到应用资源目录
- 需要按目标三元组命名

### 文件命名

```
src-tauri/bin/
├── ffmpeg.exe                              # 原始文件
└── ffmpeg.exe-x86_64-pc-windows-msvc.exe  # Tauri 2.0 要求
```

---

## 路径查找逻辑

### 开发环境

```rust
#[cfg(debug_assertions)]
fn get_ffmpeg_path() -> PathBuf {
    // 1. 系统 PATH
    if Command::new("ffmpeg").arg("-version").output().is_ok() {
        return PathBuf::from("ffmpeg");
    }
    
    // 2. 项目目录
    let dev_path = PathBuf::from("src-tauri/bin/ffmpeg.exe");
    if dev_path.exists() {
        return dev_path;
    }
    
    // 3. 当前目录
    let dev_path2 = PathBuf::from("bin/ffmpeg.exe");
    if dev_path2.exists() {
        return dev_path2;
    }
    
    PathBuf::from("ffmpeg")  // 最后尝试系统 PATH
}
```

### 生产环境

```rust
#[cfg(not(debug_assertions))]
fn get_ffmpeg_path() -> PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // 按优先级查找
            let possible_paths = vec![
                exe_dir.join("ffmpeg.exe"),
                exe_dir.join("bin").join("ffmpeg.exe"),
                exe_dir.join("resources").join("bin").join("ffmpeg.exe"),
            ];
            
            for path in possible_paths {
                if path.exists() {
                    return path;
                }
            }
        }
    }
    
    PathBuf::from("ffmpeg")  // 最后尝试系统 PATH
}
```

---

## 命令执行

### 检查 FFmpeg

```rust
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
```

### 提取单帧

```rust
let output = Command::new(&ffmpeg_path)
    .args(&[
        "-i", video_path.to_string_lossy().as_ref(),  // 输入文件
        "-ss", &format!("{:.3}", timestamp),        // 时间点
        "-vframes", "1",                             // 提取 1 帧
        "-q:v", "2",                                 // 质量 (2 = 高质量)
        "-y",                                        // 覆盖已存在文件
        img_path.to_string_lossy().as_ref(),        // 输出文件
    ])
    .output()
    .with_context(|| format!("提取帧失败: {}", img_path.display()))?;
```

### 获取视频信息

```rust
// 获取视频时长和 FPS
let output = Command::new(ffmpeg_path)
    .args(&[
        "-i", video_path.to_string_lossy().as_ref(),
        "-f", "null",
        "-",
    ])
    .stderr(std::process::Stdio::piped())
    .output()
    .context("无法执行 FFmpeg 获取视频信息")?;

let stderr = String::from_utf8_lossy(&output.stderr);
let duration = parse_duration(&stderr)?;
let fps = parse_fps(&stderr)?;
```

---

## 输出解析

### 解析时长

```rust
fn parse_duration(output: &str) -> Option<f64> {
    let re = Regex::new(r"Duration:\s*(\d{2}):(\d{2}):(\d{2})\.(\d{2})").ok()?;
    let caps = re.captures(output)?;
    
    let hours: f64 = caps.get(1)?.as_str().parse().ok()?;
    let minutes: f64 = caps.get(2)?.as_str().parse().ok()?;
    let seconds: f64 = caps.get(3)?.as_str().parse().ok()?;
    let centiseconds: f64 = caps.get(4)?.as_str().parse().ok()?;
    
    Some(hours * 3600.0 + minutes * 60.0 + seconds + centiseconds / 100.0)
}
```

**FFmpeg 输出格式**:
```
Duration: 00:01:30.50, start: 0.000000, bitrate: 1000 kb/s
```

### 解析 FPS

```rust
fn parse_fps(output: &str) -> Option<f64> {
    let re = Regex::new(r"(\d+\.?\d*)\s*fps").ok()?;
    let caps = re.captures(output)?;
    caps.get(1)?.as_str().parse().ok()
}
```

**FFmpeg 输出格式**:
```
25 fps, 29.97 fps, 30fps
```

---

## 错误处理

### 执行失败

```rust
if !output.status.success() {
    let error_msg = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("FFmpeg 执行失败: {}", error_msg);
}
```

### 文件不存在

```rust
let ffmpeg_path = get_ffmpeg_path();
if !ffmpeg_path.exists() {
    anyhow::bail!("FFmpeg 未找到: {}", ffmpeg_path.display());
}
```

---

## 性能优化

### 1. 批量处理

- 顺序处理每个视频
- 可以改为并行处理（使用 `tokio::spawn`）

### 2. 缓存路径

- 首次查找后缓存 FFmpeg 路径
- 避免重复查找

### 3. 错误重试

- 失败后重试机制
- 超时处理

---

## 打包说明

### 自动打包

Tauri 会自动将 `externalBin` 中的文件打包：

1. **构建时**: 复制到应用资源目录
2. **安装时**: 复制到安装目录
3. **运行时**: 从安装目录查找

### 文件大小

- **FFmpeg**: ~50-100 MB
- **应用**: ~5-10 MB
- **总计**: ~60-110 MB

---

## 下载和配置

### 下载 FFmpeg

1. 访问 https://github.com/BtbN/FFmpeg-Builds/releases
2. 下载 `ffmpeg-master-latest-win64-gpl.zip`
3. 解压后找到 `bin/ffmpeg.exe`

### 配置

```powershell
# 复制到项目目录
Copy-Item "ffmpeg.exe" "src-tauri\bin\ffmpeg.exe"

# 创建目标三元组命名版本
Copy-Item "src-tauri\bin\ffmpeg.exe" "src-tauri\bin\ffmpeg.exe-x86_64-pc-windows-msvc.exe"
```

---

## 常见问题

### Q: FFmpeg 找不到？

**A**: 检查：
1. 文件是否存在
2. 路径是否正确
3. 权限是否足够

### Q: 执行失败？

**A**: 检查：
1. FFmpeg 版本是否支持
2. 视频文件格式是否支持
3. 错误输出信息

### Q: 性能问题？

**A**: 优化：
1. 使用并行处理
2. 减少不必要的参数
3. 优化输出质量

---

## 相关文件

- `src-tauri/src/extractor.rs` - FFmpeg 集成代码
- `src-tauri/bin/ffmpeg.exe` - FFmpeg 可执行文件
- `src-tauri/tauri.conf.json` - 配置文件

---

## 参考资源

- [FFmpeg 官方文档](https://ffmpeg.org/documentation.html)
- [FFmpeg 下载](https://github.com/BtbN/FFmpeg-Builds/releases)
- [FFmpeg 命令参考](https://ffmpeg.org/ffmpeg.html)

