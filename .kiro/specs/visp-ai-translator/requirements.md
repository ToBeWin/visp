# Requirements Document

## Introduction

Visp 是一款全离线、零成本、极简、跨平台的 AI 翻译助手应用。它专为开发者和办公族设计，通过集成 Whisper Turbo 语音识别和 Qwen3.5 0.8B 大语言模型，实现"语音即翻译"与"选中即替换"的无感交互体验。所有推理均在本地完成，确保隐私安全和零网络延迟。

## Glossary

- **Visp_Application**: 基于 Tauri 2.0 构建的跨平台桌面应用程序
- **ASR_Engine**: 语音识别引擎，使用 Whisper-large-v3-turbo 模型
- **Translation_Engine**: 翻译引擎，使用 Qwen3.5-0.8B-Instruct 模型
- **Island_Component**: 屏幕顶部中央的状态显示组件
- **Glow_Component**: 显示翻译结果的半透明预览窗口
- **Voice_Mode**: 通过麦克风录音进行语音翻译的工作模式
- **Text_Mode**: 处理剪贴板文本进行翻译的工作模式
- **System_Clipboard**: 操作系统的剪贴板服务
- **Keyboard_Simulator**: 模拟键盘输入的组件（使用 Enigo 库）

## Requirements

### Requirement 1: 应用程序初始化

**User Story:** 作为用户，我希望应用能够快速启动并加载必要的模型，以便立即开始使用翻译功能。

#### Acceptance Criteria

1. WHEN Visp_Application 启动时，THE Visp_Application SHALL 在 5 秒内完成初始化
2. WHEN 初始化过程中，THE Visp_Application SHALL 加载 ASR_Engine 和 Translation_Engine 的量化模型
3. IF 模型文件缺失或损坏，THEN THE Visp_Application SHALL 显示错误提示并提供模型下载指引
4. WHEN 初始化完成后，THE Visp_Application SHALL 在系统托盘显示图标并进入就绪状态
5. THE Visp_Application SHALL 占用内存不超过 2GB

### Requirement 2: 全局快捷键注册

**User Story:** 作为用户，我希望通过快捷键快速触发翻译功能，无需切换到应用窗口。

#### Acceptance Criteria

1. WHEN Visp_Application 启动后，THE Visp_Application SHALL 注册全局快捷键 Alt+V 用于语音翻译
2. WHEN Visp_Application 启动后，THE Visp_Application SHALL 注册全局快捷键 Alt+C 用于文本翻译
3. IF 快捷键已被其他应用占用，THEN THE Visp_Application SHALL 显示冲突提示并允许用户自定义快捷键
4. THE Visp_Application SHALL 在后台持续监听已注册的全局快捷键

### Requirement 3: 语音录制功能

**User Story:** 作为用户，我希望长按快捷键时能够录制语音，松开后自动处理，实现流畅的语音输入体验。

#### Acceptance Criteria

1. WHEN 用户按下 Alt+V 键时，THE Visp_Application SHALL 开始捕获麦克风音频流
2. WHEN 录音开始时，THE Island_Component SHALL 显示在屏幕顶部中央并展示实时波形动画
3. WHILE 录音进行中，THE ASR_Engine SHALL 缓存音频数据
4. WHEN 用户释放 Alt+V 键时，THE Visp_Application SHALL 停止录音并触发语音识别处理
5. IF 录音时长少于 0.5 秒，THEN THE Visp_Application SHALL 忽略此次录音
6. IF 录音时长超过 60 秒，THEN THE Visp_Application SHALL 自动停止录音并开始处理
7. IF 麦克风访问权限被拒绝，THEN THE Visp_Application SHALL 显示权限请求提示

### Requirement 4: 语音识别处理

**User Story:** 作为用户，我希望语音能够快速准确地转换为文本，支持中英文混合识别。

#### Acceptance Criteria

