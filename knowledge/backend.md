# 后端技术栈 (Rust)

## 概述

后端使用 **Rust** 语言开发，通过 **Tauri 框架** 与前端通信。

---

## 技术栈

### 核心

- **Rust** - 编程语言
- **Tauri 2.0** - 桌面应用框架
- **Tokio** - 异步运行时

### 依赖库

- `tauri` - 框架核心
- `tauri-plugin-*` - 功能插件
- `serde` - 序列化
- `tokio` - 异步 I/O
- `anyhow` - 错误处理
- `regex` - 正则表达式

---

## 项目结构

```
src-tauri/src/
├── main.rs        # 应用入口
├── extractor.rs   # 核心业务逻辑
└── build.rs       # 构建脚本
```

---

## 应用入口 (main.rs)

### 代码结构

```rust
// 1. 模块声明
mod extractor;
use extractor::extract_frames;

// 2. Tauri 命令
#[tauri::command]
async fn process_videos(...) -> Result<Vec<ProcessResult>, String> {
    extract_frames(...).await.map_err(|e| e.to_string())
}

// 3. 主函数
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())  // 对话框插件
        .plugin(tauri_plugin_fs::init())     // 文件系统插件
        .invoke_handler(tauri::generate_handler![
            process_videos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 关键组件

1. **模块系统**: `mod extractor;`
2. **Tauri 命令**: `#[tauri::command]`
3. **插件初始化**: `.plugin()`
4. **命令注册**: `.invoke_handler()`

---

## 核心逻辑 (extractor.rs)

### 主要功能

1. **视频文件查找**: 递归查找 MP4 文件
2. **帧提取**: 使用 FFmpeg 提取帧
3. **目录管理**: 处理输出目录结构
4. **错误处理**: 统一的错误处理

### 函数结构

```rust
// 公共 API
pub async fn extract_frames(...) -> Result<Vec<ProcessResult>>

// 内部函数
async fn find_mp4_files(root: &Path) -> Result<Vec<PathBuf>>
async fn extract_frames_from_video(...) -> Result<usize>
fn get_output_dir_for_video(...) -> PathBuf
fn get_video_info(...) -> Result<(f64, f64)>
fn parse_duration(...) -> Option<f64>
fn parse_fps(...) -> Option<f64>
fn get_ffmpeg_path() -> PathBuf
fn check_ffmpeg() -> Result<()>
```

---

## 异步编程

### async/await

```rust
// 异步函数
pub async fn extract_frames(
    input_dir: &str,
    output_dir: &str,
    // ...
) -> Result<Vec<ProcessResult>> {
    // 异步文件操作
    let video_files = find_mp4_files(input_path).await?;
    
    // 异步处理
    for video_path in video_files {
        match extract_frames_from_video(...).await {
            Ok(count) => { /* ... */ },
            Err(e) => { /* ... */ },
        }
    }
    
    Ok(results)
}
```

### Tokio 运行时

```rust
// 使用 tokio::fs
use tokio::fs;

async fn find_mp4_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut entries = fs::read_dir(&dir).await?;  // 异步读取目录
    // ...
}
```

---

## FFmpeg 集成

### 路径查找

```rust
fn get_ffmpeg_path() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        // 开发环境: 从项目目录查找
        // 1. 系统 PATH
        // 2. src-tauri/bin/ffmpeg.exe
        // 3. bin/ffmpeg.exe
    }
    
    #[cfg(not(debug_assertions))]
    {
        // 生产环境: 从应用目录查找
        // 1. 应用目录/ffmpeg.exe
        // 2. 应用目录/bin/ffmpeg.exe
        // 3. 应用目录/resources/bin/ffmpeg.exe
    }
}
```

### 命令执行

```rust
let output = Command::new(&ffmpeg_path)
    .args(&[
        "-i", video_path.to_string_lossy().as_ref(),
        "-ss", &format!("{:.3}", timestamp),
        "-vframes", "1",
        "-q:v", "2",
        "-y",
        img_path.to_string_lossy().as_ref(),
    ])
    .output()
    .with_context(|| format!("提取帧失败: {}", img_path.display()))?;
```

---

## 错误处理

### anyhow 使用

```rust
use anyhow::{Context, Result};

// 带上下文的错误
let output = Command::new(&ffmpeg_path)
    .arg("-version")
    .output()
    .with_context(|| format!("无法执行 FFmpeg: {}", ffmpeg_path.display()))?;

// 错误传播
if !output.status.success() {
    anyhow::bail!("FFmpeg 执行失败");
}
```

### 错误转换

```rust
// Tauri 命令需要返回 String 错误
#[tauri::command]
async fn process_videos(...) -> Result<Vec<ProcessResult>, String> {
    extract_frames(...)
        .await
        .map_err(|e| e.to_string())  // 转换为 String
}
```

---

## 数据结构

### ProcessResult

```rust
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessResult {
    pub video_path: String,
    pub output_dir: String,
    pub frames_extracted: usize,
    pub success: bool,
    pub error: Option<String>,
}
```

**特点**:
- `Serialize`: 可序列化为 JSON
- `Clone`: 可克隆
- `Debug`: 可调试打印

---

## 文件操作

### 递归查找

```rust
async fn find_mp4_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];  // 使用栈模拟递归
    
    while let Some(dir) = stack.pop() {
        let mut entries = fs::read_dir(&dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);  // 目录入栈
            } else if path.extension().and_then(|s| s.to_str()) == Some("mp4") {
                files.push(path);  // MP4 文件
            }
        }
    }
    
    Ok(files)
}
```

### 目录创建

```rust
// 创建输出目录
fs::create_dir_all(output_dir).await?;
```

---

## 文本解析

### 正则表达式

```rust
use regex::Regex;

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

---

## 性能优化

### 1. 异步 I/O

- 使用 `tokio::fs` 进行异步文件操作
- 非阻塞 I/O 提高并发性能

### 2. 错误处理

- 使用 `anyhow` 简化错误处理
- 提供有意义的错误消息

### 3. 内存管理

- 使用引用避免数据复制
- 及时释放资源

---

## 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_duration() {
        let output = "Duration: 00:01:30.50";
        assert_eq!(parse_duration(output), Some(90.5));
    }
}
```

### 运行测试

```bash
cargo test
```

---

## 相关文件

- `src-tauri/src/main.rs` - 应用入口
- `src-tauri/src/extractor.rs` - 核心逻辑
- `src-tauri/Cargo.toml` - 依赖配置
- `src-tauri/build.rs` - 构建脚本

---

## 学习资源

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio 文档](https://tokio.rs/)
- [Tauri 文档](https://tauri.app/)
- [anyhow 文档](https://docs.rs/anyhow/)

