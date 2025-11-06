# Rust 基础库和核心概念

## 标准库 (std) 概述

Rust 标准库是 Rust 内置的核心库，无需在 `Cargo.toml` 中声明即可使用。

### 特点

- ✅ **零成本抽象** - 性能与手写代码相当
- ✅ **内存安全** - 编译时保证
- ✅ **并发安全** - 类型系统保证
- ✅ **跨平台** - 支持所有主流平台

---

## 本项目使用的标准库模块

### 1. std::path

**用途**: 路径处理

```rust
use std::path::{Path, PathBuf};

// Path - 不可变路径切片
let video_path = Path::new("video.mp4");

// PathBuf - 可变路径
let mut output = PathBuf::from("output");
output.push("frames");
output.push("frame_001.jpg");

// 路径操作
let parent = video_path.parent();           // 获取父目录
let stem = video_path.file_stem();         // 获取文件名（不含扩展名）
let extension = video_path.extension();     // 获取扩展名
```

**常用方法**:
- `Path::new()` - 创建路径
- `PathBuf::from()` - 从字符串创建
- `.join()` - 连接路径
- `.exists()` - 检查存在
- `.strip_prefix()` - 移除前缀

---

### 2. std::process

**用途**: 进程执行

```rust
use std::process::Command;

// 执行命令
let output = Command::new("ffmpeg")
    .arg("-version")
    .output()?;  // 等待执行完成

// 检查结果
if output.status.success() {
    println!("成功");
} else {
    println!("失败: {}", String::from_utf8_lossy(&output.stderr));
}
```

**常用方法**:
- `Command::new()` - 创建命令
- `.arg()` - 添加参数
- `.output()` - 执行并获取输出
- `.status()` - 获取退出状态
- `.stdout` / `.stderr` - 标准输出/错误

---

### 3. std::env

**用途**: 环境变量和系统信息

```rust
use std::env;

// 获取可执行文件路径
let exe_path = env::current_exe()?;

// 获取环境变量
let path = env::var("PATH")?;

// 设置环境变量
env::set_var("MY_VAR", "value");
```

**常用函数**:
- `env::current_exe()` - 当前可执行文件路径
- `env::var()` - 获取环境变量
- `env::set_var()` - 设置环境变量
- `env::args()` - 命令行参数

---

### 4. std::fs (通过 tokio::fs 使用异步版本)

**标准库版本** (同步):
```rust
use std::fs;

// 读取目录
let entries = fs::read_dir(".")?;

// 读取文件
let content = fs::read_to_string("file.txt")?;

// 写入文件
fs::write("output.txt", "content")?;
```

**异步版本** (本项目使用):
```rust
use tokio::fs;

// 异步读取目录
let mut entries = fs::read_dir(".").await?;

// 异步读取文件
let content = fs::read_to_string("file.txt").await?;

// 异步写入文件
fs::write("output.txt", "content").await?;
```

**为什么使用异步版本**:
- 非阻塞 I/O
- 更好的并发性能
- 适合处理大量文件

---

## Rust 核心概念

### 所有权 (Ownership)

```rust
// 所有权转移
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权转移给 s2
// println!("{}", s1);  // ❌ 错误: s1 已不再拥有数据

// 借用 (Borrowing)
let s = String::from("hello");
let len = calculate_length(&s);  // 借用，不转移所有权
println!("{}", s);  // ✅ 仍然可以使用

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

---

### 生命周期 (Lifetime)

```rust
// 生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

### 错误处理

#### Result 类型

```rust
use std::fs::File;

// Result<T, E> - 成功或错误
let file = File::open("file.txt");

match file {
    Ok(f) => println!("文件打开成功"),
    Err(e) => println!("错误: {}", e),
}

// 使用 ? 操作符
let file = File::open("file.txt")?;  // 错误时自动返回
```

#### Option 类型

```rust
// Option<T> - 有值或 None
let number = Some(5);
let no_number: Option<i32> = None;

match number {
    Some(n) => println!("值是: {}", n),
    None => println!("没有值"),
}
```

---

### 模式匹配

```rust
// match 表达式
match value {
    Some(x) => println!("值是: {}", x),
    None => println!("没有值"),
}

// if let
if let Some(x) = value {
    println!("值是: {}", x);
}

// while let
while let Some(item) = stack.pop() {
    println!("{}", item);
}
```

---

### 迭代器

```rust
// 迭代器使用
let v = vec![1, 2, 3];
let sum: i32 = v.iter().sum();

// map, filter, collect
let doubled: Vec<i32> = v.iter()
    .map(|x| x * 2)
    .collect();

// for 循环
for item in v.iter() {
    println!("{}", item);
}
```

---

### 异步编程

```rust
use tokio;

// async 函数
async fn fetch_data() -> Result<String, Error> {
    // 异步操作
    tokio::fs::read_to_string("file.txt").await
}

// 使用
#[tokio::main]
async fn main() -> Result<()> {
    let data = fetch_data().await?;
    Ok(())
}
```

---

## 类型系统

### 基本类型

```rust
// 整数
let x: i32 = 42;
let y: u64 = 100;

// 浮点数
let f: f64 = 3.14;

// 布尔值
let b: bool = true;

// 字符
let c: char = 'A';

// 字符串
let s: String = String::from("hello");
let s: &str = "hello";  // 字符串切片
```

### 复合类型

```rust
// 元组
let tup: (i32, f64, bool) = (1, 2.0, true);
let (x, y, z) = tup;  // 解构

// 数组
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// 向量
let vec: Vec<i32> = vec![1, 2, 3];
```

### 结构体

```rust
// 定义结构体
struct Person {
    name: String,
    age: u32,
}

// 创建实例
let person = Person {
    name: String::from("Alice"),
    age: 30,
};
```

### 枚举

```rust
// 定义枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 使用
let result: Result<i32, String> = Result::Ok(42);
```

---

## 内存管理

### 栈 (Stack) vs 堆 (Heap)

```rust
// 栈分配 - 固定大小，快速
let x = 5;  // i32 在栈上

// 堆分配 - 动态大小，需要指针
let s = String::from("hello");  // String 在堆上
```

### 智能指针

```rust
// Box - 堆分配
let b = Box::new(5);

// Rc - 引用计数
use std::rc::Rc;
let rc = Rc::new(5);

// Arc - 原子引用计数 (线程安全)
use std::sync::Arc;
let arc = Arc::new(5);
```

---

## 并发编程

### 线程

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("新线程");
});

handle.join().unwrap();
```

### 通道

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("消息").unwrap();
});

let received = rx.recv().unwrap();
```

---

## 本项目中的 Rust 特性使用

### 1. 泛型

```rust
// 泛型函数
fn get_output_dir<T: AsRef<Path>>(path: T) -> PathBuf {
    PathBuf::from(path.as_ref())
}
```

### 2. Trait

```rust
// 使用 trait
use std::fmt::Display;

fn print<T: Display>(item: T) {
    println!("{}", item);
}
```

### 3. 宏

```rust
// derive 宏
#[derive(Debug, Clone, Serialize)]
pub struct ProcessResult {
    // ...
}

// 函数式宏
println!("Hello, {}!", name);
```

---

## 学习资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [标准库文档](https://doc.rust-lang.org/std/)

---

## 相关文件

- `src-tauri/src/main.rs` - 主程序入口
- `src-tauri/src/extractor.rs` - 核心逻辑，大量使用标准库

