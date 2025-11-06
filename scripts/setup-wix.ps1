# WiX 工具快速配置脚本
# 用于将 wix314-binaries.zip 解压到正确位置

param(
    [string]$ZipPath = "wix314-binaries.zip"
)

$ErrorActionPreference = "Stop"

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  WiX 工具配置脚本" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# 检查 ZIP 文件是否存在
if (-not (Test-Path $ZipPath)) {
    Write-Host "错误: 找不到 $ZipPath" -ForegroundColor Red
    Write-Host ""
    Write-Host "请将 wix314-binaries.zip 放在以下位置之一：" -ForegroundColor Yellow
    Write-Host "  1. 项目根目录: $(Get-Location)\wix314-binaries.zip" -ForegroundColor Yellow
    Write-Host "  2. 或者使用参数指定路径: .\scripts\setup-wix.ps1 -ZipPath 'C:\path\to\wix314-binaries.zip'" -ForegroundColor Yellow
    Write-Host ""
    exit 1
}

# 目标目录
$targetDir = "$env:LOCALAPPDATA\tauri\WixTools314"

Write-Host "ZIP 文件路径: $ZipPath" -ForegroundColor Green
Write-Host "目标目录: $targetDir" -ForegroundColor Green
Write-Host ""

# 创建目标目录
Write-Host "正在创建目标目录..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $targetDir | Out-Null

# 检查是否已存在
if ((Get-ChildItem $targetDir -ErrorAction SilentlyContinue | Measure-Object).Count -gt 0) {
    Write-Host "警告: 目标目录已存在文件" -ForegroundColor Yellow
    $response = Read-Host "是否覆盖现有文件? (Y/N)"
    if ($response -ne "Y" -and $response -ne "y") {
        Write-Host "操作已取消" -ForegroundColor Red
        exit 0
    }
    Write-Host "正在清理现有文件..." -ForegroundColor Yellow
    Remove-Item "$targetDir\*" -Recurse -Force
}

# 解压文件
Write-Host "正在解压 WiX 工具..." -ForegroundColor Yellow
try {
    Expand-Archive -Path $ZipPath -DestinationPath $targetDir -Force
    Write-Host "✓ 解压完成" -ForegroundColor Green
} catch {
    Write-Host "错误: 解压失败 - $_" -ForegroundColor Red
    exit 1
}

# 验证关键文件
Write-Host ""
Write-Host "正在验证安装..." -ForegroundColor Yellow
$requiredFiles = @("candle.exe", "light.exe", "dark.exe")
$allExists = $true

foreach ($file in $requiredFiles) {
    $filePath = Join-Path $targetDir $file
    if (Test-Path $filePath) {
        Write-Host "  ✓ $file" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $file (未找到)" -ForegroundColor Red
        $allExists = $false
    }
}

if ($allExists) {
    Write-Host ""
    Write-Host "=====================================" -ForegroundColor Cyan
    Write-Host "  WiX 工具配置成功！" -ForegroundColor Green
    Write-Host "=====================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "现在可以运行构建命令：" -ForegroundColor Yellow
    Write-Host "  npm run tauri build" -ForegroundColor Cyan
    Write-Host ""
} else {
    Write-Host ""
    Write-Host "警告: 部分文件缺失，请检查 ZIP 文件是否完整" -ForegroundColor Yellow
}

