# Visp AI 翻译助手

<div align="center">

![Visp Logo](https://via.placeholder.com/200x200?text=Visp)

**全离线 · 零成本 · 极简 · 跨平台的 AI 翻译助手**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.2-blue)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-18.3-blue)](https://reactjs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange)](https://www.rust-lang.org/)

[功能特性](#-功能特性) • [快速开始](#-快速开始) • [技术栈](#-技术栈) • [文档](#-文档) • [贡献](#-贡献)

</div>

## 📖 简介

Visp 是一款专为开发者和办公族设计的 AI 翻译助手。通过集成 Whisper Large V3 Turbo 语音识别和 Qwen2.5-0.5B-Instruct GGUF 翻译模型，实现"语音即翻译"与"选中即替换"的无感交互体验。

### 为什么选择 Visp？

- 🔒 **隐私优先**: 所有数据处理完全离线，无需联网
- 💰 **零成本**: 无需 API 密钥，无需订阅费用
- ⚡ **极速响应**: 本地推理，无网络延迟
- 🎯 **极简交互**: 两个快捷键，一键翻译
- 🌍 **跨平台**: 支持 Windows、macOS、Linux
- 📦 **轻量级**: 安装包 < 20MB（不含模型）

## ✨ 功能特性

### 语音翻译模式 (Alt+V)

长按 Alt+V 开始录音，松开后自动识别并翻译，结果直接输入到当前光标位置。

```
按下 Alt+V → 说话 → 松开 → 自动翻译并输入
```

### 文本翻译模式 (Alt+C)

选中文本后按 Alt+C，自动翻译并显示预览，按 Enter 确认替换。

```
选中文本 → Alt+C → 预览翻译 → Enter 替换
```

### 智能特性

- 🧠 **自动语言检测**: 智能识别中英文
- 📝 **格式保留**: 保持原文换行和格式
- 💻 **代码友好**: 保留代码片段和技术术语
- 🎨 **精美 UI**: 毛玻璃效果 + 流光动画

## 🚀 快速开始

### 系统要求

- **操作系统**: Windows 10+, macOS 11+, 或 Linux (Ubuntu 20.04+)
- **内存**: 最少 4GB RAM (推荐 8GB)
- **存储**: 2GB 可用空间 (含模型文件)

### 安装

#### 方式 1: 下载预编译版本

从 [Releases](https://github.com/ToBeWin/visp/releases) 页面下载对应平台的安装包：

- **Windows**: `Visp_x64.msi`
- **macOS**: `Visp_x64.dmg`
- **Linux**: `visp_amd64.deb` 或 `.AppImage`

#### 方式 2: 从源码构建

```bash
# 1. 克隆仓库
git clone https://github.com/ToBeWin/visp.git
cd visp

# 2. 安装依赖
npm install

# 3. 构建应用
npm run tauri:build
```

### 模型文件

首次运行需要下载模型文件：

1. **Whisper-large-v3-turbo** (~800MB)
   - 下载: [HuggingFace](https://huggingface.co/ggerganov/whisper.cpp)
   - 放置到: `~/.local/share/visp/translator/models/` (Linux)
   - 或: `~/Library/Application Support/visp/translator/models/` (macOS)
   - 或: `%APPDATA%/visp/translator/models/` (Windows)

2. **Qwen2.5-0.5B-Instruct GGUF** (~350MB)
   - 下载: [HuggingFace](https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF)
   - 当前兼容文件名: `qwen3.5-0.8b-instruct-q4_k_m.gguf`
   - 放置到同一目录

### 使用

1. 启动 Visp
2. 按 Alt+V 开始语音翻译
3. 或选中文本后按 Alt+C 进行文本翻译

## 🛠️ 技术栈

### 后端 (Rust)

- **Tauri 2.2**: 跨平台应用框架
- **candle 0.8**: Meta 的 Rust ML 推理框架
- **cpal 0.15**: 跨平台音频捕获
- **arboard 3.4**: 跨平台剪贴板管理
- **enigo 0.2**: 跨平台键盘模拟
- **tokio 1.42**: 异步运行时

### 前端 (TypeScript)

- **React 18.3**: UI 框架
- **TypeScript 5.7**: 类型安全
- **Vite 6.0**: 构建工具
- **Framer Motion 11**: 动画库
- **Tailwind CSS 3.4**: 样式框架

### ML 模型

- **Whisper-large-v3-turbo**: 语音识别 (OpenAI)
- **Qwen2.5-0.5B-Instruct GGUF**: 文本翻译 (Alibaba)

## 📚 文档

- [设置指南](SETUP.md) - 详细的开发环境配置
- [快速开始](QUICK-START.md) - 常用命令和工作流
- [架构文档](ARCHITECTURE.md) - 系统架构和设计
- [开发进度](PROGRESS.md) - 项目进度和路线图
- [项目总结](FINAL-SUMMARY.md) - 完整的项目总结

## 🏗️ 开发

### 开发模式

```bash
# 启动开发服务器 (前端热重载 + Rust 后端)
npm run tauri:dev
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

### 代码格式化

```bash
# 格式化前端代码
npm run format

# 格式化 Rust 代码
cargo fmt --manifest-path src-tauri/Cargo.toml
```

## 🤝 贡献

欢迎贡献！请查看 [贡献指南](CONTRIBUTING.md) 了解详情。

### 贡献者

感谢所有贡献者的付出！

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台应用框架
- [candle](https://github.com/huggingface/candle) - Rust ML 框架
- [Whisper](https://github.com/openai/whisper) - 语音识别模型
- [Qwen](https://github.com/QwenLM/Qwen) - 大语言模型
- [React](https://reactjs.org/) - UI 框架
- [Framer Motion](https://www.framer.com/motion/) - 动画库

## 📞 联系方式

- **问题反馈**: [GitHub Issues](https://github.com/ToBeWin/visp/issues)
- **功能建议**: [GitHub Discussions](https://github.com/ToBeWin/visp/discussions)
- **官网**: [visp.live](https://visp.live)
- **邮箱**: support@visp.live

## ⭐ Star History

如果这个项目对你有帮助，请给我们一个 Star！

---

<div align="center">

Made with ❤️ by Visp Team

[官网](https://visp.live) • [GitHub](https://github.com/ToBeWin/visp) • [文档](https://visp.live/docs)

</div>
