import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { listen } from '@tauri-apps/api/event';

// 等待 DOM 加载完成
document.addEventListener('DOMContentLoaded', () => {
    initApp();
});

function initApp() {
    // DOM 元素
    const inputDirInput = document.getElementById('inputDir');
    const outputDirInput = document.getElementById('outputDir');
    const selectInputDirBtn = document.getElementById('selectInputDir');
    const selectOutputDirBtn = document.getElementById('selectOutputDir');
    const skipStartInput = document.getElementById('skipStart');
    const frameIntervalInput = document.getElementById('frameInterval');
    const preserveDirStructureCheck = document.getElementById('preserveDirStructure');
    const createVideoSubdirCheck = document.getElementById('createVideoSubdir');
    const startProcessBtn = document.getElementById('startProcess');
    const progressSection = document.getElementById('progressSection');
    const progressBar = document.getElementById('progressBar');
    const progressText = document.getElementById('progressText');
    const statusText = document.getElementById('statusText');
    const resultsSection = document.getElementById('resultsSection');
    const resultsSummary = document.getElementById('resultsSummary');
    const resultsList = document.getElementById('resultsList');
    const logOutput = document.getElementById('logOutput');
    const clearLogBtn = document.getElementById('clearLogBtn');

    // 更新开始按钮状态
    function updateStartButtonState() {
        const hasInputDir = inputDirInput.value.trim() !== '';
        const hasOutputDir = outputDirInput.value.trim() !== '';
        startProcessBtn.disabled = !(hasInputDir && hasOutputDir);
    }

    // 选择输入目录
    selectInputDirBtn.addEventListener('click', async () => {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: '选择输入目录（包含 MP4 视频）'
            });
            
            if (selected) {
                inputDirInput.value = selected;
                updateStartButtonState();
            }
        } catch (error) {
            console.error('选择目录失败:', error);
            alert('选择目录失败: ' + error);
        }
    });

    // 选择输出目录
    selectOutputDirBtn.addEventListener('click', async () => {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: '选择输出目录（保存提取的图片）'
            });
            
            if (selected) {
                outputDirInput.value = selected;
                updateStartButtonState();
            }
        } catch (error) {
            console.error('选择目录失败:', error);
            alert('选择目录失败: ' + error);
        }
    });

    // 开始处理
    startProcessBtn.addEventListener('click', async () => {
        const inputDir = inputDirInput.value.trim();
        const outputDir = outputDirInput.value.trim();
        const skipStartSec = parseFloat(skipStartInput.value) || 0;
        const frameIntervalSec = parseFloat(frameIntervalInput.value) || 5;
        const preserveDirStructure = preserveDirStructureCheck.checked;
        const createVideoSubdir = createVideoSubdirCheck.checked;

        if (!inputDir || !outputDir) {
            alert('请先选择输入和输出目录');
            return;
        }

        // 禁用按钮，显示进度
        startProcessBtn.disabled = true;
        progressSection.style.display = 'block';
        resultsSection.style.display = 'none';
        progressBar.style.width = '0%';
        progressText.textContent = '0%';
        statusText.textContent = '正在处理...';
        logOutput.innerHTML = ''; // 清空日志

        // 设置事件监听器
        await setupEventListeners();

        try {
            const results = await invoke('process_videos', {
                inputDir,
                outputDir,
                skipStartSec,
                frameIntervalSec,
                preserveDirStructure,
                createVideoSubdir,
            });

            // 更新进度为 100%
            progressBar.style.width = '100%';
            progressText.textContent = '100%';
            statusText.textContent = '处理完成！';

            // 显示结果
            displayResults(results);
            
        } catch (error) {
            console.error('处理失败:', error);
            statusText.textContent = '处理失败: ' + error;
            statusText.style.color = '#dc3545';
            alert('处理失败: ' + error);
        } finally {
            // 清理事件监听器
            await cleanupEventListeners();
            startProcessBtn.disabled = false;
        }
    });

    // 显示结果
    function displayResults(results) {
        resultsSection.style.display = 'block';
        
        const total = results.length;
        const successful = results.filter(r => r.success).length;
        const failed = total - successful;
        const totalFrames = results.reduce((sum, r) => sum + r.frames_extracted, 0);

        resultsSummary.innerHTML = `
            <strong>处理完成！</strong><br>
            共处理 ${total} 个视频<br>
            成功: ${successful} 个 | 失败: ${failed} 个<br>
            共提取 ${totalFrames} 张图片
        `;

        resultsList.innerHTML = '';
        results.forEach((result, index) => {
            const item = document.createElement('div');
            item.className = `result-item ${result.success ? 'success' : 'error'}`;
            
            const fileName = result.video_path.split(/[/\\]/).pop();
            
            item.innerHTML = `
                <div class="result-item-header">
                    <div class="result-item-title" title="${result.video_path}">
                        ${index + 1}. ${fileName}
                    </div>
                    <div class="result-item-status ${result.success ? 'success' : 'error'}">
                        ${result.success ? '✓ 成功' : '✗ 失败'}
                    </div>
                </div>
                <div class="result-item-details">
                    ${result.success 
                        ? `提取了 ${result.frames_extracted} 张图片` 
                        : `错误: ${result.error || '未知错误'}`}
                </div>
                <div class="result-item-path" title="${result.output_dir}">
                    输出: ${result.output_dir}
                </div>
            `;
            
            resultsList.appendChild(item);
        });
    }

    // 清空日志
    clearLogBtn.addEventListener('click', () => {
        logOutput.innerHTML = '';
    });

    // 添加日志行
    function addLogLine(level, message, timestamp) {
        const logLine = document.createElement('div');
        logLine.className = `log-line ${level}`;
        
        const timestampSpan = timestamp ? `<span class="log-timestamp">${timestamp}</span>` : '';
        const levelSpan = `<span class="log-level ${level}"></span>`;
        
        logLine.innerHTML = `${timestampSpan}${levelSpan}${escapeHtml(message)}`;
        logOutput.appendChild(logLine);
        
        // 自动滚动到底部
        logOutput.scrollTop = logOutput.scrollHeight;
    }

    // HTML 转义
    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    // 监听日志事件
    let logListener = null;
    let progressListener = null;

    // 初始化
    updateStartButtonState();
    
    // 设置事件监听器（在开始处理时）
    async function setupEventListeners() {
        // 清理旧的监听器
        if (logListener) {
            await logListener();
        }
        if (progressListener) {
            await progressListener();
        }

        // 监听日志事件
        logListener = await listen('log', (event) => {
            const { level, message, timestamp } = event.payload;
            addLogLine(level, message, timestamp);
        });

        // 监听进度事件
        progressListener = await listen('progress', (event) => {
            const { current, total, percentage, message } = event.payload;
            progressBar.style.width = `${percentage}%`;
            progressText.textContent = `${percentage}%`;
            statusText.textContent = message || `处理中: ${current}/${total}`;
        });
    }
    
    // 清理事件监听器
    async function cleanupEventListeners() {
        if (logListener) {
            await logListener();
            logListener = null;
        }
        if (progressListener) {
            await progressListener();
            progressListener = null;
        }
    }
}
