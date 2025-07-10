use std::fs;
use std::process::Command;
use anyhow::{Context, Result};
use crate::config::{Config, TorchlightConfig, TorchlightOptions, TorchlightHighlight};

pub struct TorchlightProcessor;

impl TorchlightProcessor {
    pub fn create_config(config: &Config, temp_dir: &str) -> Result<()> {
        let torchlight_config = TorchlightConfig {
            token: config.torchlight_token.clone(),
            cache: config.cache_dir.clone(),
            theme: config.theme.clone(),
            host: "https://api.torchlight.dev".to_string(),
            options: TorchlightOptions {
                line_numbers: config.line_numbers,
                diff_indicators: true,
                diff_indicators_in_place_of_line_numbers: true,
            },
            highlight: TorchlightHighlight {
                input: temp_dir.to_string(),
                output: temp_dir.to_string(),
                include_globs: vec!["**/*.html".to_string()],
                exclude_patterns: vec!["/node_modules/".to_string(), "/vendor/".to_string()],
            },
        };

        let config_str = format!("module.exports = {};", serde_json::to_string_pretty(&torchlight_config)?);
        fs::write("torchlight.config.js", config_str)
            .with_context(|| "Failed to write torchlight config")?;
        
        Ok(())
    }

    pub fn run_highlighting() -> Result<bool> {
        let torchlight_result = Command::new("npx")
            .arg("torchlight")
            .output()
            .with_context(|| "Failed to execute torchlight command")?;

        Ok(torchlight_result.status.success())
    }

    pub fn cleanup_config() {
        let _ = fs::remove_file("torchlight.config.js");
    }
}
