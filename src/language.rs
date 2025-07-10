use std::path::Path;

pub fn get_language_from_extension(filename: &str) -> &'static str {
    let path = Path::new(filename);
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    match extension.to_lowercase().as_str() {
        "php" => "php",
        "js" => "javascript",
        "jsx" => "javascript",
        "ts" => "typescript",
        "tsx" => "typescript",
        "py" => "python",
        "java" => "java",
        "c" => "c",
        "cpp" | "cc" | "cxx" => "cpp",
        "h" => "c",
        "hpp" => "cpp",
        "cs" => "csharp",
        "go" => "go",
        "rs" => "rust",
        "rb" => "ruby",
        "swift" => "swift",
        "kt" => "kotlin",
        "scala" => "scala",
        "html" | "htm" => "html",
        "css" => "css",
        "scss" => "scss",
        "sass" => "sass",
        "less" => "less",
        "json" => "json",
        "xml" => "xml",
        "yaml" | "yml" => "yaml",
        "sql" => "sql",
        "sh" | "bash" | "zsh" | "fish" => "bash",
        "ps1" => "powershell",
        "r" => "r",
        "m" => "matlab",
        "pl" => "perl",
        "lua" => "lua",
        "dart" => "dart",
        "vim" => "vim",
        "toml" => "toml",
        "ini" => "ini",
        "conf" => "conf",
        "dockerfile" => "dockerfile",
        "md" => "markdown",
        _ => "text",
    }
}

pub fn get_supported_extensions() -> Vec<&'static str> {
    vec![
        "php", "js", "jsx", "ts", "tsx", "py", "java", "c", "cpp", "cc", "cxx", "h", "hpp",
        "cs", "go", "rs", "rb", "swift", "kt", "scala", "html", "htm", "css", "scss", "sass",
        "less", "json", "xml", "yaml", "yml", "sql", "sh", "bash", "zsh", "fish", "ps1",
        "r", "m", "pl", "lua", "dart", "vim", "toml", "ini", "conf", "dockerfile", "md", "txt"
    ]
}
