# FFmpeg 二进制文件

请将 FFmpeg 可执行文件放置在此目录中。

## 获取 FFmpeg

### Windows
1. 访问 https://www.gyan.dev/ffmpeg/builds/ 或 https://github.com/BtbN/FFmpeg-Builds/releases
2. 下载 Windows 版本的 FFmpeg（推荐使用 `ffmpeg-release-full.7z`）
3. 解压后，将 `bin/ffmpeg.exe` 复制到此目录（`src-tauri/bin/ffmpeg.exe`）

### 快速下载（Windows）
```powershell
# 使用 PowerShell 下载 FFmpeg（示例）
# 请访问 https://www.gyan.dev/ffmpeg/builds/ 手动下载
# 或使用以下命令（需要先安装 7-Zip）：
# Invoke-WebRequest -Uri "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-full.7z" -OutFile "ffmpeg.7z"
# 然后解压并将 ffmpeg.exe 复制到此目录
```

## 注意事项

- 文件必须命名为 `ffmpeg.exe`（Windows）
- 确保文件有执行权限
- 构建应用时，此文件会被自动打包到应用中
