# FFmpeg 设置脚本
# 用于配置 FFmpeg 二进制文件以支持 Tauri 2.0 构建

$binDir = "src-tauri\bin"
$targetTriple = "x86_64-pc-windows-msvc"

Write-Host "=== FFmpeg 设置脚本 ===" -ForegroundColor Cyan
Write-Host ""

# 检查目标目录
if (-not (Test-Path $binDir)) {
    New-Item -ItemType Directory -Path $binDir -Force | Out-Null
    Write-Host "已创建目录: $binDir" -ForegroundColor Green
}

# 检查是否已存在文件
$ffmpegExe = Join-Path $binDir "ffmpeg.exe"
$ffmpegTarget = Join-Path $binDir "ffmpeg.exe-$targetTriple.exe"

if ((Test-Path $ffmpegExe) -and (Test-Path $ffmpegTarget)) {
    Write-Host "✓ FFmpeg 文件已存在" -ForegroundColor Green
    Write-Host "  - $ffmpegExe" -ForegroundColor Gray
    Write-Host "  - $ffmpegTarget" -ForegroundColor Gray
    Write-Host ""
    Write-Host "如果需要重新设置，请先删除现有文件。" -ForegroundColor Yellow
    exit 0
}

# 尝试从已安装的应用中复制
$installedPath = "$env:LOCALAPPDATA\FrameExtractor\ffmpeg.exe.exe"
if (Test-Path $installedPath) {
    Write-Host "发现已安装的 FFmpeg: $installedPath" -ForegroundColor Yellow
    $response = Read-Host "是否从此位置复制？(Y/N)"
    if ($response -eq "Y" -or $response -eq "y") {
        Copy-Item $installedPath $ffmpegExe -Force
        Copy-Item $ffmpegExe $ffmpegTarget -Force
        Write-Host "✓ FFmpeg 文件已复制" -ForegroundColor Green
        exit 0
    }
}

# 尝试从系统 PATH 中查找
$ffmpegInPath = Get-Command ffmpeg -ErrorAction SilentlyContinue
if ($ffmpegInPath) {
    Write-Host "发现系统 PATH 中的 FFmpeg: $($ffmpegInPath.Source)" -ForegroundColor Yellow
    $response = Read-Host "是否从此位置复制？(Y/N)"
    if ($response -eq "Y" -or $response -eq "y") {
        Copy-Item $ffmpegInPath.Source $ffmpegExe -Force
        Copy-Item $ffmpegExe $ffmpegTarget -Force
        Write-Host "✓ FFmpeg 文件已复制" -ForegroundColor Green
        exit 0
    }
}

# 提示用户手动下载
Write-Host ""
Write-Host "未找到 FFmpeg 文件，请手动下载并配置：" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. 访问以下任一网站下载 FFmpeg：" -ForegroundColor Cyan
Write-Host "   - https://www.gyan.dev/ffmpeg/builds/" -ForegroundColor White
Write-Host "   - https://github.com/BtbN/FFmpeg-Builds/releases" -ForegroundColor White
Write-Host ""
Write-Host "2. 下载 Windows 版本（推荐：ffmpeg-release-full.7z）" -ForegroundColor Cyan
Write-Host ""
Write-Host "3. 解压后，将 ffmpeg.exe 复制到以下位置：" -ForegroundColor Cyan
Write-Host "   $ffmpegExe" -ForegroundColor White
Write-Host ""
Write-Host "4. 然后运行以下命令创建目标三元组命名版本：" -ForegroundColor Cyan
Write-Host "   Copy-Item `"$ffmpegExe`" `"$ffmpegTarget`" -Force" -ForegroundColor White
Write-Host ""
Write-Host "或者，如果你已经下载了 ffmpeg.exe，请提供文件路径：" -ForegroundColor Yellow
$userPath = Read-Host "FFmpeg 文件路径（留空跳过）"
if ($userPath -and (Test-Path $userPath)) {
    Copy-Item $userPath $ffmpegExe -Force
    Copy-Item $ffmpegExe $ffmpegTarget -Force
    Write-Host "✓ FFmpeg 文件已复制" -ForegroundColor Green
    Write-Host "  - $ffmpegExe" -ForegroundColor Gray
    Write-Host "  - $ffmpegTarget" -ForegroundColor Gray
} else {
    Write-Host ""
    Write-Host "请按照上述步骤手动配置 FFmpeg。" -ForegroundColor Red
    exit 1
}

