# 前端技术栈

## 技术选型

本项目采用 **原生 Web 技术栈**，无框架依赖：

- **HTML5** - 页面结构
- **CSS3** - 样式定义
- **JavaScript (ES6+)** - 业务逻辑
- **Vite** - 构建工具

---

## 为什么选择原生技术？

### 优势

1. **轻量级**: 无框架开销
2. **快速**: 启动速度快
3. **简单**: 易于理解和维护
4. **兼容性**: Tauri WebView 完全支持

### 适用场景

- 小型到中型应用
- 性能关键应用
- 学习项目
- 快速原型

---

## 项目结构

```
frontend/
├── index.html      # 页面结构
├── style.css       # 样式定义
├── main.js         # 业务逻辑
└── (通过 Vite 构建)
    └── dist/       # 构建输出
```

---

## HTML 结构

### index.html

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>视频抽帧工具</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <!-- 应用界面 -->
    <script type="module" src="main.js"></script>
</body>
</html>
```

**特点**:
- 语义化 HTML
- 模块化 JavaScript (`type="module"`)
- 外部样式表

---

## CSS 样式

### style.css

**设计原则**:
- 现代 CSS 特性
- 响应式设计
- 美观的 UI

**主要特性**:
- Flexbox 布局
- CSS 变量
- 过渡动画
- 媒体查询

---

## JavaScript 逻辑

### main.js 结构

```javascript
// 1. 导入 Tauri API（必须在顶层）
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

// 2. 等待 DOM 加载完成
document.addEventListener('DOMContentLoaded', () => {
    initApp();
});

// 3. 初始化函数（所有 DOM 操作和事件监听器在这里）
function initApp() {
    // DOM 元素引用
    const inputDirInput = document.getElementById('inputDir');
    // ...

    // 事件监听器
    selectInputDirBtn.addEventListener('click', async () => {
        // 处理逻辑
    });

    // 业务函数
    function updateStartButtonState() {
        // ...
    }

    // 初始化
    updateStartButtonState();
}
```

### ⚠️ 代码执行时机

**重要**: 必须使用 `DOMContentLoaded` 事件确保代码在正确的时机执行：

1. **为什么需要？**
   - ES6 模块在顶层立即执行
   - DOM 元素可能还未加载
   - Tauri API 可能还未初始化

2. **正确做法**：
   ```javascript
   // ✅ 正确：等待 DOM 加载
   document.addEventListener('DOMContentLoaded', () => {
       // DOM 操作和事件监听器
   });
   
   // ❌ 错误：立即执行
   const element = document.getElementById('id'); // 可能为 null
   ```

3. **常见错误**：
   - `window.__TAURI_INTERNALS__ is undefined` - Tauri API 未初始化
   - `Cannot read property 'addEventListener' of null` - DOM 元素未加载

---

## Tauri API 使用

### 1. 核心 API

```javascript
import { invoke } from '@tauri-apps/api/core';

// 调用后端命令
const result = await invoke('process_videos', {
    inputDir: '...',
    outputDir: '...',
    // ...
});
```

### 2. Dialog 插件

```javascript
import { open } from '@tauri-apps/plugin-dialog';

// 打开目录选择对话框
const selected = await open({
    directory: true,
    multiple: false,
    title: '选择输入目录'
});
```

---

## 构建流程

### Vite 构建

```
npm run build
│
└── vite build
    ├── 入口: index.html
    ├── 模块解析
    ├── 代码转换 (ES6+ → ES5)
    ├── 代码压缩
    ├── 资源优化
    └── 输出到 dist/
