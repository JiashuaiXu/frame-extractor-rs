# 故障排除指南

## 常见问题及解决方案

### 文件浏览对话框无法打开

#### 症状

点击"浏览"按钮时，文件选择对话框无法打开，没有任何反应。

#### 原因

在 **Tauri 2.0** 中，插件权限需要通过 `capabilities` 文件配置。即使：
- ✅ 插件已在 Rust 代码中初始化
- ✅ 前端代码正确
- ✅ 依赖已安装

如果缺少权限配置，插件功能会被阻止。

#### 解决方案

1. **创建 capabilities 目录**（如果不存在）:
   ```powershell
   New-Item -ItemType Directory -Force -Path "src-tauri\capabilities"
   ```

2. **创建 default.json 文件**:
   创建 `src-tauri/capabilities/default.json`:
   ```json
   {
     "$schema": "../gen/schemas/desktop-schema.json",
     "identifier": "default",
     "description": "Default capabilities for the application",
     "windows": ["main"],
     "permissions": [
       "core:default",
       "dialog:default",
       "dialog:allow-open",    // ⚠️ 注意 allow- 前缀
       "dialog:allow-save",    // ⚠️ 注意 allow- 前缀
       "fs:default",
       "fs:allow-read-file",   // ⚠️ 注意 allow- 前缀
       "fs:allow-write-file",  // ⚠️ 注意 allow- 前缀
       "fs:allow-read-dir",    // ⚠️ 注意 allow- 前缀
       "fs:allow-mkdir",       // ⚠️ 注意 allow- 前缀
       "fs:allow-remove",      // ⚠️ 注意 allow- 前缀
       "fs:allow-rename",      // ⚠️ 注意 allow- 前缀
       "fs:allow-exists",      // ⚠️ 注意 allow- 前缀
       {
         "identifier": "fs:scope",  // ⚠️ 注意标识符是 fs:scope
         "allow": [
           {
             "path": "**"
           }
         ]
       }
     ]
   }
   ```

   **⚠️ 重要**: 权限标识符必须使用 `allow-` 前缀格式！

3. **重新运行应用**:
   ```bash
   npm run tauri dev
   ```

#### 验证

点击"浏览"按钮，应该能正常打开文件选择对话框。

---

### 插件初始化失败

#### 症状

应用启动时出现错误：
```
PluginInitialization("dialog", "Error deserializing 'plugins.dialog'...")
```

#### 原因

Tauri 2.0 不再在 `tauri.conf.json` 中配置插件。

#### 解决方案

1. **移除 tauri.conf.json 中的 plugins 配置**
2. **确保插件在 Rust 代码中初始化**:
   ```rust
   .plugin(tauri_plugin_dialog::init())
   ```
3. **在 capabilities 文件中配置权限**（见上文）

---

### FFmpeg 找不到

#### 症状

处理视频时提示 "FFmpeg 未找到或不可用"。

#### 原因

1. FFmpeg 文件不存在
2. 路径配置错误
3. 权限不足
4. **文件名重复后缀**：Tauri 打包后文件名变成 `ffmpeg.exe.exe`

#### 解决方案

1. **检查安装目录中的文件**:
   ```powershell
   # 检查应用安装目录（通常在 AppData\Local\FrameExtractor）
   Get-ChildItem "C:\Users\$env:USERNAME\AppData\Local\FrameExtractor"
   ```
   
   如果看到 `ffmpeg.exe.exe`（多了一个 `.exe`），这是正常的，代码已支持查找该文件名。

2. **开发环境检查**:
   ```powershell
   Test-Path "src-tauri\bin\ffmpeg.exe"
   Test-Path "src-tauri\bin\ffmpeg.exe-x86_64-pc-windows-msvc.exe"
   ```

3. **下载并配置 FFmpeg**:
   - 从 https://github.com/BtbN/FFmpeg-Builds/releases 下载
   - 复制到 `src-tauri/bin/ffmpeg.exe`
   - 创建目标三元组命名版本

4. **检查权限**: 确保应用有执行权限

5. **如果问题仍然存在**:
   - 检查代码是否已更新（应支持 `ffmpeg.exe.exe`）
   - 重新构建应用：`npm run tauri build`
   - 重新安装应用

---

### 构建失败

#### 症状

运行 `npm run tauri build` 时失败。

#### 常见原因

1. **WiX 工具下载失败**
   - 解决方案: 手动配置 WiX 工具（见构建指南）

2. **产品名称包含中文字符**
   - 解决方案: 将 `productName` 改为英文

3. **依赖缺失**
   - 解决方案: 运行 `npm install` 和 `cargo build`

---

### 权限标识符格式错误

#### 症状

构建时出现错误：
```
Permission dialog:open not found, expected one of ...
```

#### 原因

Tauri 2.0 要求权限标识符使用 `allow-` 前缀格式。

#### 解决方案

修复 `capabilities/default.json` 中的权限标识符：

**错误格式**:
```json
{
  "permissions": [
    "dialog:open",      // ❌ 错误
    "fs:read-file"      // ❌ 错误
  ]
}
```

**正确格式**:
```json
{
  "permissions": [
    "dialog:allow-open",    // ✅ 正确
    "fs:allow-read-file"    // ✅ 正确
  ]
}
```

**权限标识符规则**:
- 允许权限: `plugin:allow-command`
- 拒绝权限: `plugin:deny-command`
- 默认权限集: `plugin:default`
- 作用域权限: `plugin:scope` (对象格式)

---

