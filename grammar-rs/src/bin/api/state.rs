//! Application state - pre-built pipelines for each language

use grammar_rs::prelude::*;
use grammar_rs::checker::{
    EnglishConfusionRule, FrenchConfusionRule,
    AhoPatternRuleChecker, ReplaceRuleChecker,
    StyleChecker, CoherencyChecker, DiacriticsChecker,
    ContractionChecker, ContextChecker,
    EN_PATTERN_RULES, FR_PATTERN_RULES,
    EN_REPLACE_RULES, FR_REPLACE_RULES,
};
use std::sync::Arc;

/// Application state shared across all requests
pub struct AppState {
    pub en_pipeline: Arc<Pipeline>,
    pub fr_pipeline: Arc<Pipeline>,
    pub language_detector: LanguageDetector,
}

impl AppState {
    /// Create a new application state with pre-built pipelines
    pub fn new() -> Self {
        tracing::info!("Building English pipeline...");
        let en_pipeline = Self::create_en_pipeline();

        tracing::info!("Building French pipeline...");
        let fr_pipeline = Self::create_fr_pipeline();

        tracing::info!("Initializing language detector...");
        let language_detector = LanguageDetector::new();

        tracing::info!("Application state initialized");

        Self {
            en_pipeline: Arc::new(en_pipeline),
            fr_pipeline: Arc::new(fr_pipeline),
            language_detector,
        }
    }

    /// Get the appropriate pipeline for a language
    pub fn get_pipeline(&self, lang: &str) -> &Pipeline {
        if lang.starts_with("fr") {
            &self.fr_pipeline
        } else {
            &self.en_pipeline
        }
    }

    /// Create the English pipeline with all checkers
    fn create_en_pipeline() -> Pipeline {
        Pipeline::new(
            SimpleTokenizer::new(),
            PassthroughAnalyzer::new(),
        )
        // Basic grammar rules + confusion pairs
        .with_checker(
            RuleChecker::new()
                .with_english_rules()
                .with_rule(EnglishConfusionRule)
        )
        // Pattern-based rules (Aho-Corasick for speed)
        .with_checker(AhoPatternRuleChecker::new(EN_PATTERN_RULES))
        // Simple replacements
        .with_checker(ReplaceRuleChecker::new(EN_REPLACE_RULES, "EN_REPLACE"))
        // Style checking (wordiness, redundancy) - uses default EN_STYLE_RULES
        .with_checker(StyleChecker::new())
        // Coherency checking (US/UK spelling consistency) - uses default
        .with_checker(CoherencyChecker::new())
        // Diacritics (café, résumé, etc.) - uses default
        .with_checker(DiacriticsChecker::new())
        // Contractions (don't, won't, etc.) - uses default
        .with_checker(ContractionChecker::new())
        // Context-sensitive words - uses default
        .with_checker(ContextChecker::new())
        // Default filters (URLs, code, quotes, etc.)
        .with_default_filters()
    }

    /// Create the French pipeline with all checkers
    fn create_fr_pipeline() -> Pipeline {
        Pipeline::new(
            SimpleTokenizer::new(),
            PassthroughAnalyzer::new(),
        )
        // Basic French grammar rules + confusion pairs
        .with_checker(
            RuleChecker::new()
                .with_french_rules()
                .with_rule(FrenchConfusionRule)
        )
        // Pattern-based rules
        .with_checker(AhoPatternRuleChecker::new(FR_PATTERN_RULES))
        // Simple replacements
        .with_checker(ReplaceRuleChecker::new(FR_REPLACE_RULES, "FR_REPLACE"))
        // Default filters
        .with_default_filters()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
