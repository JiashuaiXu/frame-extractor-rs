# WiX 工具快速配置指南

## 快速操作（3 步）

### 步骤 1: 下载 WiX 工具
从 https://github.com/wixtoolset/wix3/releases 下载 `wix314-binaries.zip`

### 步骤 2: 放置 ZIP 文件
将 `wix314-binaries.zip` 放在项目根目录（与 `package.json` 同级）

### 步骤 3: 运行配置脚本
```powershell
.\scripts\setup-wix.ps1
```

完成！现在可以运行 `npm run tauri build`

---

## 手动配置（如果脚本不工作）

### 1. 创建目录
```powershell
New-Item -ItemType Directory -Force -Path "$env:LOCALAPPDATA\tauri\WixTools314"
```

### 2. 解压文件
将 `wix314-binaries.zip` 解压到：
```
C:\Users\你的用户名\AppData\Local\tauri\WixTools314\
```

### 3. 验证
确保目录中有以下文件：
- `candle.exe`
- `light.exe`
- `dark.exe`

---

## 验证配置

运行构建命令，如果不再出现下载超时错误，说明配置成功：

```powershell
npm run tauri build
```

---

## 常见问题

**Q: 脚本提示找不到 ZIP 文件？**  
A: 确保 `wix314-binaries.zip` 在项目根目录，或使用 `-ZipPath` 参数指定路径

**Q: 权限错误？**  
A: 以管理员身份运行 PowerShell

**Q: 仍然下载 WiX？**  
A: 检查路径是否正确：`%LOCALAPPDATA%\tauri\WixTools314\`

---

更多详细信息请查看 [完整构建指南](./build-guide.md)

