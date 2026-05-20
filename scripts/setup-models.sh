#!/bin/bash
# Model setup script for Visp AI Translator

set -e

echo "📥 Setting up models for Visp AI Translator..."

# Determine the models directory based on OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    MODELS_DIR="$HOME/Library/Application Support/visp/translator/models"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    MODELS_DIR="$HOME/.local/share/visp/translator/models"
else
    echo "❌ Unsupported OS. Please use macOS or Linux."
    exit 1
fi

# Create models directory
mkdir -p "$MODELS_DIR"

echo "📂 Models directory: $MODELS_DIR"
echo ""
echo "Please download the following model files and place them in the models directory:"
echo ""
echo "1. Whisper Large V3 Turbo (Q4_0 quantized, ~800MB)"
echo "   File: whisper-large-v3-turbo-q4_0.bin"
echo "   Download: https://huggingface.co/ggerganov/whisper.cpp/tree/main"
echo ""
echo "2. Qwen2.5-0.5B Instruct (Q4_K_M quantized, ~350MB)"
echo "   File: qwen3.5-0.8b-instruct-q4_k_m.gguf"
echo "   Download: https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/tree/main"
echo ""
echo "After downloading, place the files in: $MODELS_DIR"
