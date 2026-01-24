//! Application state - pre-built pipelines for each language

use grammar_rs::prelude::*;
use grammar_rs::checker::{
    EnglishConfusionRule, FrenchConfusionRule,
    AhoPatternRuleChecker, ReplaceRuleChecker,
    StyleChecker, CoherencyChecker, DiacriticsChecker,
    ContractionChecker, ContextChecker,
    PosPatternChecker, UncountableNounChecker, CompoundWordChecker,
    ProhibitChecker, SpellChecker, NgramConfusionChecker,
    // Dynamic pattern checker for complex rules (regex, postag_regexp, skip, etc.)
    create_en_dynamic_checker, create_fr_dynamic_checker,
    EN_PATTERN_RULES, FR_PATTERN_RULES,
    EN_REPLACE_RULES, FR_REPLACE_RULES,
    EN_ANTIPATTERNS, FR_ANTIPATTERNS,
    EN_POS_PATTERN_RULES, FR_POS_PATTERN_RULES,
    EN_ADDED_WORDS,
    // Number words for POS tagging
    EN_NUMBERS, FR_NUMBERS,
    // Spelling skip lists and dictionaries
    EN_IGNORE, EN_PROPER_NOUNS,
    FR_IGNORE, FR_SPELLING, FR_COMMON_WORDS,
    // Disambiguation skip patterns
    EN_DISAMBIG_SKIP, FR_DISAMBIG_SKIP,
};
use grammar_rs::dictionary::FstDictionary;
use grammar_rs::core::PosTag;
use std::sync::Arc;
use std::path::Path;
use moka::future::Cache;
use crate::types::{CacheKey, LanguageToolResponse};

/// Application state shared across all requests
pub struct AppState {
    pub en_pipeline: Arc<Pipeline>,
    pub fr_pipeline: Arc<Pipeline>,
    pub language_detector: LanguageDetector,
    pub cache: Cache<CacheKey, Arc<LanguageToolResponse>>,
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