1. WHEN 录音结束后，THE ASR_Engine SHALL 在 3 秒内将音频转换为文本
2. THE ASR_Engine SHALL 支持中文、英文及中英混合语音识别
3. WHEN 识别进行中，THE Island_Component SHALL 显示"Thinking"状态
4. IF 音频质量过低或无法识别，THEN THE ASR_Engine SHALL 返回错误信息并在 Island_Component 显示提示
5. WHEN 识别完成后，THE ASR_Engine SHALL 将识别文本传递给 Translation_Engine

### Requirement 5: 翻译处理

**User Story:** 作为用户，我希望系统能够智能识别语言并进行准确翻译，支持中英互译。

#### Acceptance Criteria

1. WHEN Translation_Engine 接收到文本时，THE Translation_Engine SHALL 自动检测源语言
2. IF 源语言为中文，THEN THE Translation_Engine SHALL 翻译为英文
3. IF 源语言为英文，THEN THE Translation_Engine SHALL 翻译为中文
4. THE Translation_Engine SHALL 在 2 秒内完成翻译处理
5. THE Translation_Engine SHALL 保留代码片段、专有名词和技术术语的原始格式
6. WHEN 翻译完成后，THE Translation_Engine SHALL 返回翻译结果文本

### Requirement 6: 语音模式输出

**User Story:** 作为用户，我希望翻译结果能够自动输入到当前光标位置，无需手动粘贴。

#### Acceptance Criteria

1. WHEN Voice_Mode 的翻译完成后，THE Keyboard_Simulator SHALL 将翻译结果模拟键盘输入到当前活动窗口的光标位置
2. THE Keyboard_Simulator SHALL 保持原文本的换行和格式
3. WHEN 输入完成后，THE Island_Component SHALL 淡出消失
4. IF 当前窗口不支持文本输入，THEN THE Visp_Application SHALL 将翻译结果复制到 System_Clipboard 并显示提示

### Requirement 7: 文本模式触发

**User Story:** 作为用户，我希望选中文本后按快捷键即可翻译，提高文本处理效率。

#### Acceptance Criteria

1. WHEN 用户按下 Alt+C 键时，THE Visp_Application SHALL 模拟 Ctrl+C 操作复制选中文本
2. WHEN 复制操作完成后，THE Visp_Application SHALL 读取 System_Clipboard 内容
3. IF System_Clipboard 为空或不包含文本，THEN THE Visp_Application SHALL 显示提示并终止处理
4. WHEN 获取到文本后，THE Visp_Application SHALL 将文本传递给 Translation_Engine 进行翻译

### Requirement 8: 翻译结果预览

**User Story:** 作为用户，我希望在替换原文前能够预览翻译结果，确认无误后再执行替换。

#### Acceptance Criteria

1. WHEN Text_Mode 的翻译完成后，THE Glow_Component SHALL 在屏幕中央显示半透明预览窗口
2. THE Glow_Component SHALL 展示翻译结果文本并带有 1px 渐变流光边框动画
3. THE Glow_Component SHALL 显示操作提示："按 Enter 替换 / 按 Esc 取消"
4. WHEN 用户按下 Enter 键时，THE Visp_Application SHALL 执行原地替换操作
5. WHEN 用户按下 Esc 键时，THE Glow_Component SHALL 淡出消失
6. IF 用户在 10 秒内无操作，THEN THE Glow_Component SHALL 自动淡出消失

### Requirement 9: 文本替换功能

**User Story:** 作为用户，我希望确认翻译后能够自动替换原文本，无需手动操作。

#### Acceptance Criteria

1. WHEN 用户在 Glow_Component 中按下 Enter 键时，THE Visp_Application SHALL 将翻译结果复制到 System_Clipboard
2. WHEN 复制完成后，THE Keyboard_Simulator SHALL 模拟 Ctrl+V 操作粘贴翻译结果
3. THE Visp_Application SHALL 在替换完成后恢复 System_Clipboard 的原始内容
4. WHEN 替换完成后，THE Glow_Component SHALL 淡出消失

### Requirement 10: UI 动画与视觉效果

**User Story:** 作为用户，我希望界面动画流畅自然，提供愉悦的视觉体验。

