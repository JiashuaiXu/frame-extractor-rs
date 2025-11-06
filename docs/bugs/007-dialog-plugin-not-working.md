# Bug 007: 文件浏览对话框无法使用

## 错误现象

点击"浏览"按钮时，文件选择对话框无法打开，无法选择输入和输出目录。

## 问题分析

### 原因

在 **Tauri 2.0** 中，插件权限管理机制发生了变化：

1. **插件初始化**: 插件在 Rust 代码中通过 `.plugin()` 初始化 ✅
2. **权限配置**: 需要通过 **capabilities** 文件配置插件权限 ❌ **缺失**

### Tauri 2.0 权限系统

Tauri 2.0 引入了新的权限管理系统：
- 使用 `capabilities` 文件定义权限
- 每个窗口需要明确授予权限
- 插件需要显式声明权限

### 为什么之前能编译？

- 插件代码正确初始化
- 但运行时缺少权限配置
- 导致插件功能被阻止

---

## 解决方案

### 步骤 1: 创建 capabilities 目录

```powershell
New-Item -ItemType Directory -Force -Path "src-tauri\capabilities"
```

### 步骤 2: 创建 default.json

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

### 步骤 3: 验证配置

重新运行应用：

```bash
npm run tauri dev
```

现在文件浏览功能应该可以正常工作了。

---

## 权限说明

### Dialog 插件权限

- `dialog:default` - 默认权限集
- `dialog:open` - 打开文件/目录对话框
- `dialog:save` - 保存文件对话框

### FS 插件权限

- `fs:default` - 默认权限集
- `fs:read-file` - 读取文件
- `fs:write-file` - 写入文件
- `fs:read-dir` - 读取目录
- `fs:create-dir` - 创建目录
- `fs:remove-dir` - 删除目录
- `fs:remove-file` - 删除文件
- `fs:copy-file` - 复制文件
- `fs:rename-file` - 重命名文件
- `fs:exists` - 检查文件存在
- `fs:scope-all` - 允许访问所有路径 (`**`)

---

## 相关文件

- `src-tauri/capabilities/default.json` - 权限配置文件 ⭐ 新增
- `src-tauri/src/main.rs` - 插件初始化
- `main.js` - 前端对话框调用

---

## 状态

✅ **已修复** - 创建了 capabilities 配置文件，并修复了权限标识符格式

---

## 后续问题：权限标识符格式错误

### 错误信息

```
Permission dialog:open not found, expected one of ...
```

### 原因

Tauri 2.0 的权限标识符需要使用 `allow-` 前缀：
- ❌ `dialog:open` 
- ✅ `dialog:allow-open`

### 解决方案

修复 `src-tauri/capabilities/default.json` 中的权限标识符：

```json
{
  "permissions": [
    "dialog:default",
    "dialog:allow-open",    // ✅ 正确格式
    "dialog:allow-save",    // ✅ 正确格式
    "fs:default",
    "fs:allow-read-file",   // ✅ 正确格式
    "fs:allow-write-file",  // ✅ 正确格式
    "fs:allow-read-dir",    // ✅ 正确格式
    "fs:allow-mkdir",       // ✅ 正确格式
    "fs:allow-remove",      // ✅ 正确格式
    "fs:allow-rename",      // ✅ 正确格式
    "fs:allow-exists",      // ✅ 正确格式
    {
      "identifier": "fs:scope",  // ✅ 正确标识符
      "allow": [{"path": "**"}]
    }
  ]
}
```

### 权限标识符规则

在 Tauri 2.0 中：
- **允许权限**: `plugin:allow-command` (如 `dialog:allow-open`)
- **拒绝权限**: `plugin:deny-command` (如 `dialog:deny-open`)
- **默认权限集**: `plugin:default` (如 `dialog:default`)
- **作用域权限**: `plugin:scope` (如 `fs:scope`)

---

## 附加说明

### Tauri 2.0 权限系统变化

**Tauri 1.x**:
- 权限在 `tauri.conf.json` 中配置
- 使用 `plugins` 对象

**Tauri 2.0**:
- 权限在 `capabilities` 文件中配置
- 更细粒度的权限控制
- 每个窗口可以有不同的权限

### 最佳实践

1. **最小权限原则**: 只授予必要的权限
2. **明确路径**: 使用具体路径而不是 `**`
3. **窗口隔离**: 不同窗口可以有不同的权限

### 示例：限制文件访问范围

```json
{
  "identifier": "fs:scope-limited",
  "allow": [
    {
      "path": "$HOME/Documents/**"
    },
    {
      "path": "$HOME/Downloads/**"
    }
  ]
}
```

---

## 参考资源

- [Tauri 2.0 Capabilities](https://tauri.app/v2/guides/security/capabilities/)
- [Tauri 2.0 Dialog Plugin](https://v2.tauri.app/plugin/dialog/)

