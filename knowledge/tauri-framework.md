# Tauri 框架详解

## 概述

**Tauri** 是一个用于构建桌面应用的框架，使用 Web 前端和 Rust 后端。

### 版本

本项目使用 **Tauri 2.0**。

---

## 核心概念

### 1. 架构

```
┌─────────────┐
│  Web Frontend│  (HTML/CSS/JS)
└──────┬───────┘
       │ IPC
       ▼
┌─────────────┐
│ Rust Backend│  (Rust)
└──────┬───────┘
       │ System APIs
       ▼
┌─────────────┐
│   OS APIs   │
└─────────────┘
```

### 2. 组件

- **WebView**: 渲染前端界面
- **IPC Bridge**: 前后端通信
- **Plugin System**: 扩展功能
- **Window Manager**: 窗口管理

---

## 配置系统

### tauri.conf.json

```json
{
  "productName": "FrameExtractor",
  "version": "1.0.0",
  "identifier": "com.frame-extractor.app",
  
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  
  "app": {
    "windows": [{
      "title": "视频抽帧工具",
      "width": 900,
      "height": 700
    }]
  },
  
  "bundle": {
    "active": true,
    "targets": "all",
    "externalBin": ["bin/ffmpeg.exe"]
  }
}
```

---

## IPC 通信

### 前端调用后端

```javascript
// 前端
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('process_videos', {
    inputDir: '...',
    outputDir: '...',
});
```

### 后端定义命令

```rust
// 后端
#[tauri::command]
async fn process_videos(
    input_dir: String,
    output_dir: String,
) -> Result<Vec<ProcessResult>, String> {
    // 处理逻辑
    Ok(results)
}
```

### 注册命令

```rust
.invoke_handler(tauri::generate_handler![
    process_videos
])
```

---

## 插件系统

### 使用插件

```rust
// 初始化插件
.plugin(tauri_plugin_dialog::init())
.plugin(tauri_plugin_fs::init())
```

### 前端使用

```javascript
// 使用对话框插件
import { open } from '@tauri-apps/plugin-dialog';

const selected = await open({
    directory: true,
});
```

---

## 窗口管理

### 窗口配置

```json
{
  "app": {
    "windows": [{
      "title": "视频抽帧工具",
      "width": 900,
      "height": 700,
      "resizable": true,
      "fullscreen": false
    }]
  }
}
```

### 窗口操作

```rust
// 创建新窗口
use tauri::Window;

let window = Window::new("label", WindowOptions::default())?;
```

---

## 资源管理

### 外部二进制文件

```json
{
  "bundle": {
    "externalBin": ["bin/ffmpeg.exe"]
  }
}
```

**说明**:
- 文件会被复制到应用目录
- 运行时可以从应用目录访问
- 需要按目标三元组命名

---

## 安全机制

### CSP (Content Security Policy)

```json
{
  "app": {
    "security": {
      "csp": null  // 开发环境
    }
  }
}
```

---

## 权限系统 (Capabilities) - Tauri 2.0

### 概述

**Tauri 2.0 引入了新的权限管理系统**，使用 `capabilities` 文件来配置插件权限。

### 为什么需要 Capabilities？

在 Tauri 2.0 中：
- ✅ 插件在 Rust 代码中初始化（`.plugin()`）
- ❌ **但还需要在 capabilities 文件中配置权限**
- 没有权限配置，插件功能会被阻止

### 创建 Capabilities 文件

#### 1. 创建目录

```powershell
New-Item -ItemType Directory -Force -Path "src-tauri\capabilities"
```

#### 2. 创建 default.json