### Tauri API 未初始化错误

#### 症状

点击前端按钮时出现错误：
```
TypeError: can't access property "invoke", window.__TAURI_INTERNALS__ is undefined
```

#### 原因

1. **代码执行时机问题**：JavaScript 代码在 DOM 加载完成之前执行
2. **Tauri API 未初始化**：Tauri 的全局对象 `__TAURI_INTERNALS__` 在代码执行时还未准备好
3. **模块导入时机**：ES6 模块导入的代码在顶层执行，可能在 Tauri 环境完全初始化之前就尝试访问 API

#### 解决方案

将所有初始化代码包装在 `DOMContentLoaded` 事件监听器中：

```javascript
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

// 等待 DOM 加载完成
document.addEventListener('DOMContentLoaded', () => {
    initApp();
});

function initApp() {
    // 所有 DOM 操作和事件监听器注册放在这里
    const inputDirInput = document.getElementById('inputDir');
    // ... 其他初始化代码
}
```

**关键点**：
- ✅ 使用 `DOMContentLoaded` 确保 DOM 已加载
- ✅ 将所有 DOM 操作和事件监听器注册放在初始化函数中
- ✅ 保持模块导入在顶层（ES6 模块要求）

#### 验证

重新运行应用，点击按钮应该能正常工作，不再出现 `__TAURI_INTERNALS__ is undefined` 错误。

---

### IPC 通信失败

#### 症状

前端调用后端命令时失败。

#### 原因

1. 命令未注册
2. 参数类型不匹配
3. 权限不足

#### 解决方案

1. **检查命令注册**:
   ```rust
   .invoke_handler(tauri::generate_handler![
       process_videos  // 确保命令已注册
   ])
   ```

2. **检查参数类型**: 确保前后端参数类型匹配

3. **检查权限**: 确保在 capabilities 中配置了相应权限

---

## 调试技巧

### 1. 查看控制台日志

**前端**:
```javascript
console.error('错误信息:', error);
```

**后端**:
```rust
eprintln!("错误: {:?}", error);
```

### 2. 启用详细日志

**Rust**:
```rust
env_logger::init();
log::debug!("调试信息");
```

**前端**:
```javascript
// 浏览器开发者工具
console.log('调试信息');
```

### 3. 检查插件状态

在浏览器控制台检查：
```javascript
// 检查 Tauri API 是否可用
console.log(window.__TAURI__);
```

---

### 抽帧操作异常缓慢

#### 症状

- 抽帧操作耗时很长，特别是长视频或大量视频
- 应用在处理时感觉"卡死"，没有响应
- 处理一个 10 分钟的视频需要 1-2 分钟

#### 原因

**原始实现问题**：
1. **每帧单独调用 FFmpeg**：对于每个视频的每一帧，都单独启动一次 FFmpeg 进程
2. **重复打开视频文件**：每次提取帧都要重新打开视频文件并定位到时间点
3. **无进度反馈**：用户不知道处理进度

**性能影响**：
- 10 分钟视频，每 5 秒一帧 = 120 次 FFmpeg 调用
- 每次调用平均耗时 0.5-1 秒
- 总耗时 60-120 秒

#### 解决方案

✅ **已优化**：使用 FFmpeg `select` 过滤器批量提取所有帧

**优化后**：
- 单次 FFmpeg 调用提取所有帧
- 性能提升 **6-25 倍**
- 10 分钟视频处理时间从 60-120 秒降低到 5-10 秒

**技术细节**：
- 使用 `select` 过滤器：`select='gte(t,start)*lt(mod(t-start,interval),0.1)'`
- 使用 `-vsync vfr` 可变帧率输出
- 使用 `-ss` 先定位到开始时间，避免从头解码

**参考**：
- [Bug #010: 性能优化文档](../docs/bugs/010-performance-optimization.md)
- [FFmpeg 集成指南](./ffmpeg-integration.md#批量提取帧推荐---高性能)

---

### Windows 黑窗口问题

#### 症状

执行抽帧操作时，会弹出多个黑色命令行窗口（FFmpeg 进程窗口）。

#### 原因

在 Windows 上，使用 `Command::new().output()` 执行 FFmpeg 时，默认会显示控制台窗口。

#### 解决方案

✅ **已修复**：使用 `CREATE_NO_WINDOW` 标志隐藏控制台窗口

**代码实现**：
```rust
#[cfg(windows)]
use std::os::windows::process::CommandExt;

fn configure_ffmpeg_command(cmd: &mut Command) {
    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd.stdout(Stdio::piped())
       .stderr(Stdio::piped());
}
```

**说明**：
- `CREATE_NO_WINDOW` 标志告诉 Windows 不要为子进程创建控制台窗口
- 重定向 `stdout` 和 `stderr` 到管道，避免输出到控制台
- 只在 Windows 平台应用此优化

**参考**：
- [Bug #010: 性能优化文档](../docs/bugs/010-performance-optimization.md)

---

## 相关文档

- [Bug 记录](../docs/bugs/) - 详细的问题记录和解决方案
- [构建指南](../docs/build-guide.md) - 构建相关问题
- [Tauri 框架](./tauri-framework.md) - 框架使用说明
- [FFmpeg 集成](./ffmpeg-integration.md) - FFmpeg 使用和性能优化

---

## 获取帮助

如果问题仍未解决：

1. 查看 [Bug 记录](../docs/bugs/)
2. 检查 [Tauri 官方文档](https://tauri.app/)
3. 查看控制台错误信息
4. 检查日志文件

