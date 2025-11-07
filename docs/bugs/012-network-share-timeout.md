# Bug #012: 网络共享（Samba）文件处理超时

## 问题描述

处理网络共享（Samba）上的视频文件时，应用在"正在获取视频信息..."步骤卡住，无响应。

### 症状

- 日志显示：`正在获取视频信息...`
- 之后没有任何输出，应用卡住
- 网络路径格式：`\\192.168.10.172\sambaShare\...`

### 原因分析

1. **FFmpeg 探测行为**：
   - FFmpeg 需要读取视频文件的元数据（文件头）
   - 对于网络文件，FFmpeg 可能需要先下载部分数据
   - 如果网络慢或有问题，会长时间等待

2. **没有超时机制**：
   - 原始实现使用同步 `Command::output()`
   - 没有超时设置，会一直等待直到完成或失败

3. **FFmpeg 参数问题**：
   - 没有限制探测时间和大小
   - FFmpeg 可能会尝试读取整个文件来获取信息

## 解决方案

### 1. 添加超时机制

使用 `tokio::time::timeout` 为 FFmpeg 命令添加超时：
- 网络文件：60 秒超时
- 本地文件：30 秒超时

### 2. 优化 FFmpeg 参数

对于网络文件，添加以下参数限制探测：
```rust
"-analyzeduration", "10000000",  // 限制分析时长为 10 秒（微秒）
"-probesize", "10000000",        // 限制探测大小为 10MB
```

### 3. 异步处理

将 `get_video_info` 改为异步版本 `get_video_info_async`：
- 使用 `tokio::process::Command` 替代 `std::process::Command`
- 支持超时和更好的错误处理

### 4. 网络路径检测

添加 `is_network_path()` 函数检测 UNC 路径：
```rust
fn is_network_path(path: &Path) -> bool {
    if let Some(path_str) = path.to_str() {
        path_str.starts_with("\\\\") || path_str.starts_with("//")
    } else {
        false
    }
}
```

## 代码变更

### 文件：`src-tauri/src/extractor.rs`

1. **添加依赖**：
   ```rust
   use tokio::process::Command as TokioCommand;
   use tokio::time::{timeout, Duration};
   ```

2. **新增异步函数**：
   ```rust
   async fn get_video_info_async(
       app: &AppHandle,
       ffmpeg_path: &Path,
       video_path: &Path,
   ) -> Result<(f64, f64)>
   ```

3. **网络路径检测**：
   ```rust
   fn is_network_path(path: &Path) -> bool
   ```

4. **更新调用**：
   ```rust
   let (duration, _fps) = get_video_info_async(app, &ffmpeg_path, video_path).await?;
   ```

## 性能影响

- **超时保护**：避免无限等待
- **网络优化**：限制探测大小，减少网络传输
- **用户体验**：提供明确的错误信息和超时提示

## 使用建议

### 对于网络文件

1. **确保网络稳定**：检查网络连接和 Samba 服务器状态
2. **考虑本地缓存**：对于大文件，考虑先复制到本地再处理
3. **调整超时时间**：如果网络很慢，可以增加超时时间（修改代码中的 `Duration::from_secs(60)`）

### 错误处理

如果超时，会显示明确的错误信息：
- 网络文件：`获取网络视频信息超时（60秒）。请检查网络连接或尝试将文件复制到本地。`
- 本地文件：`获取视频信息超时（30秒）。`

## 测试验证

### 测试场景
1. **网络共享文件**：`\\192.168.10.172\...\video.mp4`
2. **本地文件**：`C:\Videos\video.mp4`
3. **超时测试**：断开网络连接，验证超时机制

### 验证点
- ✅ 网络文件能正常获取信息
- ✅ 超时机制正常工作
- ✅ 错误信息清晰明确
- ✅ 日志输出详细

## 相关文档
- [FFmpeg 参数文档](https://ffmpeg.org/ffmpeg.html)
- [Tokio 超时文档](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html)

## 状态

✅ 已修复（基础方案）

**注意**：如果仍然超时，请参考 [Bug #013: 网络共享文件获取视频信息超时深度分析](./013-network-share-timeout-deep-analysis.md) 了解更详细的优化方案。

## 修复日期

2025-11-07

