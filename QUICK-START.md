# Visp AI 翻译助手 - 快速开始

## 一键验证设置

```bash
# Linux/macOS
./scripts/verify-setup.sh

# Windows (PowerShell)
.\scripts\verify-setup.ps1
```

## 开发模式

```bash
# 1. 安装依赖 (首次运行)
npm install

# 2. 启动开发服务器
npm run tauri:dev
```

## 常用命令

### 开发
```bash
npm run dev              # 仅启动前端开发服务器
npm run tauri:dev        # 启动 Tauri 开发模式 (前端 + 后端)
```

### 代码检查
```bash
npm run type-check       # TypeScript 类型检查
npm run lint             # ESLint 检查
npm run lint:fix         # 自动修复 ESLint 问题
npm run format           # Prettier 格式化
npm run format:check     # 检查格式
```

### 构建
```bash
npm run build            # 构建前端
npm run tauri:build      # 构建完整应用

# 或使用平台脚本
./scripts/build-all.sh   # Linux/macOS
.\scripts\build-all.ps1  # Windows
```

### Rust 开发
```bash
# 检查编译
cargo check --manifest-path src-tauri/Cargo.toml

# 格式化代码
cargo fmt --manifest-path src-tauri/Cargo.toml

# Clippy 检查
cargo clippy --manifest-path src-tauri/Cargo.toml

# 运行测试
cargo test --manifest-path src-tauri/Cargo.toml
```

## 项目结构速览

```
visp/
├── src/                 # React 前端
├── src-tauri/          # Rust 后端
│   ├── src/            # Rust 源代码
│   └── Cargo.toml      # Rust 依赖
├── scripts/            # 构建脚本
└── package.json        # Node.js 依赖
```

## 核心技术

- **后端**: Tauri 2.0 + Rust + candle ML
- **前端**: React 18 + TypeScript + Vite
- **样式**: Tailwind CSS + Framer Motion
- **系统**: cpal (音频) + arboard (剪贴板) + enigo (键盘)

## 快捷键

- **Alt+V**: 语音翻译模式
- **Alt+C**: 文本翻译模式

## 需要帮助？

查看 `SETUP.md` 获取详细文档。
