# Visp AI 翻译助手 - 架构文档

## 系统架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                     用户界面层 (React)                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Island     │  │     Glow     │  │   Settings   │      │
│  │  状态显示组件  │  │   预览窗口    │  │   设置界面    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            ↕ Tauri IPC
┌─────────────────────────────────────────────────────────────┐
│                    业务逻辑层 (Rust)                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Commands   │  │  Event Bus   │  │    State     │      │
│  │ Tauri 命令处理 │  │   事件总线    │  │   状态管理    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            ↕
┌─────────────────────────────────────────────────────────────┐
│                    系统集成层 (Rust)                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Hotkey     │  │    Audio     │  │  Clipboard   │      │
│  │  快捷键管理    │  │   音频捕获    │  │  剪贴板管理   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │  Keyboard    │  │    Models    │                        │
│  │  键盘模拟     │  │  模型加载器   │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ↕
┌─────────────────────────────────────────────────────────────┐
│                    推理引擎层 (candle)                        │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │  ASR Engine  │  │ Translation  │                        │
│  │   Whisper    │  │    Qwen3.5   │                        │
│  │  语音识别引擎  │  │   翻译引擎    │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ↕
┌─────────────────────────────────────────────────────────────┐
│                    操作系统层                                 │
│     麦克风  |  剪贴板  |  键盘  |  活动窗口  |  文件系统       │
└─────────────────────────────────────────────────────────────┘
```

## 数据流

### 语音翻译模式 (Alt+V)

```
1. 用户按下 Alt+V
   ↓
2. HotkeyManager 触发 VoiceMode 事件
   ↓
3. AudioCapture 开始录音
   ↓ (实时)
4. Island 组件显示波形动画
   ↓
5. 用户松开 Alt+V
   ↓
6. AudioCapture 停止录音，返回音频数据
   ↓
7. ASR Engine (Whisper) 识别音频 → 文本
   ↓ (Island 显示 "Thinking")
8. Translation Engine (Qwen) 翻译文本
   ↓
9. KeyboardSimulator 模拟输入翻译结果
   ↓
10. Island 组件淡出消失
```

### 文本翻译模式 (Alt+C)

```
1. 用户选中文本并按下 Alt+C
   ↓
2. HotkeyManager 触发 TextMode 事件
   ↓
3. KeyboardSimulator 模拟 Ctrl+C 复制
   ↓
4. ClipboardManager 读取剪贴板内容
   ↓
5. Translation Engine (Qwen) 翻译文本
   ↓
6. Glow 组件显示翻译预览
   ↓
7. 用户按 Enter 确认 / Esc 取消
   ↓ (如果确认)
8. ClipboardManager 备份原剪贴板
   ↓
9. ClipboardManager 写入翻译结果
   ↓
10. KeyboardSimulator 模拟 Ctrl+V 粘贴
   ↓
11. ClipboardManager 恢复原剪贴板
   ↓
12. Glow 组件淡出消失
```

## 模块详解

### 1. 配置管理 (config.rs)

**职责**: 管理应用配置的加载、保存和默认值

**数据结构**:
```rust
AppConfig
├── HotkeyConfig (快捷键配置)
│   ├── voice_mode: "Alt+V"
│   └── text_mode: "Alt+C"
├── TranslationConfig (翻译配置)
│   ├── direction: Auto | ChineseToEnglish | EnglishToChinese
│   └── preserve_formatting: bool
├── UiConfig (UI 配置)
│   ├── opacity: 0.95
│   ├── animation_speed: 1.0
│   └── auto_dismiss_timeout: 10s
└── LoggingConfig (日志配置)
    ├── level: Error | Warn | Info | Debug
    └── max_file_size: 50MB
```

**配置文件位置**:
- Windows: `%APPDATA%/visp/ai-translator/config.json`
- macOS: `~/Library/Application Support/visp/ai-translator/config.json`
- Linux: `~/.config/visp/ai-translator/config.json`

### 2. 状态管理 (state.rs)

**职责**: 维护应用运行时状态

**核心字段**:
```rust
AppState {
    config: Arc<RwLock<AppConfig>>,              // 配置 (读写锁)
    event_sender: mpsc::UnboundedSender,         // 事件发送器
    current_mode: Arc<Mutex<Option<Mode>>>,      // 当前模式
}
```

**线程安全设计**:
- `Arc`: 多线程共享所有权
- `RwLock`: 读多写少的配置访问
- `Mutex`: 互斥访问当前模式
- `mpsc`: 异步事件通道

### 3. 命令处理 (commands.rs)

**职责**: 暴露 Tauri Commands 供前端调用

**命令列表**:
```rust
#[tauri::command]
async fn start_voice_mode(state: State<AppState>) -> Result<(), String>

