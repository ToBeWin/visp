# Model setup script for Visp AI Translator (Windows)

Write-Host "📥 Setting up models for Visp AI Translator..." -ForegroundColor Green

# Determine the models directory
$MODELS_DIR = "$env:APPDATA\visp\translator\models"

# Create models directory
New-Item -ItemType Directory -Force -Path $MODELS_DIR | Out-Null

Write-Host "📂 Models directory: $MODELS_DIR" -ForegroundColor Cyan
Write-Host ""
Write-Host "Please download the following model files and place them in the models directory:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Whisper Large V3 Turbo (Q4_0 quantized, ~800MB)" -ForegroundColor White
Write-Host "   File: whisper-large-v3-turbo-q4_0.bin" -ForegroundColor Gray
Write-Host "   Download: https://huggingface.co/ggerganov/whisper.cpp/tree/main" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Qwen2.5-0.5B Instruct (Q4_K_M quantized, ~350MB)" -ForegroundColor White
Write-Host "   File: qwen3.5-0.8b-instruct-q4_k_m.gguf" -ForegroundColor Gray
Write-Host "   Download: https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/tree/main" -ForegroundColor Gray
Write-Host ""
Write-Host "After downloading, place the files in: $MODELS_DIR" -ForegroundColor Yellow
