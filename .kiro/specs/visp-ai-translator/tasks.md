# Implementation Plan: Visp AI 翻译助手

## Overview

本实现计划将 Visp AI 翻译助手分解为可执行的编码任务。应用基于 Tauri 2.0 架构，使用 Rust 后端处理核心推理和系统集成，React + TypeScript 前端实现动画丰富的用户界面。实现将按照从底层到上层的顺序进行：首先搭建项目结构和配置，然后实现推理引擎，接着构建系统集成层，最后完成前端 UI 和端到端集成。

## Tasks

- [x] 1. 项目初始化和基础配置
  - 创建 Tauri 2.0 项目结构
  - 配置 Rust 工作空间和依赖项（candle, cpal, arboard, enigo, global-hotkey）
  - 配置前端 React + TypeScript + Vite 环境
  - 设置 Framer Motion 和 Tailwind CSS
  - 创建跨平台构建配置文件
  - _Requirements: 1.1, 1.4, 12.1, 12.2, 12.3, 12.5_

- [x] 2. 数据模型和配置管理
  - [x] 2.1 实现配置数据结构
    - 创建 `AppConfig`, `HotkeyConfig`, `TranslationConfig`, `UiConfig`, `LoggingConfig` 结构体
    - 实现配置文件的序列化和反序列化
    - 实现跨平台配置文件路径解析
    - _Requirements: 13.1, 13.2, 13.3, 13.4_
  
  - [ ]* 2.2 编写配置管理单元测试
    - 测试配置加载、保存和默认值
    - 测试跨平台路径解析
    - _Requirements: 13.4_
  
  - [x] 2.3 实现运行时状态管理
    - 创建 `AppState` 结构体和状态管理逻辑
    - 实现线程安全的状态访问（Arc, Mutex, RwLock）
    - _Requirements: 1.2, 1.4_

- [x] 3. 模型加载器实现
  - [x] 3.1 实现 ModelLoader 组件
    - 创建 `ModelLoader` 结构体
    - 实现跨平台模型文件路径解析
    - 实现模型文件存在性检查和完整性验证
    - _Requirements: 1.2, 1.3, 11.2, 11.3_
  
  - [ ]* 3.2 编写模型加载器单元测试
    - 测试模型文件检查逻辑
    - 测试路径解析在不同平台的正确性
    - _Requirements: 1.3_

- [x] 4. ASR 引擎实现
  - [x] 4.1 实现 Whisper 模型加载
    - 使用 candle-transformers 加载 GGUF 格式 Whisper 模型
    - 实现 `WhisperModel` 结构体（encoder, decoder, tokenizer）
    - 实现模型初始化和设备选择逻辑
    - _Requirements: 1.2, 11.2_
  
  - [x] 4.2 实现音频预处理
    - 实现音频重采样到 16kHz
    - 实现音频归一化到 [-1, 1]
    - _Requirements: 4.1_
  
  - [x] 4.3 实现语音识别推理
    - 实现 `AsrEngine::transcribe` 方法
    - 实现语言检测功能（中文、英文、自动检测）
    - 确保推理时间 < 3 秒
    - _Requirements: 4.1, 4.2, 4.5_
  
  - [ ]* 4.4 编写 ASR 引擎单元测试
    - 测试音频预处理逻辑
    - 测试语言检测准确性
    - _Requirements: 4.2, 4.4_

- [x] 5. 翻译引擎实现
  - [x] 5.1 实现 Qwen 模型加载
    - 使用 candle 加载 GGUF 格式 Qwen3.5 模型
    - 实现 `LlamaModel` 结构体和 tokenizer
    - _Requirements: 1.2, 11.3_
  
  - [x] 5.2 实现语言检测逻辑
    - 实现中英文字符统计和语言判断
    - 实现 `detect_language` 函数
    - _Requirements: 5.1_
  
  - [x] 5.3 实现翻译推理
    - 实现 `TranslationEngine::translate` 方法
    - 实现 Prompt 模板构建（中译英、英译中）
    - 确保推理时间 < 2 秒
    - 实现代码片段和技术术语保留逻辑
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 5.6_
  
  - [ ]* 5.4 编写翻译引擎单元测试
    - 测试语言检测准确性
    - 测试格式保留功能
    - _Requirements: 5.1, 5.5_

- [x] 6. Checkpoint - 验证推理引擎
  - 确保 ASR 和翻译引擎能够独立运行并产生正确输出，询问用户是否有问题

- [x] 7. 音频捕获实现
  - [x] 7.1 实现 AudioCapture 组件
    - 使用 cpal 初始化麦克风设备
    - 实现音频流捕获和缓存（16kHz, 单声道, f32）
    - 实现 `start_recording` 和 `stop_recording` 方法
    - _Requirements: 3.1, 3.3_
  
  - [x] 7.2 实现实时波形数据提取
    - 实现 `get_waveform` 方法用于 UI 可视化
    - _Requirements: 3.2, 10.5_
  
  - [x] 7.3 实现麦克风权限检查
    - 实现 `check_permission` 方法
    - _Requirements: 3.7_
  
  - [ ]* 7.4 编写音频捕获单元测试
    - 测试录音时长限制（0.5 秒 - 60 秒）
    - 测试音频格式正确性
    - _Requirements: 3.5, 3.6_

