#!/bin/bash

# Visp AI Translator - 模型下载脚本
# 此脚本会下载 Whisper 和 Qwen 模型到正确的位置

set -e

echo "🚀 Visp AI Translator - 模型下载脚本"
echo "======================================"
echo ""

# 检测操作系统
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM=linux;;
    Darwin*)    PLATFORM=macos;;
    CYGWIN*)    PLATFORM=windows;;
    MINGW*)     PLATFORM=windows;;
    *)          PLATFORM="unknown";;
esac

echo "检测到操作系统: ${PLATFORM}"
echo ""

# 设置模型目录
if [ "$PLATFORM" = "macos" ]; then
    MODEL_DIR="$HOME/Library/Application Support/visp/translator/models"
elif [ "$PLATFORM" = "linux" ]; then
    MODEL_DIR="$HOME/.local/share/visp/translator/models"
elif [ "$PLATFORM" = "windows" ]; then
    MODEL_DIR="$APPDATA/visp/translator/models"
else
    echo "❌ 不支持的操作系统"
    exit 1
fi

echo "模型目录: $MODEL_DIR"
mkdir -p "$MODEL_DIR"
echo ""

# 检查是否已安装 huggingface-cli
if ! command -v huggingface-cli &> /dev/null; then
    echo "⚠️  未检测到 huggingface-cli"
    echo "正在安装 huggingface-hub..."
    pip3 install -U "huggingface_hub[cli]"
    echo ""
fi

# 下载 Whisper Large V3 Turbo (GGUF Q4_0 量化版本)
echo "📥 下载 Whisper Large V3 Turbo 模型..."
echo "   模型: ggerganov/whisper.cpp"
echo "   文件: ggml-large-v3-turbo-q4_0.bin (~800MB)"
echo ""

WHISPER_MODEL="$MODEL_DIR/whisper-large-v3-turbo-q4_0.bin"
if [ -f "$WHISPER_MODEL" ]; then
    echo "✅ Whisper 模型已存在，跳过下载"
else
    huggingface-cli download \
        ggerganov/whisper.cpp \
        ggml-large-v3-turbo-q4_0.bin \
        --local-dir "$MODEL_DIR" \
        --local-dir-use-symlinks False
    
    # 重命名文件
    mv "$MODEL_DIR/ggml-large-v3-turbo-q4_0.bin" "$WHISPER_MODEL"
    echo "✅ Whisper 模型下载完成"
fi
echo ""

# 下载 Whisper tokenizer
echo "📥 下载 Whisper tokenizer..."
WHISPER_TOKENIZER="$MODEL_DIR/whisper-tokenizer.json"
if [ -f "$WHISPER_TOKENIZER" ]; then
    echo "✅ Whisper tokenizer 已存在，跳过下载"
else
    huggingface-cli download \
        openai/whisper-large-v3-turbo \
        tokenizer.json \
        --local-dir "$MODEL_DIR/whisper-temp" \
        --local-dir-use-symlinks False
    
    mv "$MODEL_DIR/whisper-temp/tokenizer.json" "$WHISPER_TOKENIZER"
    rm -rf "$MODEL_DIR/whisper-temp"
    echo "✅ Whisper tokenizer 下载完成"
fi
echo ""

# 下载 Qwen2.5-0.5B-Instruct (GGUF Q4_K_M 量化版本)
echo "📥 下载 Qwen2.5-0.5B-Instruct 模型..."
echo "   模型: Qwen/Qwen2.5-0.5B-Instruct-GGUF"
echo "   文件: qwen2.5-0.5b-instruct-q4_k_m.gguf (~350MB)"
echo ""

QWEN_MODEL="$MODEL_DIR/qwen3.5-0.8b-instruct-q4_k_m.gguf"
if [ -f "$QWEN_MODEL" ]; then
    echo "✅ Qwen 模型已存在，跳过下载"
else
    # 当前实现使用 Qwen2.5-0.5B 的 GGUF 文件，并保留兼容文件名
    huggingface-cli download \
        Qwen/Qwen2.5-0.5B-Instruct-GGUF \
        qwen2.5-0.5b-instruct-q4_k_m.gguf \
        --local-dir "$MODEL_DIR" \
        --local-dir-use-symlinks False
    
    # 重命名文件
    mv "$MODEL_DIR/qwen2.5-0.5b-instruct-q4_k_m.gguf" "$QWEN_MODEL"
    echo "✅ Qwen 模型下载完成"
fi
echo ""

# 下载 Qwen tokenizer
echo "📥 下载 Qwen tokenizer..."
QWEN_TOKENIZER="$MODEL_DIR/qwen-tokenizer.json"
if [ -f "$QWEN_TOKENIZER" ]; then
    echo "✅ Qwen tokenizer 已存在，跳过下载"
else
    huggingface-cli download \
        Qwen/Qwen2.5-0.5B-Instruct \
        tokenizer.json \
        --local-dir "$MODEL_DIR/qwen-temp" \
        --local-dir-use-symlinks False
    
    mv "$MODEL_DIR/qwen-temp/tokenizer.json" "$QWEN_TOKENIZER"
    rm -rf "$MODEL_DIR/qwen-temp"
    echo "✅ Qwen tokenizer 下载完成"
fi
echo ""

# 显示模型信息
echo "======================================"
echo "✅ 所有模型下载完成！"
echo ""
echo "模型位置:"
echo "  Whisper: $WHISPER_MODEL"
echo "  Whisper Tokenizer: $WHISPER_TOKENIZER"
echo "  Qwen: $QWEN_MODEL"
echo "  Qwen Tokenizer: $QWEN_TOKENIZER"
echo ""
echo "总大小: ~1.2GB"
echo ""
echo "现在可以运行 Visp AI Translator 了！"
echo "======================================"
