use std::fs;
use anyhow::{Context, Result};

pub struct TemplateManager;

impl TemplateManager {
    pub fn load_code_block_template() -> Result<String> {
        let template_path = "templates/code_block.html";
        fs::read_to_string(template_path)
            .with_context(|| format!("Failed to read template file: {}", template_path))
    }

    pub fn render_code_block(
        template: &str,
        filename: &str,
        language: &str,
        theme: &str,
        code: &str,
    ) -> String {
        template
            .replace("{filename}", filename)
            .replace("{language}", language)
            .replace("{theme}", theme)
            .replace("{code}", code)
    }
}

pub fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#039;")
}