```

### 构建配置

```javascript
// vite.config.js
export default defineConfig({
    server: {
        port: 1420,           // Tauri 开发端口
        strictPort: true,
    },
    build: {
        target: ['es2021', 'chrome100', 'safari13'],
        minify: 'esbuild',    // 压缩工具
        sourcemap: false,     // 生产环境不生成 sourcemap
    },
});
```

---

## 开发环境

### 开发服务器

```bash
npm run dev
```

**功能**:
- 热模块替换 (HMR)
- 实时重载
- 快速编译
- 开发工具支持

### 访问

- 本地: http://localhost:1420
- Tauri 自动连接

---

## 前端依赖

### 运行时依赖

```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-dialog": "^2.0.0"
  }
}
```

**说明**:
- `@tauri-apps/api`: Tauri 核心 API
- `@tauri-apps/plugin-dialog`: 对话框插件

### 开发依赖

```json
{
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "vite": "^5.0.0"
  }
}
```

**说明**:
- `@tauri-apps/cli`: Tauri CLI 工具
- `vite`: 构建工具

---

## 模块系统

### ES6 模块

```javascript
// 导入
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

// 导出 (如果需要)
export function myFunction() {
    // ...
}
```

### 模块特点

- 静态分析
- 树摇优化
- 作用域隔离
- 异步加载

---

## 异步编程

### async/await

```javascript
// 异步函数
async function selectDirectory() {
    try {
        const selected = await open({
            directory: true,
        });
        if (selected) {
            inputDirInput.value = selected;
        }
    } catch (error) {
        console.error('选择目录失败:', error);
        alert('选择目录失败: ' + error);
    }
}
```

### Promise

```javascript
// Promise 链
invoke('process_videos', {...})
    .then(results => {
        displayResults(results);
    })
    .catch(error => {
        console.error('处理失败:', error);
    });
```

---

## 错误处理

### try/catch

```javascript
try {
    const results = await invoke('process_videos', {...});
    displayResults(results);
} catch (error) {
    console.error('处理失败:', error);
    alert('处理失败: ' + error);
}
```

### 错误显示

```javascript
// 用户友好的错误提示
statusText.textContent = '处理失败: ' + error;
statusText.style.color = '#dc3545';
```

---

## 用户界面交互

### 事件处理

```javascript
// 点击事件
startProcessBtn.addEventListener('click', async () => {
    // 处理逻辑
});

// 输入事件
skipStartInput.addEventListener('input', () => {
    // 更新逻辑
});
```

### 状态管理

```javascript
// 按钮状态
function updateStartButtonState() {
    const hasInputDir = inputDirInput.value.trim() !== '';
    const hasOutputDir = outputDirInput.value.trim() !== '';
    startProcessBtn.disabled = !(hasInputDir && hasOutputDir);
}
```

---

## 数据处理

### 结果展示

```javascript
function displayResults(results) {
    // 统计
    const total = results.length;
    const successful = results.filter(r => r.success).length;
    const totalFrames = results.reduce((sum, r) => sum + r.frames_extracted, 0);
    
    // 显示
    resultsSummary.innerHTML = `
        <strong>处理完成！</strong><br>
        共处理 ${total} 个视频<br>
        成功: ${successful} 个<br>
        共提取 ${totalFrames} 张图片
    `;
}
```

---

## 性能优化

### 1. 代码压缩

- Vite 自动压缩
- 生产环境移除注释
- 变量名缩短

### 2. 资源优化

- CSS 压缩
- 图片优化 (如有)
- 代码分割 (如需)

### 3. 加载优化

- 延迟加载
- 按需加载
- 缓存策略

---

## 浏览器兼容性

### 目标环境

- **Chrome 100+** (Tauri WebView)
- **Safari 13+** (macOS)
- **现代浏览器特性**

### 不支持的特性

- 旧版 Internet Explorer
- 旧版 Edge
- 移动浏览器 (桌面应用)

---

## 相关文件

- `index.html` - 页面结构
- `style.css` - 样式定义
- `main.js` - 业务逻辑
- `vite.config.js` - 构建配置
- `package.json` - 依赖管理

---

## 学习资源

- [MDN Web Docs](https://developer.mozilla.org/)
- [JavaScript.info](https://javascript.info/)
- [Vite 文档](https://vitejs.dev/)
- [Tauri API 文档](https://tauri.app/api/)

