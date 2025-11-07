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

### 010: 抽帧性能优化 - 黑窗口和卡顿问题
**文件**: [010-performance-optimization.md](./010-performance-optimization.md)

**问题**: 
1. **Windows 黑窗口**：执行抽帧时弹出多个黑色命令行窗口
2. **性能卡顿**：抽帧操作异常缓慢，特别是长视频或大量视频
   - 每帧单独调用 FFmpeg（N 次调用）
   - 串行处理，无并发
   - 重复打开视频文件
   - 无进度反馈

**解决方案**:
- 使用 `CREATE_NO_WINDOW` 标志隐藏 Windows 控制台窗口
- 使用 FFmpeg `select` 过滤器批量提取所有帧（从 N 次调用减少到 1 次）
- 性能提升 6-25 倍

**状态**: ✅ 已修复

### 011: Vite 导入解析错误
**文件**: [011-vite-import-parsing-error.md](./011-vite-import-parsing-error.md)

**问题**: Vite 无法解析 `main.js` 文件，报错 `Unexpected end of file`
- **根本原因**：`main.js` 文件缺少 `initApp()` 函数的闭合大括号 `}`
- 错误信息：`main.js:248:0: Unexpected end of file`

**解决方案**:
- 在 `main.js` 文件末尾添加缺失的闭合大括号来闭合 `initApp()` 函数

**状态**: ✅ 已修复

### 012: 网络共享（Samba）文件处理超时
**文件**: [012-network-share-timeout.md](./012-network-share-timeout.md)

**问题**: 处理网络共享上的视频文件时，在"正在获取视频信息..."步骤卡住
- **原因**：FFmpeg 需要读取网络文件元数据，没有超时机制导致无限等待
- **影响**：网络文件无法处理，应用无响应

**解决方案**:
- 添加超时机制（网络文件 30 秒，本地文件 15 秒）
- 优化 FFmpeg 参数（`-analyzeduration`、`-probesize`、`-readrate`）限制探测
- 使用异步处理支持超时
- 添加网络路径检测和特殊处理
- 添加降级方案：超时后使用默认值继续处理

**状态**: ✅ 已修复（基础方案）

### 013: 网络共享文件获取视频信息超时深度分析
**文件**: [013-network-share-timeout-deep-analysis.md](./013-network-share-timeout-deep-analysis.md)

**问题**: 即使添加了超时机制，网络文件仍然超时（30-60秒）
- **深度分析**：FFmpeg 获取视频信息的工作原理、网络文件访问的特殊性
- **根本原因**：网络带宽限制、文件格式（moov atom位置）、参数设置不当

**优化方案**:
- 进一步优化 FFmpeg 参数（探测大小 10MB → 2MB，超时 60秒 → 30秒）
- 添加读取速度限制（`-readrate`）
- 实现降级方案：超时后使用默认值继续处理
- 提供最佳实践建议

**状态**: ✅ 已优化

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

