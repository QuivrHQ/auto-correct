//! Convert grammar-rs types to LanguageTool-compatible format

use grammar_rs::core::{CheckResult, Match as GrsMatch, Severity};
use crate::types::*;

/// Convert a grammar-rs CheckResult to LanguageTool format
pub fn convert_result(
    result: CheckResult,
    text: &str,
    lang_code: &str,
    confidence: f32,
) -> LanguageToolResponse {
    let matches: Vec<LTMatch> = result
        .matches
        .into_iter()
        .map(|m| convert_match(m, text))
        .collect();

    LanguageToolResponse {
        software: Software {
            name: "grammar-rs".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            api_version: 1,
        },
        language: LanguageInfo {
            code: lang_code.to_string(),
            name: language_name(lang_code).to_string(),
            detected_language: DetectedLanguage {
                code: lang_code.to_string(),
                name: language_name(lang_code).to_string(),
                confidence,
            },
        },
        matches,
    }
}

/// Convert a single grammar-rs Match to LanguageTool format
fn convert_match(m: GrsMatch, text: &str) -> LTMatch {
    // Calculate context (40 chars around the error, capped at text boundaries)
    let context_start = m.span.start.saturating_sub(20);
    let context_end = (m.span.end + 20).min(text.len());

    // Ensure we don't split UTF-8 characters by finding nearest char boundaries
    let context_start = floor_char_boundary(text, context_start);
    let context_end = ceil_char_boundary(text, context_end);

    let context_text = &text[context_start..context_end];

    LTMatch {
        message: m.message.clone(),
        short_message: short_message(&m.message),
        offset: m.span.start,
        length: m.span.end - m.span.start,
        replacements: m
            .suggestions
            .iter()
            .take(5) // Limit to 5 suggestions
            .map(|s| Replacement { value: s.clone() })
            .collect(),
        rule: RuleInfo {
            id: m.rule_id.clone(),
            category: categorize_rule(&m.rule_id, m.severity),
        },
        context: Context {
            text: context_text.to_string(),
            offset: m.span.start - context_start,
            length: m.span.end - m.span.start,
        },
    }
}

/// Extract a short message from the full message
fn short_message(message: &str) -> String {
    // Take the first sentence or first 50 chars
    message
        .split('.')
        .next()
        .unwrap_or(message)
        .chars()
        .take(50)
        .collect()
}

/// Map rule IDs to categories
fn categorize_rule(rule_id: &str, severity: Severity) -> Category {
    let (cat_id, cat_name) = match rule_id {
        // Spelling
        "SPELL" | "HUNSPELL" => ("TYPOS", "Typo"),

        // Typography
        "DOUBLE_SPACE" | "REPEATED_PUNCTUATION" | "MISSING_SPACE_AFTER_PUNCT"
        | "TYPOGRAPHIC_QUOTES" => ("TYPOGRAPHY", "Typography"),

        // Repeated words
        "REPEATED_WORD" => ("DUPLICATION", "Duplication"),

        // Grammar - English
        "A_AN" | "IMPROVED_A_AN" | "SUBJECT_VERB_AGREEMENT" | "ITS_ITS" | "YOUR_YOURE"
        | "THEIR_THEYRE_THERE" | "COMMA_SPLICE" | "LESS_FEWER" | "WHO_WHOM"
        | "GOOD_WELL" | "DOUBLE_NEGATIVE" | "SENTENCE_FRAGMENT" => ("GRAMMAR", "Grammar"),

        // Grammar - French
        _ if rule_id.starts_with("FR_") => ("GRAMMAR", "French Grammar"),

        // Style
        "PASSIVE_VOICE" | "WORDINESS" | "SENTENCE_LENGTH" | "CLICHE" | "REDUNDANCY" => {
            ("STYLE", "Style")
        }

        // Confusion
        "EN_CONFUSION" | "FR_CONFUSION" => ("CONFUSED_WORDS", "Commonly Confused Words"),

        // Coherency
        "COHERENCY" | "EN_COHERENCY" => ("CONSISTENCY", "Consistency"),

        // Diacritics
        "DIACRITICS" | "EN_DIACRITICS" => ("TYPOGRAPHY", "Typography"),

        // Contractions
        "CONTRACTION" | "EN_CONTRACTION" => ("GRAMMAR", "Grammar"),

        // Pattern-based rules
        _ if rule_id.starts_with("PATTERN_") => {
            match severity {
                Severity::Error => ("GRAMMAR", "Grammar"),
                Severity::Warning => ("STYLE", "Style"),
                Severity::Hint => ("HINTS", "Hints"),
            }
        }

        // Replace rules
        _ if rule_id.starts_with("REPLACE_") => ("STYLE", "Style"),

        // Default
        _ => match severity {
            Severity::Error => ("GRAMMAR", "Grammar"),
            Severity::Warning => ("STYLE", "Style"),
            Severity::Hint => ("MISC", "Miscellaneous"),
        },
    };

    Category {
        id: cat_id.to_string(),
        name: cat_name.to_string(),
    }
}

/// Get the human-readable name for a language code
fn language_name(code: &str) -> &'static str {
    match code {
        "en" | "en-US" => "English (US)",
        "en-GB" => "English (GB)",
        "fr" | "fr-FR" => "French",
        "fr-CA" => "French (Canada)",
        "auto" => "Auto-detected",
        _ => "Unknown",
    }
}

/// Round down to the nearest UTF-8 character boundary
fn floor_char_boundary(s: &str, mut index: usize) -> usize {
    if index > s.len() {
        return s.len();
    }
    while index > 0 && !s.is_char_boundary(index) {
        index -= 1;
    }
    index
}

/// Round up to the nearest UTF-8 character boundary
fn ceil_char_boundary(s: &str, mut index: usize) -> usize {
    if index >= s.len() {
        return s.len();
    }
    while index < s.len() && !s.is_char_boundary(index) {
        index += 1;
    }
    index
}