        // Initialize cache with configurable settings
        let cache_size = std::env::var("CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10_000);

        let cache_ttl = std::env::var("CACHE_TTL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3600); // 1 hour default

        let cache = Cache::builder()
            .max_capacity(cache_size as u64)
            .time_to_live(std::time::Duration::from_secs(cache_ttl))
            .build();

        tracing::info!("Cache initialized (capacity: {}, TTL: {}s)", cache_size, cache_ttl);
        tracing::info!("Application state initialized");

        Self {
            en_pipeline: Arc::new(en_pipeline),
            fr_pipeline: Arc::new(fr_pipeline),
            language_detector,
            cache,
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

        // Load number words (twenty-one, thirty-five, etc.) as CD (cardinal number)
        tagger.load_from_lines(EN_NUMBERS.iter().copied());

        tracing::debug!("EN POS tagger loaded with {} dictionary entries + suffix heuristics",
                       tagger.dictionary_size());
        tagger
    }

    /// Create a French POS tagger with number words
    fn create_fr_pos_tagger() -> PosTagger {
        let mut tagger = PosTagger::new();

        // Load French number words (vingt-et-un, trente-deux, etc.) as CD (cardinal number)
        tagger.load_from_lines(FR_NUMBERS.iter().copied());

        tracing::debug!("FR POS tagger loaded with {} dictionary entries + suffix heuristics",
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

    /// Create an English spell checker with FST dictionary (370K words)
    fn create_en_spell_checker() -> Option<SpellChecker> {
        let dict_path = Path::new("data/dictionaries/en_US.fst");
        if !dict_path.exists() {
            tracing::warn!("EN dictionary not found at {:?}, spell checking disabled", dict_path);
            return None;
        }

        match FstDictionary::from_fst(dict_path) {
            Ok(dict) => {
                let word_count = dict.len();
                let skip_count = EN_IGNORE.len() + EN_PROPER_NOUNS.len() + EN_DISAMBIG_SKIP.len();
                let checker = SpellChecker::with_fst_dictionary(dict)
                    .with_skip_words(EN_IGNORE.iter().copied())
                    .with_skip_words(EN_PROPER_NOUNS.iter().copied())
                    .with_skip_words(EN_DISAMBIG_SKIP.iter().copied());
                tracing::info!("EN spell checker enabled ({} dictionary words, {} skip words)",
                              word_count, skip_count);
                Some(checker)
            }
            Err(e) => {
                tracing::warn!("Failed to load EN dictionary: {}", e);
                None
            }
        }
    }

    /// Create a French spell checker using FR_COMMON_WORDS + FR_SPELLING as dictionary
    fn create_fr_spell_checker() -> Option<SpellChecker> {
        // FR doesn't have a full FST dictionary, combine FR_COMMON_WORDS (9.7K) + FR_SPELLING (34K)
        // This gives ~44K words which covers basic French vocabulary
        let checker = SpellChecker::new()
            .with_words(FR_COMMON_WORDS.iter().copied())
            .with_words(FR_SPELLING.iter().copied())
            .with_skip_words(FR_IGNORE.iter().copied())
            .with_skip_words(FR_DISAMBIG_SKIP.iter().copied());

        let total_words = FR_COMMON_WORDS.len() + FR_SPELLING.len();
        let skip_count = FR_IGNORE.len() + FR_DISAMBIG_SKIP.len();
        tracing::info!("FR spell checker enabled ({} dictionary words, {} skip words)",
                      total_words, skip_count);
        Some(checker)
    }

    /// Create the English pipeline with all checkers
    fn create_en_pipeline() -> Pipeline {
        // Use POS tagger instead of passthrough for better rule matching
        let pos_tagger = Self::create_en_pos_tagger();

        let mut pipeline = Pipeline::new(
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
        // Prohibited words (common misspellings that are always wrong)
        .with_checker(ProhibitChecker::new())
        // Dynamic pattern checker (complex rules with regex, postag_regexp, skip, etc.)
        // 2110 additional rules from LanguageTool that can't be compiled to static code
        .with_checker(create_en_dynamic_checker());

        tracing::info!("EN dynamic pattern checker enabled ({} rules)", create_en_dynamic_checker().rule_count());

        // Spell checker (370K word FST dictionary + skip lists)
        if let Some(spell_checker) = Self::create_en_spell_checker() {
            pipeline = pipeline.with_checker(spell_checker);
        }

        // N-gram confusion checker (optional - requires data/ngrams/en_ngrams.bin)
        if let Some(ngram_checker) = NgramConfusionChecker::try_load_en() {
            tracing::info!("EN N-gram confusion checker enabled");
            pipeline = pipeline.with_checker(ngram_checker);
        }

        // Default filters (URLs, code, quotes, etc.)
        pipeline.with_default_filters()
    }

    /// Create the French pipeline with all checkers
    fn create_fr_pipeline() -> Pipeline {
        // Use POS tagger for better rule matching (includes French number words)
        let pos_tagger = Self::create_fr_pos_tagger();

        let mut pipeline = Pipeline::new(
            SimpleTokenizer::new(),
            pos_tagger,
        )
        // Basic French grammar rules + confusion pairs
        .with_checker(
            RuleChecker::new()
                .with_french_rules()
                .with_rule(FrenchConfusionRule)
        )
        // Pattern-based rules with antipattern filtering
        .with_checker(AhoPatternRuleChecker::with_antipatterns(FR_PATTERN_RULES, FR_ANTIPATTERNS))
        // POS pattern rules (25 rules from LanguageTool)
        .with_checker(PosPatternChecker::with_rules(FR_POS_PATTERN_RULES))
        // Simple replacements
        .with_checker(ReplaceRuleChecker::new(FR_REPLACE_RULES, "FR_REPLACE"))
        // Style checking (wordiness, redundancy) - 51 FR rules
        .with_checker(StyleChecker::french())
        // Compound word errors (aller retour → aller-retour)
        .with_checker(CompoundWordChecker::french())
        // Dynamic pattern checker (complex rules with regex, postag_regexp, skip, etc.)
        // 826 additional rules from LanguageTool that can't be compiled to static code
        .with_checker(create_fr_dynamic_checker());

        tracing::info!("FR dynamic pattern checker enabled ({} rules)", create_fr_dynamic_checker().rule_count());

        // Spell checker (34K word dictionary from FR_SPELLING + skip list)
        if let Some(spell_checker) = Self::create_fr_spell_checker() {
            pipeline = pipeline.with_checker(spell_checker);
        }

        // N-gram confusion checker (optional - requires data/ngrams/fr_ngrams.bin)
        if let Some(ngram_checker) = NgramConfusionChecker::try_load_fr() {
            tracing::info!("FR N-gram confusion checker enabled");
            pipeline = pipeline.with_checker(ngram_checker);
        }

        // Default filters
        pipeline.with_default_filters()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
