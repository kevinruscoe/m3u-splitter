# M3U Splitter - Cross-Platform Builds

This directory contains pre-built binaries for different platforms.

## Available Builds

### macOS

- **`m3u-splitter-macos-universal`** - Universal binary that works on both Intel and Apple Silicon Macs
- **`m3u-splitter-macos-arm64`** - Apple Silicon (M1/M2/M3) only
- **`m3u-splitter-macos-x64`** - Intel Macs only

### Windows

- **`m3u-splitter-windows-x64.exe`** - Windows 64-bit executable

### Linux

_Linux builds are not included due to cross-compilation toolchain limitations. To build for Linux, run the following on a Linux system:_

```bash
cargo build --release
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

**Windows:**

```cmd
m3u-splitter-windows-x64.exe -i my_playlist.m3u -o ./split_files
```

## File Sizes

| Platform        | File Size | Architecture   |
| --------------- | --------- | -------------- |
| macOS Universal | ~5.7 MB   | x86_64 + ARM64 |
| macOS ARM64     | ~2.8 MB   | ARM64 only     |
| macOS x64       | ~2.9 MB   | x86_64 only    |
| Windows x64     | ~4.9 MB   | x86_64         |

## Notes

- All binaries are optimized release builds
- The universal macOS binary contains both Intel and Apple Silicon code
- Windows binary was built with MinGW-w64 toolchain
- For maximum performance on macOS, use the architecture-specific binary for your system
