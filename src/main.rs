mod config;
mod language;
mod template;
mod file_utils;
mod torchlight;
mod converter;

use clap::{Arg, Command};
use log::{error, warn};
use std::env;
use dotenv::dotenv;
use std::path::Path;
use std::process;
use anyhow::Result;

use crate::config::Config;
use crate::language::get_supported_extensions;
use crate::converter::CodeToHTMLConverter;

fn main() -> Result<()> {
    env_logger::init();

    let matches = Command::new("rust-code-converter")
        .version("1.0.0")
        .about("Universal Code to HTML Converter using Torchlight")
        .arg(
            Arg::new("input")
                .help("Input file or directory")
                .required(false)
                .index(1)
        )
        .arg(
            Arg::new("no-empty")
                .long("no-empty")
                .help("Remove empty lines from code")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    if matches.get_one::<String>("input").is_none() {
        print_help();
        return Ok(());
    }

    // Load .env file
    if let Err(err) = dotenv() {
        eprintln!("Warning: Could not load .env file: {}", err);
        eprintln!("Make sure you have a .env file in the project root or set environment variables directly.");
    }

    let torchlight_token = env::var("TORCHLIGHT_TOKEN")
        .map_err(|_| {
            error!("❌ Error: TORCHLIGHT_TOKEN environment variable is required");
            println!("Get your token at: https://torchlight.dev");
            println!("Set it with: export TORCHLIGHT_TOKEN=your_token");
            anyhow::anyhow!("TORCHLIGHT_TOKEN not set")
        })?;

    let remove_empty_lines = matches.get_flag("no-empty") || 
        env::var("REMOVE_EMPTY_LINES").unwrap_or_default() == "true";

    let theme = env::var("TORCHLIGHT_THEME")
        .unwrap_or_else(|_| "material-theme-palenight".to_string());

    let include_line_numbers = env::var("INCLUDE_LINE_NUMBERS")
        .unwrap_or_else(|_| "true".to_string())
        .to_lowercase() == "true";

    let output_dir = env::var("OUTPUT_DIR")
        .unwrap_or_else(|_| "output".to_string());

    let cache_dir = env::var("CACHE_DIR")
        .unwrap_or_else(|_| "cache".to_string());

    let config = Config {
        torchlight_token,
        theme,
        line_numbers: include_line_numbers,
        cache_dir,
        output_dir,
        remove_empty_lines,
    };

    let converter = CodeToHTMLConverter::new(config)?;

    let input_path = matches.get_one::<String>("input").unwrap();
    let input_path = Path::new(input_path);

    if !input_path.exists() {
        error!("❌ Error: File or directory not found: {}", input_path.display());
        process::exit(1);
    }

    if input_path.is_file() {
        let supported_extensions = get_supported_extensions();

        if let Some(extension) = input_path.extension().and_then(|ext| ext.to_str()) {
            if !supported_extensions.contains(&extension.to_lowercase().as_str()) {
                warn!("⚠️  Warning: '{}' extension not specifically supported, treating as plain text", extension);
            }
        }

        converter.convert_code_file(input_path)?;
    } else if input_path.is_dir() {
        converter.convert_directory(input_path)?;
    }

    println!("\n🎉 Conversion completed!");
    println!("📁 Check the output directory: ./{}/", converter.config.output_dir);
    
    Ok(())
}

fn print_help() {
    println!(r#"
🚀 Universal Code to HTML Converter using Torchlight (Rust Edition)

Usage:
  rust-code-converter <file>                  Convert any code file to HTML
  rust-code-converter <directory>             Convert all code files in directory
  rust-code-converter --help                  Show this help
  rust-code-converter --no-empty <file>       Convert and remove empty lines

Supported Languages:
  PHP, JavaScript, TypeScript, Python, Java, C/C++, C#, Go, Rust, Ruby,
  Swift, HTML, CSS, SQL, Bash, PowerShell, and many more...

Environment Variables:
  TORCHLIGHT_TOKEN                        Your Torchlight API token (required)
  TORCHLIGHT_THEME                        Theme name (default: material-theme-palenight)
  REMOVE_EMPTY_LINES                      Set to 'true' to remove empty lines
  INCLUDE_LINE_NUMBERS                    Set to 'false' to disable line numbers (default: true)
  OUTPUT_DIR                              Output directory (default: output)
  CACHE_DIR                               Cache directory (default: cache)

Template Customization:
  Edit templates/code_block.html           Customize the HTML structure
  Edit templates/styles.css                Customize the CSS styling

Examples:
  rust-code-converter footer.php
  rust-code-converter script.js
  rust-code-converter --no-empty app.py
  rust-code-converter ../project-directory/
  TORCHLIGHT_TOKEN=your_token rust-code-converter main.go
  TORCHLIGHT_TOKEN=your_token REMOVE_EMPTY_LINES=true rust-code-converter styles.css
  INCLUDE_LINE_NUMBERS=false rust-code-converter main.rs
"#);
}
