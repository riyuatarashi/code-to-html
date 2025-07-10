use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Config {
    pub torchlight_token: String,
    pub theme: String,
    pub line_numbers: bool,
    pub cache_dir: String,
    pub output_dir: String,
    pub remove_empty_lines: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TorchlightConfig {
    pub token: String,
    pub cache: String,
    pub theme: String,
    pub host: String,
    pub options: TorchlightOptions,
    pub highlight: TorchlightHighlight,
}

#[derive(Serialize, Deserialize)]
pub struct TorchlightOptions {
    #[serde(rename = "lineNumbers")]
    pub line_numbers: bool,
    #[serde(rename = "diffIndicators")]
    pub diff_indicators: bool,
    #[serde(rename = "diffIndicatorsInPlaceOfLineNumbers")]
    pub diff_indicators_in_place_of_line_numbers: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TorchlightHighlight {
    pub input: String,
    pub output: String,
    #[serde(rename = "includeGlobs")]
    pub include_globs: Vec<String>,
    #[serde(rename = "excludePatterns")]
    pub exclude_patterns: Vec<String>,
}
