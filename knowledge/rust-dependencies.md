# Rust 依赖库详解

## 依赖分类

本项目中的 Rust 依赖分为两类：

1. **标准库 (std)** - Rust 内置，无需声明
2. **第三方依赖** - 在 `Cargo.toml` 中声明

---

## 标准库 (std) 使用

### 核心模块

```rust
// src-tauri/src/extractor.rs
use std::path::{Path, PathBuf};  // 路径处理
use std::process::Command;         // 进程执行
```

**标准库特点**:
- ✅ 无需在 `Cargo.toml` 中声明
- ✅ 随 Rust 安装自动包含
- ✅ 稳定且文档完善
- ✅ 性能优异

### 本项目使用的标准库

| 模块 | 用途 | 示例 |
|------|------|------|
| `std::path` | 路径处理 | `PathBuf`, `Path::new()` |
| `std::process` | 进程执行 | `Command::new()` |
| `std::env` | 环境变量 | `std::env::current_exe()` |
| `std::fs` | 文件系统 | (通过 `tokio::fs` 使用异步版本) |

---

## 第三方依赖详解

### 构建时依赖 (build-dependencies)

#### tauri-build = "2.0"

**类型**: 构建时依赖  
**用途**: Tauri 构建脚本支持

```toml
[build-dependencies]
tauri-build = { version = "2.0", features = [] }
```

**作用**:
- 在 `build.rs` 中调用 `tauri_build::build()`
- 生成 Tauri 配置代码
- 处理资源文件和配置

**为什么是 build-dependencies**:
- 只在编译时使用
- 不会包含在最终二进制文件中
- 减少应用大小

---

### 运行时依赖 (dependencies)

#### tauri = "2.0"

**类型**: 运行时依赖  
**用途**: Tauri 框架核心

```toml
[dependencies]
tauri = { version = "2.0", features = [] }
```

**功能**:
- 桌面应用窗口管理
- 前端与后端通信 (IPC)
- 应用生命周期管理
- 系统集成

**使用示例**:
```rust
use tauri::Builder;

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**特性标志**:
- `custom-protocol` - 自定义协议支持 (用于生产构建)

---

#### tauri-plugin-dialog = "2.0"

**类型**: 运行时依赖  
**用途**: 文件对话框插件

```toml
[dependencies]
tauri-plugin-dialog = "2.0"
```

**功能**:
- 打开文件对话框
- 保存文件对话框
- 选择目录对话框

**使用示例**:
```rust
.use(tauri_plugin_dialog::init())
```

**前端调用**:
```javascript
import { open } from '@tauri-apps/plugin-dialog';
const selected = await open({ directory: true });
```

---

#### tauri-plugin-fs = "2.0"

**类型**: 运行时依赖  
**用途**: 文件系统操作插件

```toml
[dependencies]
tauri-plugin-fs = "2.0"
```

**功能**:
- 读取文件
- 写入文件
- 目录操作
- 文件权限管理

**使用示例**:
```rust
.use(tauri_plugin_fs::init())
```

**配置** (在 `tauri.conf.json`):
```json
{
  "plugins": {
    "fs": {
      "scope": ["**"]  // 允许访问所有路径
    }
  }
}
```

---

#### serde = "1.0" + serde_json = "1.0"

**类型**: 运行时依赖  
**用途**: 序列化和反序列化

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**功能**:
- **serde**: 序列化框架
- **serde_json**: JSON 序列化实现

**使用示例**:
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessResult {
    pub video_path: String,
    pub frames_extracted: usize,
    pub success: bool,
}
```

**为什么需要**:
- Tauri IPC 通信需要序列化
- 前端与后端数据交换
- 配置文件解析

**features**:
- `derive`: 自动生成序列化代码

---

#### tokio = "1.0"

**类型**: 运行时依赖  
**用途**: 异步运行时

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

**功能**:
- 异步运行时
- 异步文件 I/O
- 异步任务调度
- 网络编程支持

**使用示例**:
```rust
use tokio::fs;

async fn find_mp4_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(&dir).await?;
    // ...
}
```

**features**:
- `full`: 启用所有功能 (开发时)
- 生产环境可只启用需要的功能

**为什么需要**:
- 异步 I/O 提高性能
- 非阻塞文件操作
- 并发任务处理

---

#### anyhow = "1.0"

**类型**: 运行时依赖  
**用途**: 错误处理