创建文件 `src-tauri/capabilities/default.json`:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for the application",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "dialog:default",
    "dialog:open",
    "dialog:save",
    "fs:default",
    "fs:read-file",
    "fs:write-file",
    "fs:read-dir",
    "fs:create-dir",
    "fs:remove-dir",
    "fs:remove-file",
    "fs:copy-file",
    "fs:rename-file",
    "fs:exists",
    {
      "identifier": "fs:scope-all",
      "allow": [
        {
          "path": "**"
        }
      ]
    }
  ]
}
```

### 权限说明

#### Dialog 插件权限

| 权限 | 说明 |
|------|------|
| `dialog:default` | 默认权限集 |
| `dialog:allow-open` | 打开文件/目录对话框 ⚠️ 注意 `allow-` 前缀 |
| `dialog:allow-save` | 保存文件对话框 ⚠️ 注意 `allow-` 前缀 |
| `dialog:allow-ask` | 询问对话框 |
| `dialog:allow-confirm` | 确认对话框 |
| `dialog:allow-message` | 消息对话框 |

#### FS 插件权限

| 权限 | 说明 |
|------|------|
| `fs:default` | 默认权限集 |
| `fs:allow-read-file` | 读取文件 ⚠️ 注意 `allow-` 前缀 |
| `fs:allow-write-file` | 写入文件 ⚠️ 注意 `allow-` 前缀 |
| `fs:allow-read-dir` | 读取目录 ⚠️ 注意 `allow-` 前缀 |
| `fs:allow-mkdir` | 创建目录 ⚠️ 注意 `allow-` 前缀 |
| `fs:allow-remove` | 删除文件/目录 ⚠️ 注意 `allow-` 前缀 |
| `fs:allow-rename` | 重命名文件 ⚠️ 注意 `allow-` 前缀 |
| `fs:allow-exists` | 检查文件存在 ⚠️ 注意 `allow-` 前缀 |
| `fs:allow-copy-file` | 复制文件 ⚠️ 注意 `allow-` 前缀 |
| `fs:scope` | 路径作用域配置（需要对象格式） |

### 常见问题

#### Q: 文件浏览对话框无法打开？

**原因**: 缺少 `dialog:open` 权限

**解决方案**: 在 `capabilities/default.json` 中添加：
```json
{
  "permissions": [
    "dialog:default",
    "dialog:allow-open"  // ⚠️ 注意使用 allow- 前缀
  ]
}
```

#### Q: 文件操作失败？

**原因**: 缺少 FS 插件权限或路径范围限制

**解决方案**: 添加 FS 权限和路径范围：
```json
{
  "permissions": [
    "fs:default",
    "fs:allow-read-file",  // ⚠️ 注意使用 allow- 前缀
    "fs:allow-write-file", // ⚠️ 注意使用 allow- 前缀
    {
      "identifier": "fs:scope",  // ⚠️ 注意标识符是 fs:scope
      "allow": [{"path": "**"}]
    }
  ]
}
```

### 权限配置最佳实践

#### 1. 最小权限原则

只授予必要的权限：

```json
{
  "permissions": [
    "dialog:open",  // 只需要打开对话框
    "fs:read-file", // 只需要读取文件
    {
      "identifier": "fs:scope-limited",
      "allow": [
        {"path": "$HOME/Documents/**"},  // 限制访问范围
        {"path": "$HOME/Downloads/**"}
      ]
    }
  ]
}
```

#### 2. 窗口隔离

不同窗口可以有不同的权限：

```json
// capabilities/admin.json - 管理员窗口
{
  "identifier": "admin",
  "windows": ["admin-window"],
  "permissions": [
    "fs:default",
    "fs:scope-all"
  ]
}

