# Visp AI Translator - 模型下载脚本 (Windows)
# 此脚本会下载 Whisper 和 Qwen 模型到正确的位置

$ErrorActionPreference = "Stop"

Write-Host "🚀 Visp AI Translator - 模型下载脚本" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""

# 设置模型目录
$MODEL_DIR = "$env:APPDATA\visp\translator\models"
Write-Host "模型目录: $MODEL_DIR"
New-Item -ItemType Directory -Force -Path $MODEL_DIR | Out-Null
Write-Host ""

# 检查是否已安装 huggingface-cli
$hfcli = Get-Command huggingface-cli -ErrorAction SilentlyContinue
if (-not $hfcli) {
    Write-Host "⚠️  未检测到 huggingface-cli" -ForegroundColor Yellow
    Write-Host "正在安装 huggingface-hub..."
    pip3 install -U "huggingface_hub[cli]"
    Write-Host ""
}

# 下载 Whisper Large V3 Turbo
Write-Host "📥 下载 Whisper Large V3 Turbo 模型..." -ForegroundColor Green
Write-Host "   模型: ggerganov/whisper.cpp"
Write-Host "   文件: ggml-large-v3-turbo-q4_0.bin (~800MB)"
Write-Host ""

$WHISPER_MODEL = "$MODEL_DIR\whisper-large-v3-turbo-q4_0.bin"
if (Test-Path $WHISPER_MODEL) {
    Write-Host "✅ Whisper 模型已存在，跳过下载" -ForegroundColor Green
} else {
    huggingface-cli download `
        ggerganov/whisper.cpp `
        ggml-large-v3-turbo-q4_0.bin `
        --local-dir $MODEL_DIR `
        --local-dir-use-symlinks False
    
    Move-Item "$MODEL_DIR\ggml-large-v3-turbo-q4_0.bin" $WHISPER_MODEL -Force
    Write-Host "✅ Whisper 模型下载完成" -ForegroundColor Green
}
Write-Host ""

# 下载 Whisper tokenizer
Write-Host "📥 下载 Whisper tokenizer..." -ForegroundColor Green
$WHISPER_TOKENIZER = "$MODEL_DIR\whisper-tokenizer.json"
if (Test-Path $WHISPER_TOKENIZER) {
    Write-Host "✅ Whisper tokenizer 已存在，跳过下载" -ForegroundColor Green
} else {
    huggingface-cli download `
        openai/whisper-large-v3-turbo `
        tokenizer.json `
        --local-dir "$MODEL_DIR\whisper-temp" `
        --local-dir-use-symlinks False
    
    Move-Item "$MODEL_DIR\whisper-temp\tokenizer.json" $WHISPER_TOKENIZER -Force
    Remove-Item "$MODEL_DIR\whisper-temp" -Recurse -Force
    Write-Host "✅ Whisper tokenizer 下载完成" -ForegroundColor Green
}
Write-Host ""

# 下载 Qwen2.5-0.5B-Instruct
Write-Host "📥 下载 Qwen2.5-0.5B-Instruct 模型..." -ForegroundColor Green
Write-Host "   模型: Qwen/Qwen2.5-0.5B-Instruct-GGUF"
Write-Host "   文件: qwen2.5-0.5b-instruct-q4_k_m.gguf (~350MB)"
Write-Host ""

$QWEN_MODEL = "$MODEL_DIR\qwen3.5-0.8b-instruct-q4_k_m.gguf"
if (Test-Path $QWEN_MODEL) {
    Write-Host "✅ Qwen 模型已存在，跳过下载" -ForegroundColor Green
} else {
    # 当前实现使用 Qwen2.5-0.5B 的 GGUF 文件，并保留兼容文件名
    huggingface-cli download `
        Qwen/Qwen2.5-0.5B-Instruct-GGUF `
        qwen2.5-0.5b-instruct-q4_k_m.gguf `
        --local-dir $MODEL_DIR `
        --local-dir-use-symlinks False
    
    Move-Item "$MODEL_DIR\qwen2.5-0.5b-instruct-q4_k_m.gguf" $QWEN_MODEL -Force
    Write-Host "✅ Qwen 模型下载完成" -ForegroundColor Green
}
Write-Host ""

# 下载 Qwen tokenizer
Write-Host "📥 下载 Qwen tokenizer..." -ForegroundColor Green
$QWEN_TOKENIZER = "$MODEL_DIR\qwen-tokenizer.json"
if (Test-Path $QWEN_TOKENIZER) {
    Write-Host "✅ Qwen tokenizer 已存在，跳过下载" -ForegroundColor Green
} else {
    huggingface-cli download `
        Qwen/Qwen2.5-0.5B-Instruct `
        tokenizer.json `
        --local-dir "$MODEL_DIR\qwen-temp" `
        --local-dir-use-symlinks False
    
    Move-Item "$MODEL_DIR\qwen-temp\tokenizer.json" $QWEN_TOKENIZER -Force
    Remove-Item "$MODEL_DIR\qwen-temp" -Recurse -Force
    Write-Host "✅ Qwen tokenizer 下载完成" -ForegroundColor Green
}
Write-Host ""

# 显示模型信息
Write-Host "======================================" -ForegroundColor Cyan
Write-Host "✅ 所有模型下载完成！" -ForegroundColor Green
Write-Host ""
Write-Host "模型位置:"
Write-Host "  Whisper: $WHISPER_MODEL"
Write-Host "  Whisper Tokenizer: $WHISPER_TOKENIZER"
Write-Host "  Qwen: $QWEN_MODEL"
Write-Host "  Qwen Tokenizer: $QWEN_TOKENIZER"
Write-Host ""
Write-Host "总大小: ~1.2GB"
Write-Host ""
Write-Host "现在可以运行 Visp AI Translator 了！"
Write-Host "======================================" -ForegroundColor Cyan
