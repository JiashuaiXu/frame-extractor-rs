# 项目架构设计

## 整体架构

```
┌─────────────────────────────────────────────────┐
│              用户界面层 (UI Layer)                │
│  ┌───────────────────────────────────────────┐  │
│  │  HTML + CSS + JavaScript (原生)          │  │
│  │  - index.html                            │  │
│  │  - style.css                             │  │
│  │  - main.js                               │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
                      │
                      │ IPC (Inter-Process Communication)
                      ▼
┌─────────────────────────────────────────────────┐
│          Tauri 框架层 (Framework Layer)          │
│  ┌───────────────────────────────────────────┐ │
│  │  WebView (前端渲染)                        │ │
│  │  IPC Bridge (通信桥接)                     │ │
│  │  Plugin System (插件系统)                  │ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
                      │
                      │ API Calls
                      ▼
┌─────────────────────────────────────────────────┐
│         Rust 后端层 (Backend Layer)              │
│  ┌───────────────────────────────────────────┐ │
│  │  main.rs - 应用入口                        │ │
│  │  extractor.rs - 核心业务逻辑                │ │
│  │  Command Handlers (命令处理)                │ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
                      │
                      │ System Calls
                      ▼
┌─────────────────────────────────────────────────┐
│        系统集成层 (System Integration)           │
│  ┌──────────────┬──────────────┬──────────────┐ │
│  │  FFmpeg      │  File System │  Dialog      │ │
│  │  (视频处理)   │  (文件操作)   │  (文件选择)  │ │
│  └──────────────┴──────────────┴──────────────┘ │
└─────────────────────────────────────────────────┘
```

---

## 架构层次详解

### 1. 用户界面层 (Frontend)

**技术栈**: HTML + CSS + JavaScript (原生)

**职责**:
- 用户界面渲染
- 用户交互处理
- 数据展示
- 调用后端 API

**文件结构**:
```
frontend/
├── index.html      # 页面结构
├── style.css       # 样式定义
└── main.js         # 业务逻辑
```

**特点**:
- 无框架依赖 (纯原生 JS)
- 轻量级
- 通过 Tauri API 与后端通信

---

### 2. Tauri 框架层

**技术栈**: Tauri 2.0

**职责**:
- 前端渲染 (WebView)
- IPC 通信桥接
- 插件管理
- 窗口管理
- 安全策略

**核心组件**:
- **WebView**: 渲染前端界面
- **IPC Bridge**: 前后端通信
- **Plugin System**: 扩展功能
- **Window Manager**: 窗口管理

---

### 3. Rust 后端层

**技术栈**: Rust + Tauri

**职责**:
- 业务逻辑处理
- 系统调用
- 资源管理
- 性能关键操作

**文件结构**:
```
src-tauri/src/
├── main.rs        # 应用入口
└── extractor.rs   # 视频抽帧逻辑
```

**特点**:
- 类型安全
- 内存安全
- 高性能
- 并发支持

---

### 4. 系统集成层

**组件**:
- **FFmpeg**: 视频处理
- **File System**: 文件操作
- **Dialog**: 文件选择对话框

---

## 数据流

### 用户操作流程

```
用户点击"开始处理"
    │
    ▼
前端: main.js
    │ 调用 invoke('process_videos', {...})
    ▼
Tauri IPC Bridge
    │ 序列化参数
    ▼
后端: main.rs
    │ 接收命令
    ▼
后端: extractor.rs
    │ 执行业务逻辑
    │ - 查找视频文件
    │ - 调用 FFmpeg
    │ - 提取帧
    ▼
返回结果
    │ 序列化结果
    ▼
Tauri IPC Bridge
    │ 返回数据
    ▼
前端: main.js
    │ 更新 UI
    ▼
显示结果
```

---

## 通信机制

### IPC (Inter-Process Communication)

