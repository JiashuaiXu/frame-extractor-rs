# Bug #011: Vite 导入解析错误

## 错误信息

```
Failed to parse source for import analysis because the content contains invalid JS syntax. 
If you are using JSX, make sure to name the file with the .jsx or .tsx extension.
Plugin: vite:import-analysis
File: C:/Users/Administrator/Desktop/frame-extractor-rs/main.js:1:1
1  |  import { invoke } from '@tauri-apps/api/core';       
   |  ^
```

## 问题描述

在运行 `npm run tauri dev` 时，Vite 无法解析 `main.js` 文件中的 ES6 模块导入语句，报错说语法无效。

## 原因分析

这个问题通常由以下原因导致：

1. **Vite 配置问题**：Vite 可能没有正确识别 ES 模块语法
2. **文件编码问题**：文件可能包含 BOM 或其他不可见字符
3. **Vite 缓存问题**：旧的缓存可能导致解析错误
4. **依赖版本不匹配**：`@tauri-apps/api` 版本与 Vite 版本不兼容

## 解决方案

### 方案 1: 清理缓存并重新安装依赖（推荐）

```powershell
# 删除 node_modules 和缓存
Remove-Item -Recurse -Force node_modules
Remove-Item -Recurse -Force .vite
Remove-Item -Force package-lock.json

# 重新安装依赖
npm install

# 重新启动开发服务器
npm run tauri dev
```

### 方案 2: 更新 Vite 配置

在 `vite.config.js` 中明确指定文件处理方式：

```javascript
import { defineConfig } from 'vite';

export default defineConfig({
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
    },
    envPrefix: ['VITE_', 'TAURI_'],
    // 明确指定 ES 模块处理
    esbuild: {
        loader: 'jsx',
        include: /src\/.*\.[jt]sx?$/,
        exclude: [],
    },
    optimizeDeps: {
        esbuildOptions: {
            loader: {
                '.js': 'jsx',
            },
        },
    },
    build: {
        target: ['es2021', 'chrome100', 'safari13'],
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        sourcemap: !!process.env.TAURI_DEBUG,
    },
});
```

### 方案 3: 检查文件编码

确保 `main.js` 文件使用 UTF-8 编码（无 BOM）：

```powershell
# 检查文件编码（需要安装 chardet）
# 或者使用文本编辑器（如 VS Code）检查并转换编码
```

### 方案 4: 验证导入路径

确认 `@tauri-apps/api/event` 模块存在：

```powershell
# 检查模块是否存在
Test-Path "node_modules\@tauri-apps\api\event.d.ts"
```

如果文件存在，导入路径应该是正确的。

### 方案 5: 使用动态导入（临时方案）

如果上述方案都不行，可以尝试动态导入：

```javascript
// 在需要时动态导入
async function setupEventListeners() {
    const { listen } = await import('@tauri-apps/api/event');
    
    logListener = await listen('log', (event) => {
        // ...
    });
}
```

## 验证步骤

1. 清理缓存和重新安装依赖
2. 检查 `package.json` 中 `"type": "module"` 是否存在
3. 检查 `index.html` 中 `<script type="module">` 是否正确
4. 验证 `@tauri-apps/api` 版本是否正确安装
5. 重启开发服务器

## 相关文件

- `main.js` - 前端主文件
- `vite.config.js` - Vite 配置文件
- `package.json` - 项目依赖配置
- `index.html` - HTML 入口文件

## 根本原因

**文件语法错误**：`main.js` 文件缺少 `initApp()` 函数的闭合大括号 `}`，导致文件意外结束。

错误信息：
```
X [ERROR] Unexpected end of file
  main.js:248:0:
    248 │
        ╵ ^
```

## 最终解决方案

在 `main.js` 文件末尾添加缺失的闭合大括号：

```javascript
    // 清理事件监听器
    async function cleanupEventListeners() {
        // ...
    }
}  // ← 添加这个闭合大括号来闭合 initApp() 函数
```

## 状态

✅ 已修复

## 修复日期

2025-11-07

