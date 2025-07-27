#!/bin/bash
# Build script for Linux systems
# Run this script on a Linux machine to build the Linux binary

echo "🐧 Building M3U Splitter for Linux..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build the project
echo "📦 Building release binary..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Binary location: target/release/m3u-splitter"
    echo ""
    echo "🚀 To install globally:"
    echo "   sudo cp target/release/m3u-splitter /usr/local/bin/"
    echo ""
    echo "🧪 To test:"
    echo "   ./target/release/m3u-splitter --help"
else
    echo "❌ Build failed!"
    exit 1
fi