#### Acceptance Criteria

1. THE Island_Component SHALL 使用毛玻璃效果（Glassmorphism）样式
2. WHEN Island_Component 出现或消失时，THE Island_Component SHALL 使用弹簧物理动画（Spring Physics）实现平滑过渡
3. THE Glow_Component SHALL 使用毛玻璃效果和渐变流光边框
4. WHEN Glow_Component 出现或消失时，THE Glow_Component SHALL 使用淡入淡出动画，持续时间为 300ms
5. WHILE 录音进行中，THE Island_Component SHALL 显示实时音频波形可视化动画
6. THE Visp_Application SHALL 确保所有动画帧率不低于 60fps

### Requirement 11: 离线运行能力

**User Story:** 作为用户，我希望应用完全离线运行，保护隐私并避免网络依赖。

#### Acceptance Criteria

1. THE Visp_Application SHALL 在无网络连接的情况下正常运行所有核心功能
2. THE ASR_Engine SHALL 使用本地量化模型（whisper-large-v3-turbo Q4_0）进行推理
3. THE Translation_Engine SHALL 使用本地量化模型（Qwen3.5-0.8B-Instruct GGUF Q4_K_M）进行推理
4. THE Visp_Application SHALL 不向任何外部服务器发送用户数据
5. THE Visp_Application SHALL 不依赖 Python 运行时或 Ollama 等外部服务

### Requirement 12: 跨平台支持

**User Story:** 作为用户，我希望应用能够在不同操作系统上运行，保持一致的体验。

#### Acceptance Criteria

1. THE Visp_Application SHALL 支持 Windows 10 及以上版本
2. THE Visp_Application SHALL 支持 macOS 11 及以上版本
3. THE Visp_Application SHALL 支持主流 Linux 发行版（Ubuntu 20.04+, Fedora 35+）
4. THE Visp_Application SHALL 在所有支持的平台上提供相同的核心功能
5. THE Visp_Application 的安装包大小 SHALL 不超过 20MB（不含模型文件）

### Requirement 13: 配置与自定义

**User Story:** 作为用户，我希望能够自定义快捷键和其他设置，适应个人使用习惯。

#### Acceptance Criteria

1. THE Visp_Application SHALL 提供设置界面用于自定义全局快捷键
2. THE Visp_Application SHALL 允许用户选择翻译方向（自动检测/固定方向）
3. THE Visp_Application SHALL 允许用户调整 UI 透明度和动画速度
4. THE Visp_Application SHALL 将用户配置持久化存储到本地配置文件
5. WHEN 配置更改后，THE Visp_Application SHALL 立即应用新配置无需重启

### Requirement 14: 错误处理与日志

**User Story:** 作为用户，我希望应用能够妥善处理错误情况，并提供有用的反馈信息。

#### Acceptance Criteria

1. IF 任何推理过程发生错误，THEN THE Visp_Application SHALL 在 Island_Component 或 Glow_Component 显示友好的错误提示
2. THE Visp_Application SHALL 记录错误日志到本地日志文件
3. THE Visp_Application SHALL 在设置界面提供日志查看和导出功能
4. IF 应用崩溃，THEN THE Visp_Application SHALL 在下次启动时尝试恢复到安全状态
5. THE Visp_Application SHALL 限制日志文件大小不超过 50MB，超出时自动轮转

### Requirement 15: 性能与资源管理

**User Story:** 作为用户，我希望应用运行流畅，不占用过多系统资源。

#### Acceptance Criteria

1. WHILE 空闲状态，THE Visp_Application SHALL 占用 CPU 不超过 1%
2. WHILE 推理进行中，THE Visp_Application SHALL 占用 CPU 不超过 80%
3. THE Visp_Application SHALL 在推理完成后释放 GPU 资源（如使用 GPU 加速）
4. THE Visp_Application SHALL 在后台运行时占用内存不超过 500MB
5. WHEN 用户最小化应用到系统托盘时，THE Visp_Application SHALL 释放所有 UI 相关资源
