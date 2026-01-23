//! Application state - pre-built pipelines for each language

use grammar_rs::prelude::*;
use grammar_rs::checker::{
    EnglishConfusionRule, FrenchConfusionRule,
    AhoPatternRuleChecker, ReplaceRuleChecker,
    StyleChecker, CoherencyChecker, DiacriticsChecker,
    ContractionChecker, ContextChecker,
    PosPatternChecker, UncountableNounChecker, CompoundWordChecker,
    EN_PATTERN_RULES, FR_PATTERN_RULES,
    EN_REPLACE_RULES, FR_REPLACE_RULES,
    EN_ANTIPATTERNS, FR_ANTIPATTERNS,
    EN_POS_PATTERN_RULES, FR_POS_PATTERN_RULES,
    EN_ADDED_WORDS,
};
use grammar_rs::core::PosTag;
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

    /// Create an English POS tagger with the added words dictionary
    fn create_en_pos_tagger() -> PosTagger {
        let mut tagger = PosTagger::new();

        // Load the POS-tagged words from LanguageTool
        for entry in EN_ADDED_WORDS {
            if let Some(pos) = PosTag::from_str(entry.pos_tag) {
                tagger.add_word(entry.word, entry.base_form, pos);
            }
        }

        tracing::debug!("EN POS tagger loaded with {} dictionary entries + suffix heuristics",
                       tagger.dictionary_size());
        tagger
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
        // Use POS tagger instead of passthrough for better rule matching
        let pos_tagger = Self::create_en_pos_tagger();

        Pipeline::new(
            SimpleTokenizer::new(),
            pos_tagger,
        )
        // Basic grammar rules + confusion pairs
        .with_checker(
            RuleChecker::new()
                .with_english_rules()
                .with_rule(EnglishConfusionRule)
        )
        // Pattern-based rules (Aho-Corasick for speed) with antipattern filtering
        .with_checker(AhoPatternRuleChecker::with_antipatterns(EN_PATTERN_RULES, EN_ANTIPATTERNS))
        // POS pattern rules (require POS tagging) - 94 rules from LanguageTool
        .with_checker(PosPatternChecker::with_rules(EN_POS_PATTERN_RULES))
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
        // Uncountable noun pluralization errors (informations → information)
        .with_checker(UncountableNounChecker::new())
        // Compound word errors (air plane → airplane, well being → well-being)
        .with_checker(CompoundWordChecker::new())
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
        // Pattern-based rules with antipattern filtering
        .with_checker(AhoPatternRuleChecker::with_antipatterns(FR_PATTERN_RULES, FR_ANTIPATTERNS))
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