#[tauri::command]
async fn start_text_mode(state: State<AppState>) -> Result<(), String>

#[tauri::command]
async fn confirm_translation(state: State<AppState>) -> Result<(), String>

#[tauri::command]
async fn cancel_translation(state: State<AppState>) -> Result<(), String>

#[tauri::command]
async fn get_config(state: State<AppState>) -> Result<AppConfig, String>

#[tauri::command]
async fn update_config(state: State<AppState>, new_config: AppConfig) -> Result<(), String>
```

### 4. 错误处理 (error.rs)

**职责**: 统一错误类型定义

**错误类型**:
```rust
VispError {
    ModelLoadError,      // 模型加载失败
    AudioCaptureError,   // 音频捕获失败
    AsrError,            // 语音识别失败
    TranslationError,    // 翻译失败
    ClipboardError,      // 剪贴板操作失败
    KeyboardError,       // 键盘模拟失败
    HotkeyError,         // 快捷键注册失败
    ConfigError,         // 配置错误
    IoError,             // IO 错误
    SerdeError,          // 序列化错误
}
```

### 5. 模型加载器 (models.rs)

**职责**: 管理 ML 模型文件的加载和验证

**核心方法**:
```rust
ModelLoader {
    check_models() -> ModelStatus           // 检查模型可用性
    get_asr_model_path() -> PathBuf        // 获取 ASR 模型路径
    get_translation_model_path() -> PathBuf // 获取翻译模型路径
    verify_model(path) -> Result<()>       // 验证模型完整性
}
```

**模型文件位置**:
- Windows: `%APPDATA%/visp/ai-translator/models/`
- macOS: `~/Library/Application Support/visp/ai-translator/models/`
- Linux: `~/.local/share/visp/ai-translator/models/`

**所需模型**:
1. `whisper-large-v3-turbo-q4_0.gguf` (~800MB)
2. `qwen3.5-0.8b-instruct-q4_k_m.gguf` (~500MB)

## 前端架构

### 组件层次

```
App (根组件)
├── Island (状态显示)
│   ├── 波形动画
│   ├── 状态图标
│   └── 消息文本
├── Glow (预览窗口)
│   ├── 翻译结果
│   ├── 流光边框
│   └── 操作提示
└── Settings (设置界面)
    ├── 快捷键配置
    ├── 翻译选项
    └── UI 调整
```

### 状态管理

使用 React Hooks:
- `useState`: 组件本地状态
- `useEffect`: 副作用和事件订阅
- Tauri Events: 后端事件监听

### 动画实现

使用 Framer Motion:
```typescript
// Island 弹簧动画
<motion.div
  initial={{ y: -100, opacity: 0 }}
  animate={{ y: 0, opacity: 1 }}
  exit={{ y: -100, opacity: 0 }}
  transition={{ type: "spring", stiffness: 300, damping: 20 }}
>

// Glow 淡入淡出
<motion.div
  initial={{ opacity: 0, scale: 0.95 }}
  animate={{ opacity: 1, scale: 1 }}
  exit={{ opacity: 0, scale: 0.95 }}
  transition={{ duration: 0.3 }}
