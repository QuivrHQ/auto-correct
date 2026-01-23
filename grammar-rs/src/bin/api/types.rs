//! LanguageTool-compatible API types
//!
//! These types match the LanguageTool API format for drop-in compatibility.

use serde::{Deserialize, Serialize};

/// Request for /v2/check endpoint
#[derive(Debug, Deserialize)]
pub struct CheckRequest {
    pub text: String,
    pub language: String,
    #[serde(rename = "preferredVariants")]
    pub preferred_variants: Option<String>,
    #[serde(rename = "disabledRules")]
    pub disabled_rules: Option<String>,
    #[serde(rename = "enabledRules")]
    pub enabled_rules: Option<String>,
    pub level: Option<String>,
}

/// Response from /v2/check endpoint
#[derive(Debug, Serialize)]
pub struct LanguageToolResponse {
    pub software: Software,
    pub language: LanguageInfo,
    pub matches: Vec<LTMatch>,
}

/// Software information
#[derive(Debug, Serialize)]
pub struct Software {
    pub name: String,
    pub version: String,
    #[serde(rename = "apiVersion")]
    pub api_version: i32,
}

/// Language information
#[derive(Debug, Serialize)]
pub struct LanguageInfo {
    pub code: String,
    pub name: String,
    #[serde(rename = "detectedLanguage")]
    pub detected_language: DetectedLanguage,
}

/// Detected language information
#[derive(Debug, Serialize)]
pub struct DetectedLanguage {
    pub code: String,
    pub name: String,
    pub confidence: f32,
}

/// A grammar/spelling error match
#[derive(Debug, Serialize)]
pub struct LTMatch {
    pub message: String,
    #[serde(rename = "shortMessage")]
    pub short_message: String,
    pub offset: usize,
    pub length: usize,
    pub replacements: Vec<Replacement>,
    pub rule: RuleInfo,
    pub context: Context,
}

/// A suggested replacement
#[derive(Debug, Serialize)]
pub struct Replacement {
    pub value: String,
}

/// Information about the rule that triggered the match
#[derive(Debug, Serialize)]
pub struct RuleInfo {
    pub id: String,
    pub category: Category,
}

/// Category of the rule
#[derive(Debug, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

/// Context around the error
#[derive(Debug, Serialize)]
pub struct Context {
    pub text: String,
    pub offset: usize,
    pub length: usize,
}

/// Response for /v2/languages endpoint
#[derive(Debug, Serialize)]
pub struct LanguageResponse {
    pub name: String,
    pub code: String,
    #[serde(rename = "longCode")]
    pub long_code: String,
}
