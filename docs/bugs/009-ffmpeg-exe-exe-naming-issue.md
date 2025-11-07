# Bug 009: FFmpeg 文件名重复 .exe 后缀问题

**错误信息**：
```
处理失败: FFmpeg 未找到或不可用。请确保 FFmpeg 已安装或在应用目录中
```

**问题描述**：
应用安装后，在 `C:\Users\Administrator\AppData\Local\FrameExtractor` 目录下，FFmpeg 文件名变成了 `ffmpeg.exe.exe`（多了一个 `.exe` 后缀），而代码查找的是 `ffmpeg.exe`，导致找不到 FFmpeg。

**实际文件**：
```
C:\Users\Administrator\AppData\Local\FrameExtractor\
├── ffmpeg.exe.exe          # ❌ 文件名错误（多了一个 .exe）
├── frame-extractor-rs.exe
└── uninstall.exe
```

**根本原因**：
Tauri 在打包 `externalBin` 中的文件时，如果配置为 `bin/ffmpeg.exe`，可能会自动添加 `.exe` 后缀，导致最终文件名变成 `ffmpeg.exe.exe`。

**解决方案**：
修改 `src-tauri/src/extractor.rs` 中的 `get_ffmpeg_path()` 函数，添加对 `ffmpeg.exe.exe` 的查找支持：

```rust
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
                // ... 其他路径
            ];
            
            for path in possible_paths {
                if path.exists() {
                    return path;
                }
            }
        }
    }
}
```

**修改内容**：
- 在 `get_ffmpeg_path()` 函数中添加了对 `ffmpeg.exe.exe` 的查找
- 在所有可能的路径中都添加了对两种文件名的支持

**验证**：
1. 重新构建应用：`npm run tauri build`
2. 安装新构建的应用
3. 检查安装目录中的 FFmpeg 文件名
4. 运行应用，测试视频处理功能

**状态**：✅ 已修复

**相关文件**：
- `src-tauri/src/extractor.rs`

**备注**：
这是一个兼容性修复，确保无论 Tauri 如何命名文件，应用都能找到 FFmpeg。理想情况下，应该修复 Tauri 配置以避免文件名重复，但作为临时解决方案，代码层面的兼容性修复是必要的。

