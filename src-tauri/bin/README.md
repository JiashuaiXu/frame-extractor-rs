# FFmpeg 二进制文件

请将 FFmpeg 可执行文件放置在此目录中。

## 获取 FFmpeg

### Windows
1. 访问 https://www.gyan.dev/ffmpeg/builds/ 或 https://github.com/BtbN/FFmpeg-Builds/releases
2. 下载 Windows 版本的 FFmpeg（推荐使用 `ffmpeg-release-full.7z`）
3. 解压后，将 `bin/ffmpeg.exe` 复制到此目录（`src-tauri/bin/ffmpeg.exe`）

### Tauri 2.0 命名要求

**重要**：Tauri 2.0 要求外部二进制文件必须按照目标三元组命名。

对于 Windows x64，需要创建以下文件：

```powershell
# 1. 将下载的 ffmpeg.exe 复制到此目录
Copy-Item "path\to\ffmpeg.exe" "src-tauri\bin\ffmpeg.exe"

# 2. 创建目标三元组命名的副本（必需！）
Copy-Item "src-tauri\bin\ffmpeg.exe" "src-tauri\bin\ffmpeg.exe-x86_64-pc-windows-msvc.exe"
```

**文件结构**：
```
src-tauri/bin/
├── ffmpeg.exe                              # 原始文件
└── ffmpeg.exe-x86_64-pc-windows-msvc.exe  # Tauri 2.0 要求的目标三元组命名
```

### 快速设置脚本

```powershell
# 假设你已经下载了 ffmpeg.exe 到某个位置
$ffmpegSource = "C:\path\to\ffmpeg.exe"  # 修改为你的 FFmpeg 路径
$binDir = "src-tauri\bin"

# 复制文件
Copy-Item $ffmpegSource "$binDir\ffmpeg.exe" -Force

# 创建目标三元组命名版本
Copy-Item "$binDir\ffmpeg.exe" "$binDir\ffmpeg.exe-x86_64-pc-windows-msvc.exe" -Force

Write-Host "FFmpeg 文件已配置完成！"
```

## 注意事项

- ✅ 文件必须命名为 `ffmpeg.exe`（Windows）
- ✅ **必须创建** `ffmpeg.exe-x86_64-pc-windows-msvc.exe`（Tauri 2.0 要求）
- ✅ 确保文件有执行权限
- ✅ 构建应用时，此文件会被自动打包到应用中
- ⚠️ 如果使用 Git LFS，两个文件都需要添加到 LFS 跟踪
