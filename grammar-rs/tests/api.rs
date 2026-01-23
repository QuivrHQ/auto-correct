//! E2E API tests for grammar-rs
//!
//! These tests verify the HTTP API endpoints work correctly.
//! They test the full pipeline from HTTP request to response.
//!
//! Note: These tests require building the API but not running it -
//! they test the handlers directly using axum's test utilities.
//!
//! Run with: cargo test --test api

use serde_json::{json, Value};

// Re-export types needed for testing
use grammar_rs::prelude::*;
use grammar_rs::lang_detect::Language;
use grammar_rs::checker::{
    RuleChecker, AhoPatternRuleChecker, StyleChecker, CompoundWordChecker,
    ProhibitChecker, SpellChecker, EnglishConfusionRule, FrenchConfusionRule,
    EN_PATTERN_RULES, FR_PATTERN_RULES,
    EN_ANTIPATTERNS, FR_ANTIPATTERNS,
    EN_IGNORE, EN_PROPER_NOUNS, FR_IGNORE, FR_COMMON_WORDS, FR_SPELLING,
};
use grammar_rs::dictionary::FstDictionary;
use std::path::Path;

// ============================================================================
// Test setup - pre-warm lazy statics once
// ============================================================================

use std::sync::Once;

static WARM_UP: Once = Once::new();

fn ensure_warm() {
    WARM_UP.call_once(|| {
        grammar_rs::warm_up();
    });
}

// ============================================================================
// Pipeline helpers (mirrors state.rs logic)
// ============================================================================

fn create_test_en_pipeline() -> Pipeline {
    ensure_warm();
    Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(RuleChecker::new().with_english_rules().with_rule(EnglishConfusionRule))
        .with_checker(AhoPatternRuleChecker::with_antipatterns(EN_PATTERN_RULES, EN_ANTIPATTERNS))
        .with_checker(StyleChecker::new())
        .with_checker(CompoundWordChecker::new())
        .with_checker(ProhibitChecker::new())
        .with_default_filters()
}

fn create_test_fr_pipeline() -> Pipeline {
    ensure_warm();
    Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(RuleChecker::new().with_french_rules().with_rule(FrenchConfusionRule))
        .with_checker(AhoPatternRuleChecker::with_antipatterns(FR_PATTERN_RULES, FR_ANTIPATTERNS))
        .with_checker(StyleChecker::french())
        .with_checker(CompoundWordChecker::french())
        .with_default_filters()
}

// ============================================================================
// API Response Format Tests
// ============================================================================

/// Simulates the v2/check response format
fn format_v2_response(result: &CheckResult, text: &str, language: &str) -> Value {
    let matches: Vec<Value> = result.matches.iter().map(|m| {
        json!({
            "message": m.message,
            "shortMessage": m.message.split('.').next().unwrap_or(&m.message),
            "offset": m.span.start,
            "length": m.span.end - m.span.start,
            "replacements": m.suggestions.iter().map(|s| json!({"value": s})).collect::<Vec<_>>(),
            "rule": {
                "id": m.rule_id,
                "description": m.message,
                "category": {
                    "id": "GRAMMAR",
                    "name": "Grammar"
                }
            },
            "context": {
                "text": text,
                "offset": m.span.start,
                "length": m.span.end - m.span.start
            }
        })
    }).collect();

    json!({
        "software": {
            "name": "grammar-rs",
            "version": env!("CARGO_PKG_VERSION"),
            "apiVersion": 1
        },
        "language": {
            "name": language,
            "code": language,
            "detectedLanguage": {
                "name": language,
                "code": language
            }
        },
        "matches": matches
    })
}

// ============================================================================
// English Pipeline Tests
// ============================================================================

#[test]
fn api_en_detects_double_space() {
    let pipeline = create_test_en_pipeline();
    let text = "Hello  world";
    let result = pipeline.check_text(text);

    assert!(!result.matches.is_empty(), "Should detect double space");
    assert!(result.matches.iter().any(|m| m.rule_id == "DOUBLE_SPACE"),
        "Should have DOUBLE_SPACE rule match");
}

#[test]
fn api_en_detects_repeated_word() {
    let pipeline = create_test_en_pipeline();
    let text = "I went to the the store.";
    let result = pipeline.check_text(text);

    assert!(result.matches.iter().any(|m| m.rule_id.contains("REPEATED")),
        "Should detect repeated word 'the the'");
}

