# M3U Splitter - Cross-Platform Builds

This directory contains pre-built binaries for different platforms.

## Available Builds

### macOS

- **`m3u-splitter-macos-universal`** - Universal binary that works on both Intel and Apple Silicon Macs
- **`m3u-splitter-macos-arm64`** - Apple Silicon (M1/M2/M3) only
- **`m3u-splitter-macos-x64`** - Intel Macs only

### Linux

_Linux builds are not included due to cross-compilation toolchain limitations. To build for Linux, run the following on a Linux system:_

```bash
cargo build --release
```

### Windows

_Windows builds are not included due to cross-compilation toolchain limitations. To build for Windows:_

**Option 1: Build on Windows**

1. Install Rust: https://rustup.rs/
2. Clone this repository
3. Build the project:

```cmd
cargo build --release
```

The executable will be created at `target\release\m3u-splitter.exe`

**Option 2: Cross-compile from Linux/macOS**
Install the Windows target and MinGW-w64 toolchain:

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW-w64 (Ubuntu/Debian)
sudo apt install gcc-mingw-w64-x86-64

# Or on macOS with Homebrew
brew install mingw-w64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

All binaries have the same command-line interface:

```bash
# Basic usage
./m3u-splitter-[platform] -i input.m3u -o output_directory

# Show help
./m3u-splitter-[platform] --help
```

### Examples

**macOS/Linux:**

```bash
./m3u-splitter-macos-universal -i my_playlist.m3u -o ./split_files
```

**Windows (if built locally):**

```cmd
target\release\m3u-splitter.exe -i my_playlist.m3u -o ./split_files
```

## File Sizes

| Platform        | File Size | Architecture   |
| --------------- | --------- | -------------- |
| macOS Universal | ~2.8 MB   | x86_64 + ARM64 |
| macOS ARM64     | ~2.8 MB   | ARM64 only     |
| macOS x64       | ~2.9 MB   | x86_64 only    |

## Notes

- All binaries are optimized release builds
- The universal macOS binary contains both Intel and Apple Silicon code
- For maximum performance on macOS, use the architecture-specific binary for your system
- Windows users should build locally using the instructions above
