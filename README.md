# M3U Splitter

Inspired by https://github.com/lakeconstance78/iptv-tools

## - `-i, --input <INPUT>`: Input M3U file to split (required)

- `-o, --output <OUTPUT>`: Output directory for split files (optional, defaults to `./output`)
- `-h, --help`: Show help messagecription

This program splits M3U playlist files into separate files organized by group title.

## Features

- Reads M3U playlist files with channel information
- Extracts group titles and channel names
- Creates separate M3U files for each group
- Automatically adds `#EXTM3U` header if missing
- Sanitizes filenames by replacing invalid characters
- Uses title case for group names and filenames

## Requirements

- **Pre-built Binary**: No additional requirements - the pre-built binaries are self-contained
- **Building from Source**: Rust (with cargo) and dependencies: `regex` and `clap` crates

## Binary Information

Pre-built binaries are available in the `releases/` directory:

- **macOS Universal**: `m3u-splitter-macos-universal` (~2.8MB, works on Intel and Apple Silicon)
- **macOS ARM64**: `m3u-splitter-macos-arm64` (~2.8MB, Apple Silicon only)
- **macOS x64**: `m3u-splitter-macos-x64` (~2.9MB, Intel only)
- **Windows**: Build instructions provided in `releases/README.md`
- **Linux**: Build instructions provided in `releases/README.md`

All binaries are self-contained executables with no external dependencies.

## Usage

### Using Pre-built Binaries

```bash
# macOS - Download or copy the appropriate binary to your desired location
# Make it executable (if needed)
chmod +x m3u-splitter-macos-universal

# Split M3U file with output directory
./m3u-splitter-macos-universal -i input.m3u -o output_folder

# Split to current directory (default: ./output)
./m3u-splitter-macos-universal -i input.m3u

# Show help
./m3u-splitter-macos-universal --help
```

For Windows and Linux users, see build instructions in `releases/README.md`.

### Building from Source

```bash
# Build the program
cargo build --release

# Run tests
cargo test

# Run with input file and output directory
cargo run -- -i input.m3u -o output_folder

# Run with just input file (outputs to current directory)
cargo run -- -i input.m3u

# Or run the compiled binary directly
./target/release/m3u-splitter -i input.m3u -o output_folder

# Show help
cargo run -- --help
```

### Command Line Options

- `-i, --input <INPUT>`: Input M3U file to split (required)
- `-o, --output <OUTPUT>`: Output directory for split files (optional, defaults to current directory)
- `-h, --help`: Show help message
- `-V, --version`: Show version information

## Input Format

The program expects M3U files with the following format:

```
#EXTM3U
#EXTINF:-1 group-title="Sports",ESPN
https://example.com/espn.m3u8
#EXTINF:-1 group-title="News",CNN
https://example.com/cnn.m3u8
```

## Output

- Creates separate `.m3u` files for each group in the specified output directory
- Files are named using the group title (e.g., `Sports.m3u`, `News.m3u`)
- Invalid filename characters are replaced with underscores
- Empty groups are saved to `m3u_emptygroup.m3u`
- Automatically creates the output directory if it doesn't exist

## Changes from Python Version

- Uses Rust's `regex` crate for pattern matching
- Uses `clap` for command-line argument parsing
- Improved error handling with `Result` types
- More explicit memory management
- Compile-time safety guarantees

## Testing

The project includes comprehensive tests:

### Unit Tests

- **`to_title_case`** function testing
- **`sanitize_filename`** function testing
- **`process_m3u_content`** core logic testing
- **Empty group handling** testing
- **#EXTM3U header insertion** testing

### Integration Tests

- **CLI integration** testing with real files
- **Help command** testing
- **Version command** testing

### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run tests with output
cargo test -- --nocapture
```

All tests use temporary directories and files to avoid affecting the file system.