// capabilities/user.json - 普通用户窗口
{
  "identifier": "user",
  "windows": ["main"],
  "permissions": [
    "fs:read-file",
    "fs:scope-limited"
  ]
}
```

### Tauri 1.x vs 2.0 权限对比

| 特性 | Tauri 1.x | Tauri 2.0 |
|------|-----------|-----------|
| **配置位置** | `tauri.conf.json` | `capabilities/*.json` |
| **配置方式** | `plugins` 对象 | `permissions` 数组 |
| **权限粒度** | 插件级别 | 命令级别 |
| **窗口隔离** | 不支持 | 支持 |
| **路径限制** | 简单 | 细粒度 |

### 实际案例：文件浏览功能

#### 问题

点击"浏览"按钮时，文件选择对话框无法打开。

#### 原因分析

1. ✅ 插件已初始化：`.plugin(tauri_plugin_dialog::init())`
2. ✅ 前端代码正确：`import { open } from '@tauri-apps/plugin-dialog'`
3. ❌ **缺少权限配置**：没有 `capabilities/default.json` 文件

#### 解决方案

创建 `src-tauri/capabilities/default.json` 并添加 `dialog:open` 权限。

#### 完整配置示例

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
    "fs:allow-read-dir",     // ⚠️ 注意 allow- 前缀
    "fs:allow-mkdir",        // ⚠️ 注意 allow- 前缀
    "fs:allow-remove",       // ⚠️ 注意 allow- 前缀
    "fs:allow-rename",       // ⚠️ 注意 allow- 前缀
    "fs:allow-exists",       // ⚠️ 注意 allow- 前缀
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

### ⚠️ 重要：权限标识符格式

在 Tauri 2.0 中，权限标识符必须遵循以下格式：

- **允许权限**: `plugin:allow-command` (如 `dialog:allow-open`, `fs:allow-read-file`)
- **拒绝权限**: `plugin:deny-command` (如 `dialog:deny-open`)
- **默认权限集**: `plugin:default` (如 `dialog:default`, `fs:default`)
- **作用域权限**: `plugin:scope` (如 `fs:scope`，需要对象格式)

**常见错误**:
- ❌ `dialog:open` → ✅ `dialog:allow-open`
- ❌ `fs:read-file` → ✅ `fs:allow-read-file`
- ❌ `fs:scope-all` → ✅ `fs:scope` (对象格式)

---

### 旧版权限控制（已废弃）

> ⚠️ **注意**: 以下配置方式在 Tauri 2.0 中已不再使用

```json
{
  "plugins": {
    "fs": {
      "scope": ["**"]  // ❌ Tauri 2.0 不再支持
    }
  }
}
```

在 Tauri 2.0 中，应使用 capabilities 文件配置权限。

---

## 构建系统

### 开发模式

```bash
npm run tauri dev
```

**流程**:
1. 启动前端开发服务器 (Vite)
2. 编译 Rust 后端 (debug)
3. 启动应用，连接前端

### 生产构建

```bash
npm run tauri build
```

**流程**:
1. 构建前端 (Vite)
2. 编译 Rust 后端 (release)
3. 打包安装程序

---

## 特性标志

### custom-protocol

```toml
[features]
custom-protocol = ["tauri/custom-protocol"]
```

**用途**: 生产环境使用自定义协议加载前端资源。

---

## 生命周期

### 应用启动

```rust
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化逻辑
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 事件处理

```rust
// 监听事件
app.listen("event-name", |event| {
    println!("收到事件: {:?}", event);
});

// 发送事件
app.emit("event-name", "data")?;
```

---

## 与 Electron 对比

| 特性 | Tauri | Electron |
|------|-------|----------|
| **后端语言** | Rust | Node.js |
| **二进制大小** | 小 (~5-10 MB) | 大 (~100+ MB) |
| **性能** | 高 | 中等 |
| **内存占用** | 低 | 高 |
| **安全性** | 高 | 中等 |
| **学习曲线** | 中等 | 低 |

---

## 最佳实践

### 1. 错误处理

```rust
#[tauri::command]
async fn my_command() -> Result<String, String> {
    // 返回明确的错误
    Err("错误信息".to_string())
}
```

### 2. 异步操作

```rust
#[tauri::command]
async fn my_command() -> Result<String, String> {
    // 使用 async/await
    let result = async_operation().await?;
    Ok(result)
}
```

### 3. 数据序列化

```rust
// 使用 serde
#[derive(Serialize)]
struct MyData {
    field: String,
}
```

---

## 相关文件

- `src-tauri/tauri.conf.json` - 配置文件
- `src-tauri/src/main.rs` - 应用入口
- `src-tauri/Cargo.toml` - 依赖配置
- `src-tauri/capabilities/default.json` - 权限配置文件 ⭐ Tauri 2.0

---

## 学习资源

- [Tauri 官方文档](https://tauri.app/)
- [Tauri API 文档](https://tauri.app/api/)
- [Tauri 示例](https://github.com/tauri-apps/tauri/tree/dev/examples)