>
```

## 性能目标

### 启动性能
- 应用启动时间: < 5 秒
- 模型加载时间: < 3 秒 (异步)
- 首次渲染时间: < 1 秒

### 运行时性能
- 空闲 CPU: < 1%
- 空闲内存: < 500MB
- 推理 CPU: < 80%
- 推理内存: < 2GB

### 响应时间
- 语音识别: < 3 秒
- 文本翻译: < 2 秒
- UI 动画: 60fps

### 安装包大小
- Windows: ~15MB
- macOS: ~18MB
- Linux: ~16MB
- 目标: < 20MB ✅

## 安全和隐私

### 数据处理
- ✅ 完全离线运行
- ✅ 无网络请求
- ✅ 无数据上传
- ✅ 本地模型推理

### 权限要求
- 麦克风访问 (语音模式)
- 剪贴板访问 (文本模式)
- 键盘模拟 (输入/替换)
- 全局快捷键 (触发功能)

## 开发路线图

### Phase 1: 基础设施 ✅ (当前)
- [x] 项目初始化
- [x] 依赖配置
- [x] 基础模块实现

### Phase 2: 推理引擎 (进行中)
- [ ] ASR 引擎实现
- [ ] 翻译引擎实现
- [ ] 模型加载优化

### Phase 3: 系统集成
- [ ] 音频捕获
- [ ] 剪贴板管理
- [ ] 键盘模拟
- [ ] 快捷键管理

### Phase 4: 用户界面
- [ ] Island 组件
- [ ] Glow 组件
- [ ] Settings 界面

### Phase 5: 集成测试
- [ ] 语音模式端到端
- [ ] 文本模式端到端
- [ ] 跨平台测试

### Phase 6: 优化和发布
- [ ] 性能优化
- [ ] 资源管理
- [ ] 打包和分发

## 技术决策记录

### 为什么选择 Tauri 而不是 Electron？
- **大小**: Tauri 安装包 ~15MB vs Electron ~100MB
- **性能**: 原生 Rust 后端，无 Node.js 开销
- **安全**: 更小的攻击面，更好的沙箱
- **资源**: 更低的内存和 CPU 占用

### 为什么选择 candle 而不是 PyTorch？
- **无 Python 依赖**: 纯 Rust 实现，简化部署
- **GGUF 支持**: 直接加载量化模型
- **性能**: 编译时优化，更快的推理
- **大小**: 无需打包 Python 运行时

### 为什么选择 Qwen3.5-0.8B？
- **大小**: 量化后仅 500MB，适合本地部署
- **性能**: CPU 推理 < 2 秒
- **质量**: 中英翻译质量优秀
- **开源**: MIT 许可证，可商用

### 为什么选择 Whisper-large-v3-turbo？
- **速度**: 比 large-v3 快 8 倍
- **质量**: 保持 large-v3 的识别准确度
- **大小**: Q4_0 量化后 ~800MB
- **语言**: 支持中英混合识别

## 依赖关系图

```
main.rs
  ├── config (配置管理)
  ├── state (状态管理)
  │   └── config
  ├── commands (命令处理)
  │   ├── state
  │   ├── audio
  │   ├── asr
  │   ├── translation
  │   ├── clipboard
  │   └── keyboard
  ├── hotkey (快捷键)
  │   └── events
  ├── models (模型加载)
  │   └── error
  └── events (事件总线)
      └── error

推理层依赖:
  asr → candle-transformers → candle-core
  translation → candle-transformers → candle-core
  audio → cpal
  clipboard → arboard
  keyboard → enigo
  hotkey → tauri-plugin-global-shortcut
```

## 构建流程

### 开发构建
```
npm run tauri:dev
  ↓
1. Vite 启动开发服务器 (localhost:1420)
  ↓
2. Cargo 编译 Rust 后端 (debug 模式)
  ↓
3. Tauri 启动应用窗口
  ↓
4. 前端热重载 (HMR) 启用
```

### 生产构建
```
npm run tauri:build
  ↓
1. TypeScript 编译检查
  ↓
2. Vite 构建前端 (dist/)
  ↓
3. Cargo 编译 Rust (release 模式)
  ↓
4. Tauri 打包应用
  ↓
5. 生成平台安装包
   ├── Windows: MSI
   ├── macOS: DMG
   └── Linux: DEB/AppImage
```

## 测试策略

### 单元测试
- Rust: `cargo test`
- TypeScript: Jest/Vitest (待配置)

### 集成测试
- 语音模式端到端流程
- 文本模式端到端流程
- 配置管理测试

### 性能测试
- 启动时间测试
- 推理速度测试
- 内存占用监控
- CPU 使用率监控

### 跨平台测试
- Windows 10/11
- macOS 11/12/13
- Ubuntu 20.04/22.04
- Fedora 35+

## 部署和分发

### 安装包命名
- Windows: `Visp_0.1.0_x64.msi`
- macOS: `Visp_0.1.0_x64.dmg`
- Linux: `visp_0.1.0_amd64.deb`

### 模型分发
- 模型文件单独下载
- 首次启动检测并提示下载
- 提供自动下载脚本

### 更新机制
- 待实现 (Tauri Updater 插件)

---

**文档版本**: 1.0  
**最后更新**: 任务 1 完成时