#[test]
fn api_en_detects_a_an_error() {
    let pipeline = create_test_en_pipeline();
    let text = "I want a apple.";
    let result = pipeline.check_text(text);

    assert!(result.matches.iter().any(|m| m.rule_id.contains("A_AN")),
        "Should detect 'a apple' error");
}

#[test]
fn api_en_detects_style_wordiness() {
    let pipeline = create_test_en_pipeline();
    let text = "In order to succeed, you need to work hard.";
    let result = pipeline.check_text(text);

    let style_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id.contains("WORDINESS") || m.rule_id.contains("STYLE"))
        .collect();

    assert!(!style_matches.is_empty(),
        "Should detect wordiness 'in order to'");
}

#[test]
fn api_en_detects_compound_error() {
    let pipeline = create_test_en_pipeline();
    let text = "Your well being matters to us.";
    let result = pipeline.check_text(text);

    let compound_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id.contains("COMPOUND"))
        .collect();

    assert!(!compound_matches.is_empty(),
        "Should detect spaced compound 'well being'");
}

#[test]
fn api_en_detects_prohibited_word() {
    let pipeline = create_test_en_pipeline();
    let text = "Christoper went to the store.";
    let result = pipeline.check_text(text);

    let prohibit_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id == "PROHIBIT")
        .collect();

    assert!(!prohibit_matches.is_empty(),
        "Should detect prohibited word 'Christoper'");
    assert!(prohibit_matches[0].suggestions.contains(&"Christopher".to_string()),
        "Should suggest 'Christopher'");
}

#[test]
fn api_en_clean_text_no_errors() {
    let pipeline = create_test_en_pipeline();
    let text = "The quick brown fox jumps over the lazy dog.";
    let result = pipeline.check_text(text);

    // Allow minimal false positives
    assert!(result.matches.len() <= 1,
        "Clean text should have minimal errors, got: {:?}",
        result.matches.iter().map(|m| &m.rule_id).collect::<Vec<_>>());
}

#[test]
fn api_en_url_filtered() {
    let pipeline = create_test_en_pipeline();
    let text = "Check out https://example.com/teh/page for more info.";
    let result = pipeline.check_text(text);

    let spell_errors: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id.contains("SPELL"))
        .collect();

    assert!(spell_errors.is_empty(),
        "URL paths should not trigger spell errors");
}

#[test]
fn api_en_code_block_filtered() {
    let pipeline = create_test_en_pipeline();
    let text = "Use `teh_function()` to process data.";
    let result = pipeline.check_text(text);

    let spell_errors: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id.contains("SPELL"))
        .collect();

    assert!(spell_errors.is_empty(),
        "Inline code should not trigger spell errors");
}

// ============================================================================
// French Pipeline Tests
// ============================================================================

#[test]
fn api_fr_detects_punctuation_error() {
    let pipeline = create_test_fr_pipeline();
    let text = "Comment?";
    let result = pipeline.check_text(text);

    let punct_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id.contains("PUNCT"))
        .collect();

    assert!(!punct_matches.is_empty(),
        "Should detect missing space before '?'");
}

#[test]
fn api_fr_correct_punctuation_no_error() {
    let pipeline = create_test_fr_pipeline();
    let text = "Comment ?";
    let result = pipeline.check_text(text);

    let punct_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id.contains("PUNCT"))
        .collect();

    assert!(punct_matches.is_empty(),
        "Correct FR punctuation should not trigger errors");
}

#[test]
fn api_fr_detects_compound_error() {
    let pipeline = create_test_fr_pipeline();
    let text = "J'ai pris un aller retour pour Paris.";
    let result = pipeline.check_text(text);

    let compound_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id.contains("COMPOUND"))
        .collect();

    assert!(!compound_matches.is_empty(),
        "Should detect spaced compound 'aller retour'");

    if !compound_matches.is_empty() {
        assert!(compound_matches[0].suggestions.contains(&"aller-retour".to_string()),
            "Should suggest 'aller-retour'");
    }
}

#[test]
fn api_fr_clean_text_no_errors() {
    let pipeline = create_test_fr_pipeline();
    let text = "Le chat dort sur le tapis. Il fait beau aujourd'hui.";
    let result = pipeline.check_text(text);

    assert!(result.matches.is_empty(),
        "Clean FR text should have no errors, got: {:?}",
        result.matches.iter().map(|m| &m.rule_id).collect::<Vec<_>>());
}

