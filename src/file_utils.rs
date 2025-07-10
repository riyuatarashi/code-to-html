use std::path::{Path, PathBuf};
use anyhow::Result;
use walkdir::WalkDir;
use crate::language::get_supported_extensions;

pub fn find_code_files(dir_path: &Path) -> Result<Vec<PathBuf>> {
    let supported_extensions = get_supported_extensions();
    let mut code_files = Vec::new();

    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        
        // Skip files in excluded directories
        if is_excluded_directory(path) {
            continue;
        }

        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            if supported_extensions.contains(&extension.to_lowercase().as_str()) {
                code_files.push(path.to_path_buf());
            }
        }
    }

    Ok(code_files)
}

fn is_excluded_directory(path: &Path) -> bool {
    path.ancestors().any(|ancestor| {
        ancestor.file_name().map_or(false, |name| {
            matches!(name.to_str(), Some("node_modules") | Some("vendor") | Some(".git") | Some("target"))
        })
    })
}

pub fn process_empty_lines(code: &str, remove_empty_lines: bool) -> String {
    if !remove_empty_lines {
        return code.to_string();
    }

    let lines: Vec<&str> = code.lines().collect();
    let mut filtered_lines: Vec<&str> = lines.into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect();
    
    // Remove trailing empty lines
    while let Some(last) = filtered_lines.last() {
        if last.trim().is_empty() {
            filtered_lines.pop();
        } else {
            break;
        }
    }
    
    filtered_lines.join("\n")
}
