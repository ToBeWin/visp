#!/bin/bash
# Cross-platform build script for Visp AI Translator

set -e

echo "🚀 Building Visp AI Translator..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust 1.70+ first."
    exit 1
fi

# Install frontend dependencies
echo "📦 Installing frontend dependencies..."
npm install

# Build the application
echo "🔨 Building application..."
npm run tauri:build

echo "✅ Build completed successfully!"
echo "📦 Build artifacts are in: src-tauri/target/release/bundle/"
