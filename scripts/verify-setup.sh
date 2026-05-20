#!/bin/bash
# Verification script for Visp AI Translator setup

set -e

echo "🔍 Verifying Visp AI Translator setup..."
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check Node.js
echo -n "Checking Node.js... "
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo -e "${GREEN}✓${NC} Found: $NODE_VERSION"
else
    echo -e "${RED}✗${NC} Not found"
    exit 1
fi

# Check npm
echo -n "Checking npm... "
if command -v npm &> /dev/null; then
    NPM_VERSION=$(npm --version)
    echo -e "${GREEN}✓${NC} Found: v$NPM_VERSION"
else
    echo -e "${RED}✗${NC} Not found"
    exit 1
fi

# Check Rust
echo -n "Checking Rust... "
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    echo -e "${GREEN}✓${NC} Found: $RUST_VERSION"
else
    echo -e "${RED}✗${NC} Not found"
    exit 1
fi

# Check Cargo
echo -n "Checking Cargo... "
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version | cut -d' ' -f2)
    echo -e "${GREEN}✓${NC} Found: $CARGO_VERSION"
else
    echo -e "${RED}✗${NC} Not found"
    exit 1
fi

echo ""
echo "📦 Checking project files..."

# Check package.json
echo -n "Checking package.json... "
if [ -f "package.json" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    exit 1
fi

# Check Cargo.toml
echo -n "Checking src-tauri/Cargo.toml... "
if [ -f "src-tauri/Cargo.toml" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    exit 1
fi

# Check tauri.conf.json
echo -n "Checking src-tauri/tauri.conf.json... "
if [ -f "src-tauri/tauri.conf.json" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    exit 1
fi

# Check vite.config.ts
echo -n "Checking vite.config.ts... "
if [ -f "vite.config.ts" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    exit 1
fi

# Check tailwind.config.js
echo -n "Checking tailwind.config.js... "
if [ -f "tailwind.config.js" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    exit 1
fi

echo ""
echo "🔧 Checking Rust modules..."

# Check Rust source files
RUST_FILES=(
    "src-tauri/src/main.rs"
    "src-tauri/src/lib.rs"
    "src-tauri/src/config.rs"
    "src-tauri/src/state.rs"
    "src-tauri/src/commands.rs"
    "src-tauri/src/error.rs"
    "src-tauri/src/models.rs"
    "src-tauri/src/audio.rs"
    "src-tauri/src/asr.rs"
    "src-tauri/src/translation.rs"
    "src-tauri/src/clipboard.rs"
    "src-tauri/src/keyboard.rs"
    "src-tauri/src/hotkey.rs"
    "src-tauri/src/events.rs"
)

for file in "${RUST_FILES[@]}"; do
    echo -n "Checking $file... "
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠${NC} Not found (may need implementation)"
    fi
done

echo ""
echo "📱 Checking frontend files..."

# Check frontend source files
FRONTEND_FILES=(
    "src/main.tsx"
    "src/App.tsx"
    "src/index.css"
    "index.html"
)

for file in "${FRONTEND_FILES[@]}"; do
    echo -n "Checking $file... "
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${RED}✗${NC}"
        exit 1
    fi
done

echo ""
echo "📋 Checking dependencies..."

# Check if node_modules exists
echo -n "Checking node_modules... "
if [ -d "node_modules" ]; then
    echo -e "${GREEN}✓${NC} Installed"
else
    echo -e "${YELLOW}⚠${NC} Not installed (run 'npm install')"
fi

echo ""
echo -e "${GREEN}✅ Setup verification complete!${NC}"
echo ""
echo "Next steps:"
echo "  1. Run 'npm install' if dependencies are not installed"
echo "  2. Run 'npm run tauri:dev' to start development"
echo "  3. See SETUP.md for detailed documentation"
