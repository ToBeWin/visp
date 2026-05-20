# Visp AI 翻译助手 - 项目设置指南

## 项目概述

Visp 是一款基于 Tauri 2.0 构建的全离线、零成本、极简、跨平台的 AI 翻译助手。本文档详细说明了项目的初始化配置和开发环境设置。

## 技术栈

### 后端 (Rust)
- **Tauri 2.2**: 跨平台桌面应用框架
- **candle 0.8**: Meta 的 Rust ML 推理框架
- **cpal 0.15**: 跨平台音频捕获库
- **arboard 3.4**: 跨平台剪贴板管理
- **enigo 0.2**: 跨平台键盘模拟
- **tokio 1.42**: 异步运行时
- **tracing**: 日志框架

### 前端 (TypeScript)
- **React 18.3**: UI 框架
- **TypeScript 5.7**: 类型安全
- **Vite 6.0**: 构建工具
- **Framer Motion 11.11**: 动画库
- **Tailwind CSS 3.4**: 样式框架

## 系统要求

### 开发环境
- **Node.js**: 18.0 或更高版本
- **Rust**: 1.70 或更高版本
- **操作系统**: Windows 10+, macOS 11+, 或 Linux (Ubuntu 20.04+)

### 运行时要求
- **内存**: 最少 4GB RAM (推荐 8GB)
- **存储**: 2GB 可用空间 (含模型文件)
- **CPU**: 支持 AVX2 指令集 (用于 ML 推理加速)

## 项目结构

```
visp/
├── src/                          # 前端源代码
│   ├── App.tsx                   # 主应用组件
│   ├── main.tsx                  # 入口文件
│   └── index.css                 # 全局样式
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs              # 应用入口
│   │   ├── lib.rs               # 库导出
│   │   ├── config.rs            # 配置管理 ✅
│   │   ├── state.rs             # 状态管理 ✅
│   │   ├── commands.rs          # Tauri Commands ✅
│   │   ├── error.rs             # 错误处理 ✅
│   │   ├── models.rs            # 模型加载器 ✅
│   │   ├── audio.rs             # 音频捕获 (待实现)
│   │   ├── asr.rs               # 语音识别 (待实现)
│   │   ├── translation.rs       # 翻译引擎 (待实现)
│   │   ├── clipboard.rs         # 剪贴板管理 (待实现)
│   │   ├── keyboard.rs          # 键盘模拟 (待实现)
│   │   ├── hotkey.rs            # 快捷键管理 (待实现)
│   │   └── events.rs            # 事件总线 (待实现)
│   ├── Cargo.toml               # Rust 依赖配置 ✅
│   ├── tauri.conf.json          # Tauri 配置 ✅
│   └── build.rs                 # 构建脚本 ✅
├── scripts/                      # 构建脚本
│   ├── build-all.sh             # Linux/macOS 构建脚本 ✅
│   └── build-all.ps1            # Windows 构建脚本 ✅
├── package.json                  # Node.js 依赖配置 ✅
├── tsconfig.json                 # TypeScript 配置 ✅
├── vite.config.ts                # Vite 配置 ✅
├── tailwind.config.js            # Tailwind 配置 ✅
├── eslint.config.js              # ESLint 9 Flat Config ✅
├── .eslintrc.cjs                 # 兼容占位配置（旧版工具）✅
└── .prettierrc                   # Prettier 配置 ✅
```

## 快速开始

### 1. 安装依赖

```bash
# 安装前端依赖
npm install

# 验证 Rust 工具链
cargo --version
rustc --version
```

### 2. 开发模式运行

```bash
# 启动开发服务器 (前端热重载 + Rust 后端)
npm run tauri:dev
```

### 3. 构建生产版本

```bash
# Linux/macOS
chmod +x scripts/build-all.sh
./scripts/build-all.sh

# Windows (PowerShell)
.\scripts\build-all.ps1

# 或使用 npm 脚本
npm run tauri:build
```

构建产物位置：
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Linux**: `src-tauri/target/release/bundle/deb/` 或 `appimage/`

## 配置详情

### Cargo.toml 依赖说明

```toml
[dependencies]
# Tauri 核心
tauri = { version = "2.2", features = ["tray-icon"] }
tauri-plugin-shell = "2.0"
tauri-plugin-global-shortcut = "2.0"

# ML 推理 (candle 框架)
candle-core = "0.8"              # 核心张量操作
candle-nn = "0.8"                # 神经网络层
candle-transformers = "0.8"      # Transformer 模型支持
tokenizers = "0.21"              # HuggingFace tokenizers

# 音频处理
cpal = "0.15"                    # 跨平台音频 I/O

# 系统集成
arboard = "3.4"                  # 剪贴板操作
enigo = "0.2"                    # 键盘/鼠标模拟

# 工具库
tokio = { version = "1.42", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "2.0"
tracing = "0.1"
directories = "5.0"
```

### package.json 依赖说明

