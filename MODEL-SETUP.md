# Visp AI Translator - 模型设置指南

## 快速开始

Visp AI Translator 需要下载两个 AI 模型才能正常工作：

1. **Whisper Large V3 Turbo** - 用于语音识别 (~800MB)
2. **Qwen2.5-0.5B-Instruct GGUF** - 用于翻译 (~350MB)

总大小约 **1.2GB**，完全离线运行，无需网络连接。

## 自动下载（推荐）

### macOS / Linux

```bash
chmod +x scripts/download-models.sh
./scripts/download-models.sh
```

### Windows

```powershell
powershell -ExecutionPolicy Bypass -File scripts/download-models.ps1
```

## 手动下载

如果自动脚本失败，可以手动下载：

### 1. 安装 Hugging Face CLI

```bash
pip3 install -U "huggingface_hub[cli]"
```

### 2. 确定模型目录

- **macOS**: `~/Library/Application Support/visp/translator/models`
- **Linux**: `~/.local/share/visp/translator/models`
- **Windows**: `%APPDATA%\visp\translator\models`

### 3. 下载 Whisper 模型

```bash
# 创建模型目录
mkdir -p ~/Library/Application\ Support/visp/translator/models  # macOS
# 或
mkdir -p ~/.local/share/visp/translator/models  # Linux

# 下载 Whisper 模型
huggingface-cli download \
    ggerganov/whisper.cpp \
    ggml-large-v3-turbo-q4_0.bin \
    --local-dir ~/Library/Application\ Support/visp/translator/models \
    --local-dir-use-symlinks False

# 下载 Whisper tokenizer
huggingface-cli download \
    openai/whisper-large-v3-turbo \
    tokenizer.json \
    --local-dir ~/Library/Application\ Support/visp/translator/models \
    --local-dir-use-symlinks False
```

### 4. 下载 Qwen 模型

```bash
# 下载 Qwen 模型
huggingface-cli download \
    Qwen/Qwen2.5-0.5B-Instruct-GGUF \
    qwen2.5-0.5b-instruct-q4_k_m.gguf \
    --local-dir ~/Library/Application\ Support/visp/translator/models \
    --local-dir-use-symlinks False

# 下载 Qwen tokenizer
huggingface-cli download \
    Qwen/Qwen2.5-0.5B-Instruct \
    tokenizer.json \
    --local-dir ~/Library/Application\ Support/visp/translator/models \
    --local-dir-use-symlinks False
```

### 5. 重命名文件

确保文件名正确：

```bash
cd ~/Library/Application\ Support/visp/translator/models  # macOS

# Whisper
mv ggml-large-v3-turbo-q4_0.bin whisper-large-v3-turbo-q4_0.bin
mv tokenizer.json whisper-tokenizer.json

# Qwen
mv qwen2.5-0.5b-instruct-q4_k_m.gguf qwen3.5-0.8b-instruct-q4_k_m.gguf
# 如果有第二个 tokenizer.json，重命名为：
mv tokenizer.json qwen-tokenizer.json
```

## 验证安装

模型目录应该包含以下文件：

```
models/
├── whisper-large-v3-turbo-q4_0.bin      (~800MB)
├── whisper-tokenizer.json                (~2MB)
├── qwen3.5-0.8b-instruct-q4_k_m.gguf    (~350MB)
└── qwen-tokenizer.json                   (~2MB)
```

## 故障排除

### 问题：下载速度慢

使用镜像站点：

```bash
export HF_ENDPOINT=https://hf-mirror.com
# 然后重新运行下载脚本
```

### 问题：磁盘空间不足

确保至少有 **2GB** 可用空间（包括临时文件）。

### 问题：模型文件损坏

删除模型目录并重新下载：

```bash
rm -rf ~/Library/Application\ Support/visp/translator/models  # macOS
# 然后重新运行下载脚本
```

## 模型信息

### Whisper Large V3 Turbo

- **用途**: 语音识别（Speech-to-Text）
- **语言**: 支持中文和英文
- **精度**: Q4_0 量化（4-bit）
- **速度**: ~2-3秒/音频片段
- **来源**: OpenAI Whisper

### Qwen2.5-0.5B-Instruct

- **用途**: 文本翻译
- **语言**: 中英互译
- **精度**: Q4_K_M 量化（4-bit）
- **速度**: ~1-2秒/句子
- **来源**: 阿里云 Qwen 系列

## 隐私说明

所有模型文件存储在本地，**完全离线运行**，不会上传任何数据到云端。

## 许可证

- Whisper: MIT License
- Qwen: Apache 2.0 License

使用这些模型需要遵守相应的许可证条款。
