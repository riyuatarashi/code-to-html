#!/bin/bash

# Rust Code Converter Setup Script
# This script helps you set up the project for first-time use

set -e

echo "🦀 Rust Code Converter Setup"
echo "============================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "Please install Rust from: https://rustup.rs/"
    echo "Then run this script again."
    exit 1
fi

echo "✅ Rust is installed: $(rustc --version)"

# Check if Node.js is installed (required for Torchlight CLI)
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed!"
    echo "Node.js is required for the Torchlight CLI."
    echo "Please install Node.js from: https://nodejs.org/"
    echo "Then run this script again."
    exit 1
fi

echo "✅ Node.js is installed: $(node --version)"

# Check if npm is available
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not available!"
    echo "npm is required to install the Torchlight CLI."
    exit 1
fi

echo "✅ npm is available: $(npm --version)"

# Install Torchlight CLI globally
echo ""
echo "📦 Installing Torchlight CLI..."
if npm list -g @torchlight-api/torchlight-cli &> /dev/null; then
    echo "✅ Torchlight CLI is already installed"
else
    echo "Installing @torchlight-api/torchlight-cli globally..."
    npm install -g @torchlight-api/torchlight-cli
    echo "✅ Torchlight CLI installed successfully"
fi

# Build the Rust project
echo ""
echo "🔨 Building Rust project..."
cargo build --release
echo "✅ Project built successfully"

# Create .env file if it doesn't exist
if [ ! -f ".env" ]; then
    echo ""
    echo "📄 Creating .env file from template..."
    cp .env.example .env
    echo "✅ Created .env file"
    echo ""
    echo "🔧 IMPORTANT: Edit the .env file and add your Torchlight token!"
    echo "   1. Get a free token at: https://torchlight.dev"
    echo "   2. Edit .env and replace 'your_torchlight_token_here' with your actual token"
    echo "   3. Optionally customize other settings"
else
    echo "✅ .env file already exists"
fi

# Create output and cache directories
echo ""
echo "📁 Creating directories..."
mkdir -p output cache
echo "✅ Created output and cache directories"

# Show usage instructions
echo ""
echo "🎉 Setup complete!"
echo ""
echo "Next steps:"
echo "1. Edit the .env file and add your Torchlight token"
echo "2. Test the converter:"
echo "   ./target/release/rust-code-converter sample.php"
echo "   ./target/release/rust-code-converter sample.rs"
echo ""
echo "Usage examples:"
echo "   # Convert a single file"
echo "   ./target/release/rust-code-converter path/to/your/file.js"
echo ""
echo "   # Convert all files in a directory"
echo "   ./target/release/rust-code-converter path/to/your/project/"
echo ""
echo "   # Remove empty lines during conversion"
echo "   ./target/release/rust-code-converter --no-empty path/to/file.py"
echo ""
echo "   # Using environment variables"
echo "   TORCHLIGHT_THEME=github-dark ./target/release/rust-code-converter file.go"
echo ""
echo "For more help, run:"
echo "   ./target/release/rust-code-converter --help"
echo ""
echo "Happy coding! 🚀"
