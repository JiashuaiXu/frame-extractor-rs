# 构建系统详解

## 概述

本项目采用 **前后端分离的构建架构**：
- **前端**: HTML/CSS/JavaScript + Vite
- **后端**: Rust + Cargo
- **集成**: Tauri 2.0 框架

---

## 前端构建流程

### 构建工具链

```
Vite 5.4.21
├── 开发服务器 (端口 1420)
├── 生产构建
└── 热模块替换 (HMR)
```

### 前端构建步骤

#### 1. 开发环境 (`npm run dev`)

```
npm run dev
│
└── vite serve
    ├── 启动开发服务器 (localhost:1420)
    ├── 监听文件变化
    ├── 热模块替换 (HMR)
    └── 实时编译 ES6+ → ES5
```

#### 2. 生产构建 (`npm run build`)

```
npm run build
│
└── vite build
    ├── 入口: index.html
    ├── 模块转换 (7 个模块)
    │   ├── main.js → bundle
    │   ├── style.css → bundle
    │   └── index.html → dist/
    ├── 代码压缩 (esbuild)
    ├── 资源优化
    └── 输出到 dist/
        ├── index.html (3.53 kB)
        ├── assets/index-B0FZ67ao.css (3.97 kB)
        └── assets/index-BxpddpXX.js (3.80 kB)
```

### Vite 配置

```javascript
// vite.config.js
export default defineConfig({
    server: {
        port: 1420,           // Tauri 开发端口
        strictPort: true,     // 端口被占用时失败
    },
    envPrefix: ['VITE_', 'TAURI_'],  // 环境变量前缀
    build: {
        target: ['es2021', 'chrome100', 'safari13'],  // 目标环境
        minify: 'esbuild',    // 生产环境压缩
        sourcemap: false,     // 生产环境不生成 sourcemap
    },
});
```

### 前端依赖

#### 运行时依赖
- `@tauri-apps/api` - Tauri API 核心
- `@tauri-apps/plugin-dialog` - 文件对话框插件

#### 开发依赖
- `vite` - 构建工具
- `@tauri-apps/cli` - Tauri CLI 工具

---

## 后端构建流程

### 构建工具链

```
Cargo (Rust 包管理器)
├── rustc (Rust 编译器)
├── 依赖管理
└── 构建脚本 (build.rs)
```

### 后端构建步骤

#### 1. 开发构建 (`cargo build`)

```
cargo build
│
├── 编译 build-dependencies (tauri-build)
│   └── 执行 build.rs
│       └── tauri_build::build()
│           ├── 生成代码
│           ├── 处理配置
│           └── 准备资源
│
├── 编译依赖库
│   ├── tauri (框架核心)
│   ├── tauri-plugin-* (插件)
│   ├── serde (序列化)
│   ├── tokio (异步运行时)
│   └── ... (其他依赖)
│
└── 编译项目代码
    ├── main.rs
    └── extractor.rs
    └── 生成: target/debug/frame-extractor-rs.exe
```

#### 2. 生产构建 (`cargo build --release`)

```
cargo build --release
│
├── 优化编译 (-O3)
├── 移除调试信息
├── 链接时优化 (LTO)
└── 生成: target/release/frame-extractor-rs.exe
    ├── 大小: ~5-10 MB
    ├── 性能: 优化后
    └── 包含: 所有依赖
```

### Cargo 配置

```toml
# Cargo.toml
[package]
name = "frame-extractor-rs"
version = "1.0.0"
edition = "2021"  # Rust 版本

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[dependencies]
tauri = { version = "2.0", features = [] }
tauri-plugin-dialog = "2.0"
tauri-plugin-fs = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
regex = "1.10"
```

### 构建脚本 (build.rs)

```rust
// src-tauri/build.rs
fn main() {
    tauri_build::build()
}
```

**作用**:
- 生成 Tauri 配置代码
- 处理资源文件
- 设置编译环境

---

## Tauri 集成构建流程

### 完整构建流程 (`npm run tauri build`)