**前端调用后端**:
```javascript
// 前端 (main.js)
const results = await invoke('process_videos', {
    inputDir,
    outputDir,
    skipStartSec,
    frameIntervalSec,
    preserveDirStructure,
    createVideoSubdir,
});
```

**后端处理命令**:
```rust
// 后端 (main.rs)
#[tauri::command]
async fn process_videos(
    input_dir: String,
    output_dir: String,
    // ...
) -> Result<Vec<ProcessResult>, String> {
    extract_frames(...).await
}
```

**数据序列化**:
- 使用 `serde` 进行序列化
- 支持 JSON 格式
- 自动类型转换

---

## 模块划分

### 前端模块

```javascript
// main.js 模块结构
├── DOM 元素引用
├── 事件监听器
│   ├── 选择输入目录
│   ├── 选择输出目录
│   └── 开始处理
├── 业务逻辑
│   ├── 更新按钮状态
│   ├── 显示结果
│   └── 错误处理
└── 初始化
```

### 后端模块

```rust
// extractor.rs 模块结构
├── 数据结构
│   └── ProcessResult
├── 核心函数
│   ├── extract_frames()       // 主入口
│   ├── find_mp4_files()       // 查找文件
│   ├── extract_frames_from_video()  // 提取帧
│   └── get_video_info()       // 获取视频信息
├── 工具函数
│   ├── get_ffmpeg_path()      // 获取 FFmpeg 路径
│   ├── check_ffmpeg()         // 检查 FFmpeg
│   ├── parse_duration()       // 解析时长
│   └── parse_fps()            // 解析 FPS
└── 辅助函数
    └── get_output_dir_for_video()  // 获取输出目录
```

---

## 安全机制

### 1. CSP (Content Security Policy)

```json
{
  "app": {
    "security": {
      "csp": null  // 开发环境禁用，生产环境应配置
    }
  }
}
```

### 2. 权限控制

```json
{
  "plugins": {
    "fs": {
      "scope": ["**"]  // 文件系统访问范围
    }
  }
}
```

### 3. IPC 命令白名单

- 所有命令必须在 Rust 中显式声明
- 前端无法直接调用系统 API
- 通过 Tauri 命令进行安全封装

---

## 性能优化

### 1. 异步处理

```rust
// 使用 async/await
pub async fn extract_frames(...) -> Result<Vec<ProcessResult>> {
    // 异步文件操作
    let files = find_mp4_files(input_path).await?;
    // ...
}
```

### 2. 并发处理

```rust
// 可以并行处理多个视频
// 使用 tokio 运行时
```

### 3. 资源管理

- 及时释放文件句柄
- 使用引用避免数据复制
- 合理使用缓存

---

## 扩展性设计

### 1. 插件系统

```rust
// 添加新插件
.plugin(tauri_plugin_new::init())
```

### 2. 命令扩展

```rust
// 添加新命令
.invoke_handler(tauri::generate_handler![
    process_videos,
    new_command,  // 新命令
])
```

### 3. 前端扩展

```javascript
// 添加新功能模块
// 通过 Tauri API 调用后端
```

---

## 部署架构

### 开发环境

```
开发服务器 (Vite)
    │
    ├── 前端: http://localhost:1420
    │
    └── 后端: cargo run (调试模式)
```

### 生产环境

```
打包后的应用
    │
    ├── 前端: 内嵌到应用 (dist/)
    │
    └── 后端: 编译为二进制 (release)
```

---

## 相关文件

- `index.html` - 前端入口
- `main.js` - 前端逻辑
- `src-tauri/src/main.rs` - 后端入口
- `src-tauri/src/extractor.rs` - 核心逻辑
- `src-tauri/tauri.conf.json` - 配置

---

## 架构优势

1. **类型安全**: Rust 类型系统保证
2. **性能优异**: 编译为原生代码
3. **内存安全**: 无内存泄漏风险
4. **跨平台**: 支持 Windows/macOS/Linux
5. **安全性**: IPC 通信隔离
6. **可维护性**: 清晰的模块划分

