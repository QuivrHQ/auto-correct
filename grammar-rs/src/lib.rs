//! Grammar-rs: Correcteur grammatical ultra-performant
//!
//! Architecture en pipeline composable:
//! ```text
//! Text → Tokenizer → Analyzer → Checker(s) → CheckResult
//! ```
//!
//! Chaque étape est un trait que tu peux implémenter/remplacer.
//!
//! # Exemple d'utilisation
//!
//! ```rust
//! use grammar_rs::prelude::*;
//!
//! // Construire le pipeline
//! let pipeline = Pipeline::new(
//!     SimpleTokenizer::new(),
//!     PassthroughAnalyzer::new(),
//! )
//! .with_checker(SpellChecker::new().with_words(["hello", "world"]))
//! .with_checker(RuleChecker::new().with_english_rules());
//!
//! // Vérifier du texte
//! let result = pipeline.check_text("helo  world");
//!
//! for m in result.matches {
//!     println!("{}: {} ({:?})", m.rule_id, m.message, m.suggestions);
//! }
//! ```

pub mod core;
pub mod tokenizer;
pub mod analyzer;
pub mod checker;
pub mod dictionary;
pub mod lang_detect;
pub mod filter;

/// Prelude - importe tout ce dont tu as besoin
pub mod prelude {
    pub use crate::core::{
        Token, TokenKind, AnalyzedToken, PosTag,
        Match, Severity, CheckResult,
        MaskKind, MaskedRegion,
    };
    pub use crate::core::traits::{
        Tokenizer, Analyzer, Checker, Suggester, GrammarChecker,
    };
    pub use crate::core::filter::{Filter, FilterChain};
    pub use crate::core::pipeline::Pipeline;
    pub use crate::tokenizer::{SimpleTokenizer, ContractionTokenizer};
    pub use crate::analyzer::{PassthroughAnalyzer, DictAnalyzer, PosTagger};
    pub use crate::checker::{SpellChecker, RuleChecker};
    pub use crate::dictionary::FstDictionary;
    pub use crate::lang_detect::{Language, LanguageDetector, DetectionResult};
    pub use crate::filter::{
        UrlFilter, CodeBlockFilter, QuotedTextFilter, DateFilter, NumberFilter,
        default_filters, FilterBuilder,
    };
}

#[cfg(test)]
mod integration_tests {
    use super::prelude::*;

    #[test]
    fn test_full_pipeline() {
        let pipeline = Pipeline::new(
            SimpleTokenizer::new(),
            PassthroughAnalyzer::new(),
        )
        .with_checker(SpellChecker::new().with_words([
            "hello", "world", "this", "is", "a", "test",
        ]))
        .with_checker(RuleChecker::new().with_english_rules());

        let result = pipeline.check_text("helo world");

        assert!(!result.matches.is_empty());
        assert!(result.matches.iter().any(|m| m.rule_id == "SPELL"));
    }

    #[test]
    fn test_french_pipeline() {
        let pipeline = Pipeline::new(
            SimpleTokenizer::new(),
            PassthroughAnalyzer::new(),
        )
        .with_checker(RuleChecker::new().with_french_rules());

        let result = pipeline.check_text("Bonjour! Comment ça va?");

        // Should detect missing spaces before ! and ?
        assert!(result.matches.iter().any(|m| m.rule_id == "FR_PUNCT_SPACE"));
    }
}