- [x] 8. 剪贴板管理器实现
  - [x] 8.1 实现 ClipboardManager 组件
    - 使用 arboard 实现剪贴板读写
    - 实现 `read_text`, `write_text`, `backup`, `restore` 方法
    - _Requirements: 7.2, 9.1, 9.3_
  
  - [x] 8.2 实现 Ctrl+C 模拟
    - 使用 Enigo 实现 `simulate_copy` 方法
    - 添加 100ms 延迟等待剪贴板更新
    - _Requirements: 7.1_
  
  - [ ]* 8.3 编写剪贴板管理器单元测试
    - 测试备份和恢复功能
    - 测试空剪贴板处理
    - _Requirements: 7.3, 9.3_

- [x] 9. 键盘模拟器实现
  - [x] 9.1 实现 KeyboardSimulator 组件
    - 使用 Enigo 实现键盘模拟
    - 实现 `type_text` 方法（支持 Unicode，10ms/字符延迟）
    - 实现 `paste` 方法（模拟 Ctrl+V）
    - 实现换行符处理（\n → Enter 键）
    - _Requirements: 6.1, 6.2, 9.2_
  
  - [ ]* 9.2 编写键盘模拟器单元测试
    - 测试 Unicode 字符输入
    - 测试换行符处理
    - _Requirements: 6.2_

- [x] 10. 全局快捷键管理器实现
  - [x] 10.1 实现 HotkeyManager 组件
    - 使用 Tauri global-hotkey 注册 Alt+V 和 Alt+C
    - 实现 `register`, `unregister`, `check_conflict` 方法
    - 实现快捷键事件监听循环
    - _Requirements: 2.1, 2.2, 2.4_
  
  - [x] 10.2 实现快捷键冲突检测
    - 实现冲突检测逻辑和用户提示
    - _Requirements: 2.3_
  
  - [ ]* 10.3 编写快捷键管理器单元测试
    - 测试快捷键注册和注销
    - 测试冲突检测逻辑
    - _Requirements: 2.3_

- [x] 11. 事件总线和 Tauri Commands
  - [x] 11.1 定义事件类型
    - 创建 `AppEvent` 枚举（StateChanged, RecordingStarted, TranscriptionComplete, Error 等）
    - 实现事件序列化和反序列化
    - _Requirements: 1.4, 3.2, 3.4, 4.3, 5.6, 14.1_
  
  - [x] 11.2 实现 Tauri Commands
    - 实现 `start_voice_mode` command
    - 实现 `start_text_mode` command
    - 实现 `confirm_translation` command
    - 实现 `cancel_translation` command
    - 实现 `get_config` 和 `update_config` commands
    - _Requirements: 3.1, 7.1, 8.4, 8.5, 13.5_
  
  - [ ]* 11.3 编写事件总线单元测试
    - 测试事件发送和接收
    - 测试事件序列化
    - _Requirements: 1.4_

- [x] 12. Checkpoint - 验证后端集成
  - 确保所有后端组件能够协同工作，询问用户是否有问题

- [x] 13. 前端 Island 组件实现
  - [x] 13.1 创建 Island 组件基础结构
    - 创建 React 组件和 TypeScript 接口
    - 实现毛玻璃效果样式（backdrop-filter: blur(20px)）
    - 实现屏幕顶部中央定位（200px × 60px，距顶部 20px）
    - _Requirements: 10.1, 10.2_
  
  - [x] 13.2 实现状态显示逻辑
    - 实现 idle, recording, thinking, error 状态切换
    - 实现状态图标和消息显示
    - _Requirements: 3.2, 3.4, 4.3, 4.4_
  
  - [x] 13.3 实现波形可视化动画
    - 使用 Framer Motion 实现实时波形动画
    - 订阅后端 WaveformUpdate 事件
    - _Requirements: 3.2, 10.5_
  
  - [x] 13.4 实现弹簧物理动画
    - 使用 Framer Motion 实现出现/消失动画（stiffness: 300, damping: 20）
    - 确保动画帧率 ≥ 60fps
    - _Requirements: 10.2, 10.6_

- [x] 14. 前端 Glow 组件实现
  - [x] 14.1 创建 Glow 组件基础结构
    - 创建 React 组件和 TypeScript 接口
    - 实现毛玻璃效果和屏幕中央定位（最大 600px × 400px）
    - _Requirements: 8.1, 10.3_
  
  - [x] 14.2 实现渐变流光边框动画
    - 使用 CSS 实现 1px 渐变流光边框
    - 实现 glow-rotate 动画（3s linear infinite）
    - _Requirements: 8.2, 10.3_
  
  - [x] 14.3 实现用户交互逻辑
    - 实现 Enter 键确认和 Esc 键取消
    - 实现 10 秒自动消失逻辑
    - 调用后端 `confirm_translation` 或 `cancel_translation` commands
    - _Requirements: 8.3, 8.4, 8.5, 8.6_
  
  - [x] 14.4 实现淡入淡出动画
    - 使用 Framer Motion 实现 300ms 淡入淡出
    - 确保动画帧率 ≥ 60fps
    - _Requirements: 10.4, 10.6_

