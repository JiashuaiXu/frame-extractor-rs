# Bug 记录索引

本目录包含项目开发过程中遇到的所有 bug 及其解决方案。

## Bug 列表

### 001-005: 编译和配置问题
**文件**: [001-005-build-issues.md](./001-005-build-issues.md)

包含以下 bug：
- **001**: Tauri 2.0 配置字段过时（devPath/distDir）
- **002**: 外部二进制文件命名问题
- **003**: Rust 代码编译错误（strip_prefix 类型错误）
- **004**: 前端依赖缺失（@tauri-apps/plugin-dialog）
- **005**: Tauri 2.0 插件配置格式问题

### 006: WiX light.exe 执行失败
**文件**: [006-wix-light-execution-failed.md](./006-wix-light-execution-failed.md)

**问题**: WiX 工具执行失败，通常由产品名称包含中文字符导致路径编码问题
**状态**: ✅ 已修复

### 007: 文件浏览对话框无法使用
**文件**: [007-dialog-plugin-not-working.md](./007-dialog-plugin-not-working.md)

**问题**: 点击浏览按钮时文件选择对话框无法打开
- 缺少 Tauri 2.0 capabilities 权限配置
- 权限标识符格式错误（需要使用 `allow-` 前缀）

**状态**: ✅ 已修复

### 008: Tauri API 未初始化错误
**文件**: [008-tauri-internals-undefined.md](./008-tauri-internals-undefined.md)

**问题**: 点击前端选择路径按钮后出现 `window.__TAURI_INTERNALS__ is undefined` 错误
- 代码执行时机问题：在 DOM 加载完成之前执行
- Tauri API 未初始化：全局对象还未准备好

**状态**: ✅ 已修复

### 009: FFmpeg 文件名重复 .exe 后缀问题
**文件**: [009-ffmpeg-exe-exe-naming-issue.md](./009-ffmpeg-exe-exe-naming-issue.md)

**问题**: 应用安装后 FFmpeg 文件名变成 `ffmpeg.exe.exe`，代码查找 `ffmpeg.exe` 失败
- Tauri 打包时自动添加 `.exe` 后缀导致文件名重复
- 路径查找逻辑未考虑重复后缀的情况

**状态**: ✅ 已修复

## 添加新 Bug

当遇到新 bug 时，请按照以下格式记录：

1. 创建新的 bug 记录文件，命名格式：`{编号}-{简短描述}.md`
2. 在本文档中添加索引
3. 包含以下信息：
   - 错误信息
   - 问题描述
   - 解决方案
   - 状态（✅ 已修复 / 🔄 进行中 / ❌ 未修复）

## 格式模板

```markdown
# Bug {编号}: {标题}

**错误信息**：
```
{错误信息}
```

**问题描述**：
{详细描述问题}

**解决方案**：
{解决方案和代码修改}

**状态**：✅ 已修复 / 🔄 进行中 / ❌ 未修复
```

