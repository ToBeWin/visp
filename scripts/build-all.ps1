# Cross-platform build script for Visp AI Translator (Windows)

Write-Host "🚀 Building Visp AI Translator..." -ForegroundColor Green

# Check if Node.js is installed
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Node.js is not installed. Please install Node.js 18+ first." -ForegroundColor Red
    exit 1
}

# Check if Rust is installed
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust is not installed. Please install Rust 1.70+ first." -ForegroundColor Red
    exit 1
}

# Install frontend dependencies
Write-Host "📦 Installing frontend dependencies..." -ForegroundColor Cyan
npm install

# Build the application
Write-Host "🔨 Building application..." -ForegroundColor Cyan
npm run tauri:build

Write-Host "✅ Build completed successfully!" -ForegroundColor Green
Write-Host "📦 Build artifacts are in: src-tauri\target\release\bundle\" -ForegroundColor Yellow