- [x] 15. 前端设置界面实现
  - [x] 15.1 创建设置界面组件
    - 创建设置窗口布局
    - 实现快捷键自定义输入框
    - 实现翻译方向选择器
    - 实现 UI 透明度和动画速度滑块
    - _Requirements: 13.1, 13.2, 13.3_
  
  - [x] 15.2 实现配置保存和加载
    - 调用后端 `get_config` 和 `update_config` commands
    - 实现配置实时应用（无需重启）
    - _Requirements: 13.4, 13.5_

- [x] 16. 日志系统实现
  - [x] 16.1 实现日志记录
    - 配置 Rust 日志框架（tracing 或 log）
    - 实现日志文件写入和轮转（最大 50MB）
    - 实现跨平台日志文件路径解析
    - _Requirements: 14.2, 14.5_
  
  - [x] 16.2 实现日志查看界面
    - 在设置界面添加日志查看和导出功能
    - _Requirements: 14.3_
  
  - [ ]* 16.3 编写日志系统单元测试
    - 测试日志轮转逻辑
    - 测试日志文件大小限制
    - _Requirements: 14.5_

- [x] 17. 错误处理和恢复
  - [x] 17.1 实现全局错误处理
    - 捕获推理错误并发送 Error 事件
    - 在 Island/Glow 组件显示友好错误提示
    - _Requirements: 14.1, 4.4, 6.4, 7.3_
  
  - [x] 17.2 实现崩溃恢复
    - 实现应用状态持久化
    - 实现启动时的安全状态恢复
    - _Requirements: 14.4_

- [x] 18. 语音模式端到端集成
  - [x] 18.1 实现语音模式完整流程
    - 连接 HotkeyManager → AudioCapture → AsrEngine → TranslationEngine → KeyboardSimulator
    - 实现状态同步到 Island 组件
    - 处理不支持文本输入的窗口（复制到剪贴板）
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 4.1, 4.5, 5.6, 6.1, 6.2, 6.3, 6.4_
  
  - [ ]* 18.2 编写语音模式集成测试
    - 测试完整流程的端到端执行
    - 测试错误场景处理
    - _Requirements: 3.1, 6.1_

- [x] 19. 文本模式端到端集成
  - [x] 19.1 实现文本模式完整流程
    - 连接 HotkeyManager → ClipboardManager → TranslationEngine → Glow → KeyboardSimulator
    - 实现剪贴板备份和恢复
    - 实现原地替换逻辑
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 8.1, 8.4, 9.1, 9.2, 9.3_
  
  - [ ]* 19.2 编写文本模式集成测试
    - 测试完整流程的端到端执行
    - 测试剪贴板恢复逻辑
    - _Requirements: 7.1, 9.3_

- [x] 20. 系统托盘和窗口管理
  - [x] 20.1 实现系统托盘图标
    - 使用 Tauri 系统托盘 API 创建托盘图标
    - 实现托盘菜单（显示/隐藏、设置、退出）
    - _Requirements: 1.4_
  
  - [x] 20.2 实现窗口管理
    - 实现最小化到托盘功能
    - 实现 UI 资源释放逻辑
    - _Requirements: 15.5_

- [x] 21. 性能优化和资源管理
  - [x] 21.1 实现资源监控
    - 监控 CPU 和内存使用
    - 确保空闲状态 CPU < 1%，内存 < 500MB
    - 确保推理状态 CPU < 80%
    - _Requirements: 15.1, 15.2, 15.4_
  
  - [x] 21.2 实现 GPU 资源释放
    - 在推理完成后释放 GPU 资源（如使用 GPU）
    - _Requirements: 15.3_
  
  - [x] 21.3 优化启动时间
    - 确保应用在 5 秒内完成初始化
    - 实现模型懒加载或异步加载
    - _Requirements: 1.1, 1.5_

- [x] 22. 跨平台构建和打包
  - [x] 22.1 配置跨平台构建
    - 配置 Windows、macOS、Linux 构建脚本
    - 确保安装包大小 < 20MB（不含模型）
    - _Requirements: 12.1, 12.2, 12.3, 12.5_
  
  - [x] 22.2 创建模型下载指引
    - 实现模型缺失时的下载指引界面
    - 提供模型文件下载链接和安装说明
    - _Requirements: 1.3_

- [x] 23. Final Checkpoint - 完整测试
  - 在所有支持的平台上测试完整功能，确保所有测试通过，询问用户是否有问题

## Notes

- 任务标记 `*` 为可选任务，可跳过以加快 MVP 开发
- 每个任务都引用了具体的需求条款以确保可追溯性
- Checkpoint 任务确保增量验证和及时发现问题
- 推理引擎（任务 4-5）是核心依赖，应优先实现和测试
- 前端组件（任务 13-15）可与后端并行开发
- 端到端集成（任务 18-19）需要所有底层组件完成后进行
