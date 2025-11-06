# Bug 记录 001-005：编译和配置问题

## Bug 001: Tauri 2.0 配置字段过时

**错误信息**：
```
unknown field `devPath`, expected one of `runner`, `dev-url`, `devUrl`, ...
unknown field `distDir`, expected one of `runner`, `dev-url`, `devUrl`, `frontend-dist`, `frontendDist`, ...
```

**问题描述**：
- Tauri 2.0 中配置字段名称已更改
- `devPath` 已改为 `devUrl` 或 `dev-url`
- `distDir` 已改为 `frontendDist` 或 `frontend-dist`

**解决方案**：
修改 `src-tauri/tauri.conf.json`：
```json
{
  "build": {
    "devUrl": "http://localhost:1420",  // 从 devPath 改为 devUrl
    "frontendDist": "../dist"            // 从 distDir 改为 frontendDist
  }
}
```

**状态**：✅ 已修复

---

## Bug 002: 外部二进制文件命名问题

**错误信息**：
```
resource path `bin\ffmpeg.exe-x86_64-pc-windows-msvc.exe` doesn't exist
```

**问题描述**：
- Tauri 2.0 要求外部二进制文件按照目标三元组命名
- 格式：`{original_name}-{target_triple}.exe`
- Windows 上目标三元组为 `x86_64-pc-windows-msvc`

**解决方案**：
将 `src-tauri/bin/ffmpeg.exe` 复制为 `src-tauri/bin/ffmpeg.exe-x86_64-pc-windows-msvc.exe`

**命令**：
```powershell
Copy-Item "src-tauri\bin\ffmpeg.exe" "src-tauri\bin\ffmpeg.exe-x86_64-pc-windows-msvc.exe"
```

**状态**：✅ 已修复

---

## Bug 003: Rust 代码编译错误

**错误信息**：
```
error[E0308]: mismatched types
   --> src\extractor.rs:176:64
    |
176 |         if let Ok(rel_path) = video_path.parent().and_then(|p| p.strip_prefix(input_root)) {
    |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<_>`, found `Result<&Path, StripPrefixError>`
```

**问题描述**：
- `Path::strip_prefix()` 返回 `Result<&Path, StripPrefixError>`，不是 `Option`
- 不能直接在 `and_then` 中使用 `Result` 类型
- 另有一个未使用的变量 `fps` 警告

**解决方案**：
修改 `src-tauri/src/extractor.rs`：

1. 修复 `strip_prefix` 使用：
```rust
// 修改前
if let Ok(rel_path) = video_path.parent().and_then(|p| p.strip_prefix(input_root)) {

// 修改后
if let Some(parent) = video_path.parent() {
    if let Ok(rel_path) = parent.strip_prefix(input_root) {
        output_dir = output_dir.join(rel_path);
    }
}
```

2. 修复未使用变量警告：
```rust
// 修改前
let (duration, fps) = get_video_info(&ffmpeg_path, video_path)?;

// 修改后
let (duration, _fps) = get_video_info(&ffmpeg_path, video_path)?;
```

**状态**：✅ 已修复

---

## Bug 004: 前端依赖缺失

**错误信息**：
```
[vite]: Rollup failed to resolve import "@tauri-apps/plugin-dialog" from "main.js".
```

**问题描述**：
- `main.js` 中导入了 `@tauri-apps/plugin-dialog`
- 但 `package.json` 中缺少该依赖

**解决方案**：
在 `package.json` 中添加依赖：
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-dialog": "^2.0.0"  // 添加此行
  }
}
```

然后运行：
```bash
npm install
```

**状态**：✅ 已修复

---

## Bug 005: Tauri 2.0 插件配置格式问题

**错误信息**：
```
error while running tauri application: PluginInitialization("dialog", 
"Error deserializing 'plugins.dialog' within your Tauri configuration: 
invalid type: map, expected unit")
```

**问题描述**：
- Tauri 2.0 不再在 `tauri.conf.json` 中声明插件配置
- 插件通过 Rust 代码中的 `.plugin()` 方法初始化
- 权限和范围通过 capabilities 文件管理，而不是在配置文件中

**解决方案**：
完全移除 `tauri.conf.json` 中的 `plugins` 配置部分：

```json
{
  // ... 其他配置
  // 删除整个 plugins 部分
}
```

**原因**：
- Tauri 2.0 改变了插件配置方式
- 插件在 `main.rs` 中通过 `.plugin(tauri_plugin_dialog::init())` 初始化
- 配置文件中的插件声明已不再需要

**状态**：✅ 已修复

---

## 总结

所有编译和配置问题已解决。项目现在可以成功编译，但需要注意：

1. **Tauri 2.0 配置变化**：多个字段名称已更改，需要更新配置文件
2. **外部二进制文件**：需要按照目标三元组命名
3. **插件配置**：Tauri 2.0 不再在配置文件中声明插件
4. **权限管理**：如果需要配置文件系统权限，应该通过 capabilities 文件而不是 `tauri.conf.json`

## 相关文件

- `src-tauri/tauri.conf.json` - Tauri 配置文件
- `src-tauri/src/extractor.rs` - Rust 抽帧逻辑
- `src-tauri/src/main.rs` - Tauri 主入口
- `package.json` - 前端依赖配置
- `src-tauri/bin/ffmpeg.exe-x86_64-pc-windows-msvc.exe` - 外部二进制文件