```toml
[dependencies]
anyhow = "1.0"
```

**功能**:
- 简化的错误处理
- 错误链和上下文
- 动态错误类型

**使用示例**:
```rust
use anyhow::{Context, Result};

fn check_ffmpeg() -> Result<()> {
    let output = Command::new(&ffmpeg_path)
        .arg("-version")
        .output()
        .with_context(|| format!("无法执行 FFmpeg: {}", ffmpeg_path.display()))?;
    Ok(())
}
```

**为什么需要**:
- 比标准 `Result<T, E>` 更灵活
- 更好的错误消息
- 错误上下文追踪

**对比标准库**:
```rust
// 标准库
fn func() -> Result<(), Box<dyn std::error::Error>> { }

// anyhow
fn func() -> anyhow::Result<()> { }  // 更简洁
```

---

#### regex = "1.10"

**类型**: 运行时依赖  
**用途**: 正则表达式

```toml
[dependencies]
regex = "1.10"
```

**功能**:
- 正则表达式匹配
- 文本解析
- 模式搜索

**使用示例**:
```rust
use regex::Regex;

fn parse_duration(output: &str) -> Option<f64> {
    let re = Regex::new(r"Duration:\s*(\d{2}):(\d{2}):(\d{2})\.(\d{2})").ok()?;
    let caps = re.captures(output)?;
    // ...
}
```

**为什么需要**:
- 解析 FFmpeg 输出
- 提取视频信息
- 文本模式匹配

**标准库替代**:
- Rust 标准库没有正则表达式支持
- 需要第三方库

---

## 依赖关系图

```
frame-extractor-rs
│
├── tauri (框架)
│   ├── wry (WebView)
│   ├── tao (窗口管理)
│   └── ...
│
├── tauri-plugin-dialog
│   └── tauri (依赖)
│
├── tauri-plugin-fs
│   └── tauri (依赖)
│
├── serde (序列化)
│   └── serde_json (JSON 实现)
│
├── tokio (异步)
│   ├── tokio-macros
│   └── ...
│
└── anyhow (错误处理)
```

---

## 依赖大小分析

### 编译后大小估算

| 依赖 | 大小 | 说明 |
|------|------|------|
| `tauri` | ~3-5 MB | 包含 WebView 和窗口管理 |
| `tokio` | ~1-2 MB | 异步运行时 |
| `serde` | ~500 KB | 序列化框架 |
| `regex` | ~200 KB | 正则表达式 |
| `anyhow` | ~100 KB | 错误处理 |
| **总计** | **~5-8 MB** | 未压缩 |

**实际二进制大小**: ~10-15 MB (包含所有依赖)

---

## 依赖管理最佳实践

### 1. 版本选择

```toml
# 推荐: 使用语义版本
tauri = { version = "2.0", features = [] }

# 避免: 使用通配符
tauri = "*"  # ❌ 不推荐
```

### 2. 特性标志

```toml
# 只启用需要的特性
tokio = { version = "1.0", features = ["rt", "fs"] }  # ✅
tokio = { version = "1.0", features = ["full"] }      # ⚠️ 开发时可用
```

### 3. 依赖更新

```bash
# 检查更新
cargo outdated

# 更新依赖
cargo update

# 更新特定依赖
cargo update -p tauri
```

### 4. 安全审计

```bash
# 检查安全漏洞
cargo audit
```

---

## 依赖 vs 标准库对比

| 特性 | 标准库 (std) | 第三方依赖 |
|------|-------------|-----------|
| **可用性** | 内置，无需声明 | 需要声明 |
| **大小** | 最小 | 可能较大 |
| **稳定性** | 非常稳定 | 取决于库 |
| **性能** | 优化最佳 | 取决于实现 |
| **功能** | 基础功能 | 扩展功能 |
| **维护** | Rust 团队 | 社区/个人 |

---

## 相关文件

- `Cargo.toml` - 依赖声明
- `Cargo.lock` - 依赖锁定 (自动生成)
- `src-tauri/src/main.rs` - 依赖使用示例
- `src-tauri/src/extractor.rs` - 依赖使用示例

---

## 学习资源

- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [Cargo 文档](https://doc.rust-lang.org/cargo/)
- [crates.io](https://crates.io/) - Rust 包仓库
- [Rust Book](https://doc.rust-lang.org/book/) - Rust 官方教程

