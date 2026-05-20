# Verification script for Visp AI Translator setup (Windows)

Write-Host "🔍 Verifying Visp AI Translator setup..." -ForegroundColor Cyan
Write-Host ""

$allChecksPass = $true

# Check Node.js
Write-Host "Checking Node.js... " -NoNewline
if (Get-Command node -ErrorAction SilentlyContinue) {
    $nodeVersion = node --version
    Write-Host "✓ Found: $nodeVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Not found" -ForegroundColor Red
    $allChecksPass = $false
}

# Check npm
Write-Host "Checking npm... " -NoNewline
if (Get-Command npm -ErrorAction SilentlyContinue) {
    $npmVersion = npm --version
    Write-Host "✓ Found: v$npmVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Not found" -ForegroundColor Red
    $allChecksPass = $false
}

# Check Rust
Write-Host "Checking Rust... " -NoNewline
if (Get-Command rustc -ErrorAction SilentlyContinue) {
    $rustVersion = (rustc --version).Split(' ')[1]
    Write-Host "✓ Found: $rustVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Not found" -ForegroundColor Red
    $allChecksPass = $false
}

# Check Cargo
Write-Host "Checking Cargo... " -NoNewline
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    $cargoVersion = (cargo --version).Split(' ')[1]
    Write-Host "✓ Found: $cargoVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Not found" -ForegroundColor Red
    $allChecksPass = $false
}

Write-Host ""
Write-Host "📦 Checking project files..." -ForegroundColor Cyan

# Check package.json
Write-Host "Checking package.json... " -NoNewline
if (Test-Path "package.json") {
    Write-Host "✓" -ForegroundColor Green
} else {
    Write-Host "✗" -ForegroundColor Red
    $allChecksPass = $false
}

# Check Cargo.toml
Write-Host "Checking src-tauri\Cargo.toml... " -NoNewline
if (Test-Path "src-tauri\Cargo.toml") {
    Write-Host "✓" -ForegroundColor Green
} else {
    Write-Host "✗" -ForegroundColor Red
    $allChecksPass = $false
}

# Check tauri.conf.json
Write-Host "Checking src-tauri\tauri.conf.json... " -NoNewline
if (Test-Path "src-tauri\tauri.conf.json") {
    Write-Host "✓" -ForegroundColor Green
} else {
    Write-Host "✗" -ForegroundColor Red
    $allChecksPass = $false
}

# Check vite.config.ts
Write-Host "Checking vite.config.ts... " -NoNewline
if (Test-Path "vite.config.ts") {
    Write-Host "✓" -ForegroundColor Green
} else {
    Write-Host "✗" -ForegroundColor Red
    $allChecksPass = $false
}

# Check tailwind.config.js
Write-Host "Checking tailwind.config.js... " -NoNewline
if (Test-Path "tailwind.config.js") {
    Write-Host "✓" -ForegroundColor Green
} else {
    Write-Host "✗" -ForegroundColor Red
    $allChecksPass = $false
}

Write-Host ""
Write-Host "🔧 Checking Rust modules..." -ForegroundColor Cyan

# Check Rust source files
$rustFiles = @(
    "src-tauri\src\main.rs",
    "src-tauri\src\lib.rs",
    "src-tauri\src\config.rs",
    "src-tauri\src\state.rs",
    "src-tauri\src\commands.rs",
    "src-tauri\src\error.rs",
    "src-tauri\src\models.rs",
    "src-tauri\src\audio.rs",
    "src-tauri\src\asr.rs",
    "src-tauri\src\translation.rs",
    "src-tauri\src\clipboard.rs",
    "src-tauri\src\keyboard.rs",
    "src-tauri\src\hotkey.rs",
    "src-tauri\src\events.rs"
)

foreach ($file in $rustFiles) {
    Write-Host "Checking $file... " -NoNewline
    if (Test-Path $file) {
        Write-Host "✓" -ForegroundColor Green
    } else {
        Write-Host "⚠ Not found (may need implementation)" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "📱 Checking frontend files..." -ForegroundColor Cyan

# Check frontend source files
$frontendFiles = @(
    "src\main.tsx",
    "src\App.tsx",
    "src\index.css",
    "index.html"
)

foreach ($file in $frontendFiles) {
    Write-Host "Checking $file... " -NoNewline
    if (Test-Path $file) {
        Write-Host "✓" -ForegroundColor Green
    } else {
        Write-Host "✗" -ForegroundColor Red
        $allChecksPass = $false
    }
}

Write-Host ""
Write-Host "📋 Checking dependencies..." -ForegroundColor Cyan

# Check if node_modules exists
Write-Host "Checking node_modules... " -NoNewline
if (Test-Path "node_modules") {
    Write-Host "✓ Installed" -ForegroundColor Green
} else {
    Write-Host "⚠ Not installed (run 'npm install')" -ForegroundColor Yellow
}

Write-Host ""
if ($allChecksPass) {
    Write-Host "✅ Setup verification complete!" -ForegroundColor Green
} else {
    Write-Host "⚠ Setup verification completed with warnings" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Run 'npm install' if dependencies are not installed"
Write-Host "  2. Run 'npm run tauri:dev' to start development"
Write-Host "  3. See SETUP.md for detailed documentation"
