# 🦀 Rust Universal Code to HTML Converter

[![Testing](https://github.com/riyuatarashi/code-to-html/actions/workflows/ci.yml/badge.svg)](https://github.com/riyuatarashi/code-to-html/actions/workflows/ci.yml)

A high-performance Rust implementation of a universal code to HTML converter using the Torchlight API for professional syntax highlighting.

## 🚀 Features

- **Multi-language Support**: 40+ programming languages including PHP, JavaScript, TypeScript, Python, Java, C/C++, Go, Rust, Ruby, and many more
- **Professional Syntax Highlighting**: Powered by Torchlight API with beautiful themes
- **Modern Design**: Dark theme with responsive layout and copy-to-clipboard functionality
- **Fast Performance**: Written in Rust for maximum speed and efficiency
- **Flexible Options**: Convert single files or entire directories with various configuration options
- **Memory Safe**: Leverages Rust's memory safety guarantees for reliable operation

## 📦 Installation

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) and npm (for Torchlight CLI)
- [Torchlight API Token](https://torchlight.dev) (free tier available)

### Quick Setup (Recommended)

```bash
# Clone or navigate to the project directory
cd rust-code-converter

# Run the setup script (installs dependencies and builds project)
./setup.sh

# Edit .env file with your Torchlight token
nano .env
```

### Manual Setup

```bash
# Build the project
cargo build --release

# Install Torchlight CLI
npm install -g @torchlight-api/torchlight-cli

# Copy environment template
cp .env.example .env

# Edit with your token
nano .env
```

## 🔧 Configuration

### Environment File Setup

The project uses a `.env` file for configuration. Copy `.env.example` to `.env` and customize:

```bash
cp .env.example .env
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `TORCHLIGHT_TOKEN` | Your Torchlight API token (**required**) | - |
| `TORCHLIGHT_THEME` | Syntax highlighting theme | `material-theme-palenight` |
| `REMOVE_EMPTY_LINES` | Set to `true` to remove empty lines | `false` |
| `OUTPUT_DIR` | Directory for generated HTML files | `output` |
| `CACHE_DIR` | Directory for Torchlight caching | `cache` |
| `RUST_LOG` | Logging level (error, warn, info, debug, trace) | `info` |
| `INCLUDE_LINE_NUMBERS` | Include line numbers in output | `true` |
| `MAX_FILE_SIZE` | Maximum file size to process (bytes) | `10485760` |
| `ENABLE_CONCURRENT` | Enable parallel processing (experimental) | `false` |

### Getting Your Token

1. Visit [torchlight.dev](https://torchlight.dev)
2. Sign up for a free account
3. Get your API token from the dashboard
4. Set it as an environment variable:

```bash
export TORCHLIGHT_TOKEN=your_token_here
```

## 🖥️ Usage

### Basic Usage

```bash
# Convert a single file
./target/release/rust-code-converter script.js

# Convert all code files in a directory
./target/release/rust-code-converter ../my-project/

# Remove empty lines during conversion
./target/release/rust-code-converter --no-empty app.py

# Show help
./target/release/rust-code-converter --help
```

### Development Usage

```bash
# Run directly with cargo
cargo run -- script.js
cargo run -- --no-empty app.py
cargo run -- ../my-project/
```

### Advanced Examples

```bash
# Convert with custom theme
TORCHLIGHT_THEME=github-dark cargo run -- main.rs

# Convert and remove empty lines
REMOVE_EMPTY_LINES=true cargo run -- messy-code.php

# Full example with all options
TORCHLIGHT_TOKEN=your_token TORCHLIGHT_THEME=monokai REMOVE_EMPTY_LINES=true cargo run -- --no-empty styles.css
```

## 🎨 Supported Languages

The converter automatically detects the language from file extensions:

- **Web**: HTML, CSS, SCSS, Sass, Less, JavaScript, TypeScript, PHP
- **Systems**: C, C++, Rust, Go, Assembly
- **Enterprise**: Java, C#, Kotlin, Scala, Swift
- **Scripting**: Python, Ruby, Perl, Lua, Bash, PowerShell
- **Data**: JSON, XML, YAML, SQL, TOML
- **Mobile**: Dart, Swift, Kotlin
- **Other**: R, MATLAB, Vim, Markdown, Dockerfile

## 📁 Output Structure

```
output/
├── filename1.html
├── filename2.html
└── ...

cache/
└── torchlight-cache/
```

Generated HTML files include:
- Syntax highlighted code with line numbers
- Copy-to-clipboard functionality
- Responsive dark theme design
- Professional styling with modern fonts

## 🏗️ Project Structure

```
rust-code-converter/
├── Cargo.toml           # Dependencies and project config
├── src/
│   └── main.rs         # Main application code
├── README.md           # This file
├── output/             # Generated HTML files (created automatically)
└── cache/              # Torchlight cache (created automatically)
```

## 🔧 Dependencies

### Rust Crates

- **clap**: Command-line argument parsing
- **serde/serde_json**: JSON serialization for Torchlight config
- **tokio**: Async runtime (for future HTTP features)
- **reqwest**: HTTP client (for future direct API integration)
- **walkdir**: Directory traversal
- **anyhow**: Error handling
- **log/env_logger**: Logging
- **tempfile**: Temporary file management

### External Dependencies

- **Torchlight CLI**: Syntax highlighting service
- **Node.js/npm**: For running Torchlight CLI

## 🚀 Performance

The Rust implementation offers significant performance improvements over the Node.js version:

- **Faster file I/O**: Native Rust file operations
- **Lower memory usage**: Efficient memory management
- **Better error handling**: Comprehensive error reporting with context
- **Concurrent processing**: Ready for parallel file processing (future feature)

## 🛠️ Development

### Running Tests

```bash
cargo test
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Run clippy for lints
cargo clippy

# Check without building
cargo check
```

### Debugging

```bash
# Run with debug logging
RUST_LOG=debug cargo run -- script.js

# Run with trace logging
RUST_LOG=trace cargo run -- script.js
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Torchlight](https://torchlight.dev) for the amazing syntax highlighting API
- Original Node.js implementation inspiration
- Rust community for excellent tooling and libraries

## 🐛 Troubleshooting

### Common Issues

1. **"TORCHLIGHT_TOKEN not set"**
   - Ensure you have exported the token: `export TORCHLIGHT_TOKEN=your_token`

2. **"Failed to execute torchlight command"**
   - Make sure Torchlight CLI is installed: `npm install -g @torchlight-api/torchlight-cli`
   - Check that `npx` is available in your PATH

3. **"File not found"**
   - Verify the file path is correct
   - Check file permissions

4. **Build errors**
   - Ensure you have the latest stable Rust: `rustup update`
   - Try cleaning and rebuilding: `cargo clean && cargo build`

### Getting Help

- Check the [issues](https://github.com/your-username/rust-code-converter/issues) page
- Create a new issue with detailed error information
- Include your operating system, Rust version, and command used
