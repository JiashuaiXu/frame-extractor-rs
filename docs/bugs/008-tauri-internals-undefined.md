# Bug 008: Tauri API 未初始化错误

**错误信息**：
```
TypeError: can't access property "invoke", window.__TAURI_INTERNALS__ is undefined
选择目录失败: TypeError: can't access property "invoke", window.__TAURI_INTERNALS__ is undefined
```

**问题描述**：
点击前端的选择路径按钮后，出现 `window.__TAURI_INTERNALS__ is undefined` 错误。这通常发生在以下情况：

1. **代码执行时机问题**：JavaScript 代码在 DOM 加载完成之前执行，导致无法访问 DOM 元素或 Tauri API
2. **Tauri API 未初始化**：Tauri 的全局对象 `__TAURI_INTERNALS__` 在代码执行时还未准备好
3. **模块导入时机**：ES6 模块导入的代码在顶层执行，可能在 Tauri 环境完全初始化之前就尝试访问 API

**根本原因**：
- `main.js` 中的代码在模块加载时立即执行，此时 DOM 可能还未加载完成
- 事件监听器在 DOM 元素存在之前就被注册，或者 Tauri API 还未完全初始化
- 没有使用 `DOMContentLoaded` 事件来确保代码在正确的时机执行

**解决方案**：
将所有初始化代码包装在 `DOMContentLoaded` 事件监听器中，确保：
1. DOM 元素已完全加载
2. Tauri API 已初始化
3. 事件监听器在正确的时机注册

**修改内容**：
```javascript
// 修改前：代码在模块顶层立即执行
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

const inputDirInput = document.getElementById('inputDir');
// ... 其他代码立即执行

// 修改后：等待 DOM 加载完成
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

// 等待 DOM 加载完成
document.addEventListener('DOMContentLoaded', () => {
    initApp();
});

function initApp() {
    // 所有初始化代码放在这里
    const inputDirInput = document.getElementById('inputDir');
    // ... 其他代码
}
```

**关键点**：
- 使用 `DOMContentLoaded` 确保 DOM 已加载
- 将所有 DOM 操作和事件监听器注册放在 `initApp()` 函数中
- 保持模块导入在顶层（这是 ES6 模块的要求）

**状态**：✅ 已修复

**相关文件**：
- `main.js`

