use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use tempfile::TempDir;
use log::{error, info};

use crate::config::Config;
use crate::language::get_language_from_extension;
use crate::template::{TemplateManager, escape_html};
use crate::file_utils::{find_code_files, process_empty_lines};
use crate::torchlight::TorchlightProcessor;

pub struct CodeToHTMLConverter {
    pub config: Config,
    code_block_template: String,
}

impl CodeToHTMLConverter {
    pub fn new(config: Config) -> Result<Self> {
        let converter = Self {
            code_block_template: TemplateManager::load_code_block_template()?,
            config,
        };
        converter.setup_directories()?;
        Ok(converter)
    }

    fn setup_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.config.output_dir)
            .with_context(|| format!("Failed to create output directory: {}", self.config.output_dir))?;
        
        fs::create_dir_all(&self.config.cache_dir)
            .with_context(|| format!("Failed to create cache directory: {}", self.config.cache_dir))?;
        
        Ok(())
    }

    fn create_code_block_html(&self, filename: &str, code: &str) -> String {
        let language = get_language_from_extension(filename);
        let escaped_code = escape_html(code);
        
        TemplateManager::render_code_block(
            &self.code_block_template,
            filename,
            language,
            &self.config.theme,
            &escaped_code,
        )
    }

    pub fn convert_code_file(&self, code_file_path: &Path) -> Result<Option<PathBuf>> {
        info!("Converting: {}", code_file_path.display());

        // Read code file
        let code = fs::read_to_string(code_file_path)
            .with_context(|| format!("Failed to read file: {}", code_file_path.display()))?;

        let filename = code_file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        
        let name_without_ext = code_file_path.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("unknown");

        // Process empty lines if requested
        let processed_code = process_empty_lines(&code, self.config.remove_empty_lines);

        // Create temp directory
        let temp_dir = TempDir::new()
            .with_context(|| "Failed to create temporary directory")?;
        let temp_dir_path = temp_dir.path().to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid temp directory path"))?;

        // Create code block HTML with code
        let html_content = self.create_code_block_html(filename, &processed_code);
        let temp_html_path = temp_dir.path().join(format!("{}.html", name_without_ext));
        fs::write(&temp_html_path, html_content)
            .with_context(|| "Failed to write temporary HTML file")?;

        // Create/update torchlight config
        TorchlightProcessor::create_config(&self.config, temp_dir_path)?;

        info!("Running torchlight highlighting...");

        // Run torchlight
        if !TorchlightProcessor::run_highlighting()? {
            error!("Torchlight execution failed");
            return Ok(None);
        }

        // Read the highlighted result
        let highlighted_html = fs::read_to_string(&temp_html_path)
            .with_context(|| "Failed to read highlighted HTML file")?;

        // Save to output directory
        let output_path = PathBuf::from(&self.config.output_dir).join(format!("{}.html", name_without_ext));
        fs::write(&output_path, highlighted_html)
            .with_context(|| format!("Failed to write output file: {}", output_path.display()))?;

        // Clean up torchlight config
        TorchlightProcessor::cleanup_config();

        info!("✅ Converted successfully: {}", output_path.display());
        Ok(Some(output_path))
    }

    pub fn convert_directory(&self, dir_path: &Path) -> Result<Vec<PathBuf>> {
        let code_files = find_code_files(dir_path)?;
        let mut results = Vec::new();

        info!("Found {} code files to convert", code_files.len());

        for file in code_files {
            if let Some(result) = self.convert_code_file(&file)? {
                results.push(result);
            }
        }

        Ok(results)
    }
}