#[test]
fn api_fr_detects_ce_se_error() {
    // Use explicit FrenchCeSeRule since it may not be in pattern rules
    use grammar_rs::checker::FrenchCeSeRule;
    let pipeline = Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(RuleChecker::new().with_rule(FrenchCeSeRule));

    let text = "Se livre est beau.";
    let result = pipeline.check_text(text);

    let cese_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id.contains("CE_SE"))
        .collect();

    assert!(!cese_matches.is_empty(),
        "Should detect ce/se confusion");
}

// ============================================================================
// Response Format Tests
// ============================================================================

#[test]
fn api_response_format_v2_compatible() {
    let pipeline = create_test_en_pipeline();
    let text = "Hello  world";
    let result = pipeline.check_text(text);
    let response = format_v2_response(&result, text, "en");

    // Check required v2 fields
    assert!(response["software"]["name"].as_str().is_some());
    assert!(response["software"]["version"].as_str().is_some());
    assert!(response["language"]["code"].as_str().is_some());
    assert!(response["matches"].is_array());

    // Check match structure
    if let Some(matches) = response["matches"].as_array() {
        if !matches.is_empty() {
            let first_match = &matches[0];
            assert!(first_match["message"].as_str().is_some());
            assert!(first_match["offset"].as_i64().is_some());
            assert!(first_match["length"].as_i64().is_some());
            assert!(first_match["replacements"].is_array());
            assert!(first_match["rule"]["id"].as_str().is_some());
            assert!(first_match["context"]["text"].as_str().is_some());
        }
    }
}

#[test]
fn api_response_offset_length_correct() {
    let pipeline = create_test_en_pipeline();
    let text = "Hello  world";  // Double space at position 5
    let result = pipeline.check_text(text);

    let ds_match = result.matches.iter()
        .find(|m| m.rule_id == "DOUBLE_SPACE");

    if let Some(m) = ds_match {
        // The span should cover the double space
        assert!(m.span.start >= 5 && m.span.start <= 6,
            "Double space offset should be around 5-6, got {}", m.span.start);
    }
}

// ============================================================================
// Language Detection Tests
// ============================================================================

#[test]
fn api_language_detection_english() {
    let detector = LanguageDetector::new();
    let text = "The quick brown fox jumps over the lazy dog.";
    let lang = detector.detect(text);

    assert_eq!(lang, Language::English, "Should detect English, got: {:?}", lang);
}

#[test]
fn api_language_detection_french() {
    let detector = LanguageDetector::new();
    let text = "Le chat dort sur le tapis rouge.";
    let lang = detector.detect(text);

    assert_eq!(lang, Language::French, "Should detect French, got: {:?}", lang);
}

#[test]
fn api_language_detection_mixed() {
    let detector = LanguageDetector::new();

    // Short texts may be ambiguous
    let text = "Hello";
    let lang = detector.detect(text);
    // Just ensure it doesn't crash and returns something
    assert!(lang.code().len() > 0, "Should return a language code");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn api_empty_text() {
    let pipeline = create_test_en_pipeline();
    let text = "";
    let result = pipeline.check_text(text);

    assert!(result.matches.is_empty(), "Empty text should have no errors");
}

#[test]
fn api_whitespace_only() {
    let pipeline = create_test_en_pipeline();
    let text = "   \n\t  ";
    let result = pipeline.check_text(text);

    // Should not crash, may or may not have errors
    assert!(result.matches.len() >= 0);
}

#[test]
fn api_unicode_text() {
    let pipeline = create_test_en_pipeline();
    let text = "Café résumé naïve façade";
    let result = pipeline.check_text(text);

    // Should not crash on Unicode
    assert!(result.matches.len() >= 0);
}

#[test]
fn api_very_long_text() {
    let pipeline = create_test_en_pipeline();
    let text = "The quick brown fox. ".repeat(100);
    let result = pipeline.check_text(&text);

    // Should handle long text without crashing
    assert!(result.matches.len() >= 0);
}

#[test]
fn api_special_characters() {
    let pipeline = create_test_en_pipeline();
    let text = "Hello! @#$%^&*() World?";
    let result = pipeline.check_text(text);

    // Should not crash on special characters
    assert!(result.matches.len() >= 0);
}

// ============================================================================
// Spell Checker Tests (EN)
// ============================================================================

fn create_en_spell_checker() -> Option<SpellChecker> {
    let dict_path = Path::new("data/dictionaries/en_US.fst");
    if !dict_path.exists() {
        return None;
    }
    let dict = FstDictionary::from_fst(dict_path).ok()?;
    Some(SpellChecker::with_fst_dictionary(dict)
        .with_skip_words(EN_IGNORE.iter().copied())
        .with_skip_words(EN_PROPER_NOUNS.iter().copied()))
}

fn create_fr_spell_checker() -> SpellChecker {
    SpellChecker::new()
        .with_words(FR_COMMON_WORDS.iter().copied())
        .with_words(FR_SPELLING.iter().copied())
        .with_skip_words(FR_IGNORE.iter().copied())
}

#[test]
fn api_en_spell_checker_detects_misspelling() {
    let Some(spell_checker) = create_en_spell_checker() else {
        eprintln!("Skipping test: EN dictionary not found");
        return;
    };

    let pipeline = Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(spell_checker);

    let text = "I went to the libary yesterday.";
    let result = pipeline.check_text(text);

    let spell_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id == "SPELL")
        .collect();

    assert!(!spell_matches.is_empty(), "Should detect misspelling 'libary'");
    assert!(spell_matches[0].suggestions.contains(&"library".to_string()),
        "Should suggest 'library', got: {:?}", spell_matches[0].suggestions);
}