```
npm run tauri build
│
├── 阶段 1: 前端构建
│   └── npm run build
│       └── 输出到 dist/
│
├── 阶段 2: Rust 编译
│   └── cargo build --release
│       └── 生成 frame-extractor-rs.exe
│
├── 阶段 3: 资源处理
│   ├── 复制前端资源到应用
│   ├── 处理 externalBin (FFmpeg)
│   └── 处理图标文件
│
└── 阶段 4: 打包安装程序
    ├── MSI (WiX)
    └── NSIS
```

### Tauri 构建配置

```json
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",      // 开发前命令
    "beforeBuildCommand": "npm run build",  // 构建前命令
    "devUrl": "http://localhost:1420",      // 开发 URL
    "frontendDist": "../dist"               // 前端输出目录
  },
  "bundle": {
    "active": true,
    "targets": "all",                       // 所有安装包格式
    "externalBin": ["bin/ffmpeg.exe"]       // 外部二进制文件
  }
}
```

---

## 构建产物

### 开发环境

```
src-tauri/target/debug/
├── frame-extractor-rs.exe          # 可执行文件 (未优化)
├── frame-extractor-rs.pdb          # 调试符号
└── build/                          # 构建中间文件
```

### 生产环境

```
src-tauri/target/release/
├── frame-extractor-rs.exe          # 可执行文件 (优化)
├── bundle/
│   ├── msi/
│   │   └── FrameExtractor_1.0.0_x64_en-US.msi
│   └── nsis/
│       └── FrameExtractor_1.0.0_x64-setup.exe
└── build/                          # 构建中间文件
```

---

## 构建时间分析

### 首次构建

- **前端**: ~200ms (依赖已安装)
- **Rust**: 5-10 分钟 (下载依赖)
- **打包**: ~2 分钟 (下载工具)
- **总计**: 6-12 分钟

### 增量构建

- **前端**: ~138ms (仅修改文件)
- **Rust**: 1-2 分钟 (仅编译变更)
- **打包**: ~1 分钟
- **总计**: 2-4 分钟

### 优化策略

1. **使用缓存**: `target/` 目录不要删除
2. **并行编译**: Cargo 自动使用多核
3. **增量编译**: 只编译修改的文件
4. **只构建需要的格式**: 修改 `targets` 配置

---

## 依赖管理

### 前端依赖

```json
// package.json
{
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-dialog": "^2.0.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "vite": "^5.0.0"
  }
}
```

**安装**: `npm install`
**更新**: `npm update`
**锁定**: `package-lock.json`

### Rust 依赖

```toml
# Cargo.toml
[dependencies]
tauri = { version = "2.0", features = [] }
```

**安装**: `cargo build` (自动)
**更新**: `cargo update`
**锁定**: `Cargo.lock`

---

## 环境变量

### 开发环境

- `TAURI_DEBUG=1` - 启用调试模式
- `RUST_LOG=debug` - Rust 日志级别

### 构建环境

- `TAURI_BUILD` - 构建模式
- `TAURI_PLATFORM` - 目标平台

---

## 常见问题

### Q: 为什么构建这么慢？

**A**: 首次构建需要：
- 下载 Rust 依赖
- 编译大量依赖库
- 下载打包工具

**解决方案**:
- 使用增量构建
- 保持 `target/` 目录
- 使用 SSD

### Q: 如何加速构建？

**A**:
1. 使用 `cargo build --release` 仅在生产构建
2. 开发时使用 `cargo build` (更快)
3. 禁用不必要的特性

### Q: 如何清理构建？

**A**:
```bash
# 清理 Rust 构建
cargo clean

# 清理前端构建
rm -rf dist

# 清理所有
cargo clean && rm -rf dist
```

---

## 相关文件

- `package.json` - 前端依赖
- `Cargo.toml` - Rust 依赖
- `vite.config.js` - Vite 配置
- `build.rs` - Rust 构建脚本
- `tauri.conf.json` - Tauri 配置

