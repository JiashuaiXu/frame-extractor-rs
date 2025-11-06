# Bug 006: WiX light.exe 执行失败

## 错误信息

```
failed to bundle project `failed to run C:\Users\Administrator\AppData\Local\tauri\WixTools314\light.exe`
```

## 问题描述

WiX 工具已正确配置，`light.exe` 文件存在且可以正常运行，但在 Tauri 构建过程中执行失败。

**可能原因**：
1. **产品名称包含中文字符**：`productName` 设置为 `"视频抽帧工具"`，可能导致路径或参数编码问题
2. **路径编码问题**：MSI 安装包文件名包含中文字符：`视频抽帧工具_1.0.0_x64_en-US.msi`
3. **临时文件路径问题**：WiX 在处理包含非 ASCII 字符的路径时可能失败

## 解决方案

### 方案 1: 修改产品名称为英文（推荐）

修改 `src-tauri/tauri.conf.json`：

```json
{
  "productName": "Frame Extractor",
  // 或使用拼音
  // "productName": "FrameExtractor",
  ...
}
```

**说明**：
- 产品名称会显示在安装包文件名和安装程序中
- 可以在应用窗口标题中单独设置中文标题（在 `app.windows[0].title` 中）

### 方案 2: 仅修改安装包文件名

如果必须保留中文产品名，可以尝试：

1. 在 `tauri.conf.json` 的 `bundle` 部分添加：
```json
{
  "bundle": {
    "identifier": "com.frame-extractor.app",
    "resources": [],
    "externalBin": ["bin/ffmpeg.exe"],
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

2. 或者使用英文标识符，但保持中文显示：
```json
{
  "productName": "Frame Extractor",  // 用于安装包文件名
  "app": {
    "windows": [
      {
        "title": "视频抽帧工具"  // 应用窗口标题
      }
    ]
  }
}
```

### 方案 3: 检查环境变量和编码

确保 PowerShell 编码设置正确：

```powershell
# 设置控制台编码为 UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
chcp 65001

# 然后重新构建
npm run tauri build
```

### 方案 4: 使用临时路径（临时解决）

如果问题持续，可以尝试设置 WiX 临时目录：

```powershell
# 设置临时目录（使用英文路径）
$env:WIX_TEMP = "C:\temp\wix"
New-Item -ItemType Directory -Force -Path $env:WIX_TEMP

# 然后构建
npm run tauri build
```

## 验证步骤

1. 修改 `productName` 为英文
2. 清理构建缓存：
   ```powershell
   Remove-Item -Recurse -Force "src-tauri\target\release\bundle"
   ```
3. 重新构建：
   ```powershell
   npm run tauri build
   ```
4. 检查生成的 MSI 文件路径是否正常

## 相关文件

- `src-tauri/tauri.conf.json` - Tauri 配置文件
- WiX 工具路径：`%LOCALAPPDATA%\tauri\WixTools314\`

## 状态

✅ **已修复** - 将 `productName` 改为英文 "FrameExtractor"

---

## 附加说明

### 为什么会出现这个问题？

WiX 工具在处理包含非 ASCII 字符（如中文、日文等）的文件路径时，可能会遇到编码问题。特别是在 Windows 上，路径编码和命令行参数编码需要保持一致。

### 最佳实践

1. **产品标识符使用英文**：`productName`、`identifier` 等使用英文
2. **显示名称可以使用中文**：在应用窗口标题、菜单等 UI 元素中使用中文
3. **安装包文件名自动生成**：Tauri 会根据 `productName` 生成安装包文件名

### 示例配置

```json
{
  "productName": "FrameExtractor",  // 英文，用于安装包
  "version": "1.0.0",
  "identifier": "com.frame-extractor.app",  // 英文
  "app": {
    "windows": [
      {
        "title": "视频抽帧工具"  // 中文，应用窗口标题
      }
    ]
  }
}
```

这样既保证了构建过程的兼容性，又能在应用中显示中文界面。

