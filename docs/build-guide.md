# 构建指南

本文档详细说明如何配置构建环境并成功构建项目。

## 目录

1. [WiX 工具配置](#wix-工具配置)
2. [构建流程分析](#构建流程分析)
3. [FFmpeg 打包说明](#ffmpeg-打包说明)
4. [常见问题](#常见问题)

---

## WiX 工具配置

### 问题说明

Tauri 在 Windows 上构建 MSI 安装包时需要 WiX 工具集。如果网络无法下载，需要手动配置。

### 操作步骤

#### 方法一：使用 Tauri 缓存目录（推荐）

1. **创建目录结构**
   ```powershell
   # 在 PowerShell 中执行
   New-Item -ItemType Directory -Force -Path "$env:LOCALAPPDATA\tauri\WixTools314"
   ```

2. **解压并放置文件**
   - 解压你下载的 `wix314-binaries.zip`
   - 将解压后的**所有文件**复制到以下目录：
     ```
     C:\Users\你的用户名\AppData\Local\tauri\WixTools314\
     ```
   - 确保目录结构如下：
     ```
     C:\Users\你的用户名\AppData\Local\tauri\WixTools314\
     ├── candle.exe
     ├── dark.exe
     ├── light.exe
     ├── lit.exe
     ├── ... (其他 WiX 工具文件)
     ```

3. **验证配置**
   重新运行构建命令，Tauri 会自动使用本地 WiX 工具。

#### 方法二：设置环境变量

如果不想使用默认位置，可以设置环境变量：

```powershell
# 设置 WiX 工具路径
$env:TAURI_WIX_PATH = "C:\path\to\wix314-binaries"
```

或在系统环境变量中设置：
- 变量名：`TAURI_WIX_PATH`
- 变量值：WiX 工具解压后的完整路径

### 快速操作脚本

创建 `scripts/setup-wix.ps1`（可选）：

```powershell
# 设置 WiX 工具路径
$wixPath = "$env:LOCALAPPDATA\tauri\WixTools314"
$zipPath = "wix314-binaries.zip"

if (Test-Path $zipPath) {
    Write-Host "正在解压 WiX 工具..."
    Expand-Archive -Path $zipPath -DestinationPath $wixPath -Force
    Write-Host "WiX 工具已配置完成！"
    Write-Host "路径: $wixPath"
} else {
    Write-Host "错误: 找不到 $zipPath"
    Write-Host "请将 wix314-binaries.zip 放在项目根目录"
}
```

---

## 构建流程分析

### 完整构建流程

```
npm run tauri build
├── 1. 检查依赖
│   └── 验证 Tauri CLI、Rust、Node.js 版本
│
├── 2. 运行前端构建
│   └── npm run build
│       ├── Vite 编译前端代码
│       └── 输出到 dist/ 目录
│
├── 3. 编译 Rust 后端
│   └── cargo build --release
│       ├── 编译 Rust 代码
│       ├── 处理 externalBin（FFmpeg）
│       └── 生成可执行文件
│
└── 4. 打包应用
    ├── Windows (MSI)
    │   ├── 检查 WiX 工具
    │   ├── 创建安装包资源
    │   └── 生成 .msi 文件
    ├── macOS (DMG/App)
    └── Linux (AppImage/DEB)
```

### 构建产物位置

- **可执行文件**：`src-tauri/target/release/frame-extractor-rs.exe`
- **安装包**：`src-tauri/target/release/bundle/msi/`
- **前端资源**：`dist/`（已打包到应用中）

### 构建时间估算

- **首次构建**：5-10 分钟（下载依赖）
- **增量构建**：2-3 分钟
- **仅前端更新**：< 1 分钟

---

## FFmpeg 打包说明

### ✅ FFmpeg 会自动打包

**重要**：FFmpeg 会被自动打包到应用中，用户无需单独下载！

### 配置说明

在 `src-tauri/tauri.conf.json` 中已配置：

```json
{
  "bundle": {
    "externalBin": [
      "bin/ffmpeg.exe"
    ]
  }
}
```

### 打包机制

1. **构建时**
   - Tauri 会自动将 `src-tauri/bin/ffmpeg.exe` 复制到应用资源目录
   - 同时复制 `ffmpeg.exe-x86_64-pc-windows-msvc.exe`（如果存在）

2. **运行时查找路径**
   应用会按以下顺序查找 FFmpeg：
   ```
   1. 应用目录/ffmpeg.exe
   2. 应用目录/bin/ffmpeg.exe
   3. 应用目录/resources/bin/ffmpeg.exe
   4. 系统 PATH 中的 ffmpeg
   ```

3. **验证打包**
   构建完成后，检查：
   ```powershell
   # 检查安装包内容（构建后）
   Get-ChildItem "src-tauri\target\release\bundle\msi\*.msi" | 
       ForEach-Object { Write-Host $_.FullName }
   ```

### 开发环境 vs 生产环境

| 环境 | FFmpeg 位置 | 说明 |
|------|------------|------|
| **开发环境** | `src-tauri/bin/ffmpeg.exe` | 用于开发测试 |
| **生产环境** | 打包在应用内 | 自动包含，用户无需安装 |

### 确保 FFmpeg 正确打包

1. **检查文件存在**
   ```powershell
   # 在项目根目录执行
   Test-Path "src-tauri\bin\ffmpeg.exe"
   Test-Path "src-tauri\bin\ffmpeg.exe-x86_64-pc-windows-msvc.exe"
   ```

2. **构建时验证**
   构建日志中应包含：
   ```
   Copying external binary: bin/ffmpeg.exe
   ```

3. **测试打包的应用**
   - 安装生成的 MSI
   - 运行应用
   - 尝试处理视频，确认 FFmpeg 可用

---

## 常见问题

### Q1: WiX 工具下载失败

**错误信息**：
```
Downloading https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip
failed to bundle project `timeout: global`
```

**解决方案**：
1. 手动下载 `wix314-binaries.zip`
2. 按照 [WiX 工具配置](#wix-工具配置) 章节操作

### Q2: FFmpeg 找不到

**错误信息**：
```
无法执行 FFmpeg: ...
FFmpeg 未找到或不可用
```

**解决方案**：
1. 确保 `src-tauri/bin/ffmpeg.exe` 存在
2. 确保 `src-tauri/bin/ffmpeg.exe-x86_64-pc-windows-msvc.exe` 存在
3. 重新构建项目

### Q3: 构建速度慢

**优化建议**：
1. 使用 `--debug` 标志进行开发构建（不优化）
2. 使用增量编译（Cargo 自动处理）
3. 仅在需要时构建安装包：
   ```powershell
   # 只构建可执行文件（不打包）
   cd src-tauri
   cargo build --release
   ```

### Q4: 安装包太大

**原因**：
- 包含 FFmpeg（约 50-100 MB）
- Rust 运行时库
- 前端资源

**优化**：
- 使用 UPX 压缩（可能影响启动速度）
- 考虑使用 FFmpeg 静态链接版本

---

## 团队协作说明

### 对于开发人员

1. **首次克隆项目后**：
   ```powershell
   # 安装依赖
   npm install
   
   # 下载 FFmpeg（如果还没有）
   # 从 https://github.com/BtbN/FFmpeg-Builds/releases 下载
   # 解压后复制到 src-tauri/bin/ffmpeg.exe
   
   # 配置 WiX（如果需要构建安装包）
   # 按照上面的 WiX 配置步骤
   ```

2. **开发时**：
   ```powershell
   npm run tauri dev
   ```

3. **构建发布版本**：
   ```powershell
   npm run tauri build
   ```

### 对于最终用户

✅ **无需安装 FFmpeg** - 已包含在应用内
✅ **无需安装 WiX** - 仅构建时需要
✅ **只需安装 MSI** - 一键安装所有依赖

---

## 相关文件

- `src-tauri/tauri.conf.json` - Tauri 配置
- `src-tauri/bin/ffmpeg.exe` - FFmpeg 可执行文件
- `src-tauri/bin/ffmpeg.exe-x86_64-pc-windows-msvc.exe` - FFmpeg（目标三元组命名）
- `src-tauri/src/extractor.rs` - FFmpeg 路径查找逻辑

---

## 详细构建流程

更详细的构建流程分析请参考：[构建流程分析](./build-process-analysis.md)

## 参考资源

- [Tauri 官方文档](https://tauri.app/)
- [WiX 工具集](https://wixtoolset.org/)
- [FFmpeg 下载](https://github.com/BtbN/FFmpeg-Builds/releases)
- [NSIS 文档](https://nsis.sourceforge.io/)