```json
{
  "dependencies": {
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "framer-motion": "^11.11.17",        // 动画库
    "@tauri-apps/api": "^2.2.0",         // Tauri 前端 API
    "@tauri-apps/plugin-global-shortcut": "^2.0.1"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.2.0",
    "typescript": "^5.7.2",
    "vite": "^6.0.3",
    "tailwindcss": "^3.4.17",
    "prettier": "^3.4.2",
    "eslint": "^9.17.0"
  }
}
```

### TypeScript 配置

- **Target**: ES2020
- **Module**: ESNext (Vite bundler mode)
- **Strict Mode**: 启用
- **JSX**: react-jsx (React 17+ 新 JSX 转换)

### Tailwind CSS 配置

自定义动画：
- `glow-rotate`: 3 秒线性无限循环渐变动画 (用于 Glow 组件边框)

### 构建优化

Rust Release 配置 (`Cargo.toml`):
```toml
[profile.release]
panic = "abort"        # 减小二进制大小
codegen-units = 1      # 更好的优化
lto = true             # 链接时优化
opt-level = "z"        # 优化大小
strip = true           # 移除调试符号
```

预期安装包大小：
- **Windows**: ~15MB (MSI)
- **macOS**: ~18MB (DMG)
- **Linux**: ~16MB (DEB/AppImage)

## 已完成的配置

### ✅ 项目结构
- Tauri 2.0 项目初始化完成
- Rust 工作空间配置完成
- 前端 React + TypeScript + Vite 环境配置完成

### ✅ 依赖配置
- 所有必需的 Rust crates 已添加到 `Cargo.toml`
- 所有必需的 npm 包已添加到 `package.json`
- 版本锁定确保构建一致性

### ✅ 代码质量工具
- **ESLint 9**: Flat Config + TypeScript + React 规则
- **Prettier**: 代码格式化 (2 空格缩进, 单引号)
- **TypeScript**: 严格模式启用

### ✅ 构建配置
- Vite 开发服务器配置 (端口 1420)
- Tauri 构建配置 (跨平台)
- Release 优化配置 (LTO, 大小优化)

### ✅ 基础模块实现
- `config.rs`: 配置管理 (加载/保存/默认值)
- `state.rs`: 应用状态管理 (线程安全)
- `commands.rs`: Tauri Commands 骨架
- `error.rs`: 统一错误处理
- `models.rs`: 模型加载器和路径管理

### ✅ 跨平台支持
- Windows 构建脚本 (PowerShell)
- Linux/macOS 构建脚本 (Bash)
- 平台特定配置 (Tauri conf)

## 下一步开发任务

根据 `tasks.md`，接下来需要实现：

1. **任务 2**: 数据模型和配置管理 (部分完成)
2. **任务 3**: 模型加载器实现 (部分完成)
3. **任务 4**: ASR 引擎实现 (待开始)
4. **任务 5**: 翻译引擎实现 (待开始)
5. **任务 7-10**: 系统集成层 (音频、剪贴板、键盘、快捷键)
6. **任务 13-15**: 前端 UI 组件 (Island, Glow, Settings)
7. **任务 18-19**: 端到端集成测试

## 开发工作流

### 代码格式化
```bash
# 格式化前端代码
npm run format

# 检查格式
npm run format:check

# Rust 代码格式化
cargo fmt --manifest-path src-tauri/Cargo.toml
```

### 代码检查
```bash
# TypeScript 类型检查
npm run type-check

# ESLint 检查
npm run lint

# Rust 编译检查
cargo check --manifest-path src-tauri/Cargo.toml
```

### 调试
```bash
# 启动开发模式 (带日志)
RUST_LOG=info npm run tauri:dev

# 仅构建前端
npm run dev

# 仅检查 Rust 代码
cargo build --manifest-path src-tauri/Cargo.toml
```

## 模型文件

模型文件需要单独下载并放置在以下位置：

- **Windows**: `%APPDATA%/visp/translator/models/`
- **macOS**: `~/Library/Application Support/visp/translator/models/`
- **Linux**: `~/.local/share/visp/translator/models/`

所需模型：
1. `whisper-large-v3-turbo-q4_0.bin` (~800MB)
2. `qwen3.5-0.8b-instruct-q4_k_m.gguf` (~350MB)

模型下载脚本将在后续任务中实现。

## 常见问题

### Q: 构建失败，提示找不到 Rust 工具链
A: 运行 `rustup update` 更新 Rust 工具链到最新版本。

### Q: npm install 失败
A: 尝试清除缓存：`npm cache clean --force && npm install`

### Q: Tauri 开发模式启动慢
A: 首次启动需要编译 Rust 依赖，后续启动会快很多。使用 `cargo build` 预编译可加速。

### Q: 如何更改快捷键？
A: 编辑配置文件 `config.json` 中的 `hotkeys` 部分，或通过设置界面修改（待实现）。

## 贡献指南

1. 遵循现有代码风格 (Prettier + ESLint)
2. 提交前运行 `npm run lint` 和 `npm run type-check`
3. Rust 代码使用 `cargo fmt` 和 `cargo clippy`
4. 提交信息使用中文，格式：`类型: 简短描述`

## 许可证

待定

---

**项目状态**: 🚧 开发中 - 任务 1 已完成，基础配置就绪