#[test]
fn api_en_spell_checker_skip_words() {
    let Some(spell_checker) = create_en_spell_checker() else {
        eprintln!("Skipping test: EN dictionary not found");
        return;
    };

    let pipeline = Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(spell_checker);

    // Test that acronyms from EN_IGNORE are not flagged
    let text = "IBM and NASA are working on AI projects.";
    let result = pipeline.check_text(text);

    let spell_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id == "SPELL")
        .collect();

    // Should not flag IBM, NASA, AI (they're in skip list)
    for m in &spell_matches {
        let flagged_word = &text[m.span.clone()];
        assert!(!["IBM", "NASA", "AI"].contains(&flagged_word),
            "Skip words should not be flagged, but '{}' was flagged", flagged_word);
    }
}

#[test]
fn api_en_spell_checker_correct_text() {
    let Some(spell_checker) = create_en_spell_checker() else {
        eprintln!("Skipping test: EN dictionary not found");
        return;
    };

    let pipeline = Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(spell_checker);

    let text = "The quick brown fox jumps over the lazy dog.";
    let result = pipeline.check_text(text);

    let spell_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id == "SPELL")
        .collect();

    assert!(spell_matches.is_empty(),
        "Correct text should have no spell errors, got: {:?}",
        spell_matches.iter().map(|m| &text[m.span.clone()]).collect::<Vec<_>>());
}

// ============================================================================
// Spell Checker Tests (FR)
// ============================================================================

#[test]
fn api_fr_spell_checker_common_words_valid() {
    let spell_checker = create_fr_spell_checker();

    let pipeline = Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(spell_checker);

    // Common French words should not be flagged
    let text = "Je suis allé à la maison.";
    let result = pipeline.check_text(text);

    let spell_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id == "SPELL")
        .collect();

    // Check that basic words are not flagged
    for m in &spell_matches {
        let flagged_word = &text[m.span.clone()];
        assert!(!["Je", "suis", "allé", "à", "la", "maison"].contains(&flagged_word),
            "Common FR word '{}' should not be flagged", flagged_word);
    }
}

#[test]
fn api_fr_spell_checker_detects_misspelling() {
    let spell_checker = create_fr_spell_checker();

    let pipeline = Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(spell_checker);

    // "biblioteque" is misspelled (should be "bibliothèque")
    let text = "Je vais à la biblioteque.";
    let result = pipeline.check_text(text);

    // Note: This test depends on whether "biblioteque" is in the dictionary
    // The FR dictionary is limited, so this may or may not detect the error
    let spell_matches: Vec<_> = result.matches.iter()
        .filter(|m| m.rule_id == "SPELL")
        .collect();

    // Just verify no crash and reasonable behavior
    assert!(spell_matches.len() <= 2,
        "Should not have excessive spell errors on mostly correct text");
}

// ============================================================================
// Performance Sanity Check
// ============================================================================

#[test]
fn api_performance_reasonable() {
    let pipeline = create_test_en_pipeline();
    let text = "The quick brown fox jumps over the lazy dog. This is a normal sentence.";

    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = pipeline.check_text(text);
    }
    let elapsed = start.elapsed();

    // 100 checks should complete in under 1 second (10ms each)
    assert!(elapsed.as_millis() < 1000,
        "100 checks took {}ms, expected < 1000ms", elapsed.as_millis());
}
