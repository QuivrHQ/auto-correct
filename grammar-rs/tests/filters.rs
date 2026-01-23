//! Integration tests for the filter system

use grammar_rs::prelude::*;

/// Test that URLs are filtered out and don't trigger false positives
#[test]
fn test_url_filter_integration() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules())
    .with_default_filters();

    // This URL contains "teh" which could be flagged as a typo,
    // but it should be filtered out
    let result = pipeline.check_text("Check out https://example.com/teh/page for more.");

    // No matches should be in the URL part
    for m in &result.matches {
        let text = "Check out https://example.com/teh/page for more.";
        let matched_text = &text[m.span.clone()];
        assert!(!matched_text.contains("https://"), "URL should be filtered: {}", matched_text);
    }
}

/// Test that code blocks are filtered
#[test]
fn test_code_filter_integration() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules())
    .with_default_filters();

    // Code blocks may contain "incorrect" grammar that's actually code
    let result = pipeline.check_text("Use `teh_function` to fix this.");

    // The inline code should be filtered
    for m in &result.matches {
        let text = "Use `teh_function` to fix this.";
        let matched_text = &text[m.span.clone()];
        assert!(!matched_text.contains("teh_function"), "Code should be filtered: {}", matched_text);
    }
}

/// Test that quoted text is filtered
#[test]
fn test_quoted_filter_integration() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules())
    .with_default_filters();

    // Quoted text should be preserved as-is
    let result = pipeline.check_text(r#"He said "teh quick brown fox" to everyone."#);

    // The quoted part should be filtered
    for m in &result.matches {
        let text = r#"He said "teh quick brown fox" to everyone."#;
        let matched_text = &text[m.span.clone()];
        // If any match is found, it shouldn't be inside the quotes
        assert!(!matched_text.contains("quick"), "Quoted text should be filtered: {}", matched_text);
    }
}

/// Test that dates are filtered
#[test]
fn test_date_filter_integration() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules())
    .with_default_filters();

    // Dates should not trigger grammar errors
    let result = pipeline.check_text("Meeting on 2024-01-15 at noon.");

    // The date should be filtered
    for m in &result.matches {
        let text = "Meeting on 2024-01-15 at noon.";
        let matched_text = &text[m.span.clone()];
        assert!(!matched_text.contains("2024"), "Date should be filtered: {}", matched_text);
    }
}

/// Test that email addresses are filtered
#[test]
fn test_email_filter_integration() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules())
    .with_default_filters();

    let result = pipeline.check_text("Contact support@example.com for help.");

    // The email should be filtered
    for m in &result.matches {
        let text = "Contact support@example.com for help.";
        let matched_text = &text[m.span.clone()];
        assert!(!matched_text.contains("@"), "Email should be filtered: {}", matched_text);
    }
}

/// Test that hyphenated numbers are filtered
#[test]
fn test_number_filter_integration() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules())
    .with_default_filters();

    // Hyphenated numbers should not trigger errors
    let result = pipeline.check_text("There are twenty-one items in the list.");

    // The hyphenated number should be filtered
    for m in &result.matches {
        let text = "There are twenty-one items in the list.";
        let matched_text = &text[m.span.clone()];
        assert!(!matched_text.contains("twenty-one"), "Hyphenated number should be filtered: {}", matched_text);
    }
}

/// Test filter builder
#[test]
fn test_filter_builder() {
    let filters = FilterBuilder::new()
        .with_url_filter()
        .with_code_filter()
        .build();

    let masks = filters.find_all_masks("Check https://example.com and `code` here.");
    assert_eq!(masks.len(), 2);
}

/// Test pipeline without filters still works
#[test]
fn test_pipeline_without_filters() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules());

    // Without filters, the pipeline should still work
    let result = pipeline.check_text("This is a test.");
    // Just verify it doesn't crash
    let _ = result.matches.len();
}

/// Test combining all filters
#[test]
fn test_all_filters_combined() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules())
    .with_default_filters();

    let text = r#"
        Visit https://example.com for info.
        Email us at test@example.com.
        Meeting on January 15th, 2024.
        Use `cargo build` to compile.
        She said "hello world" to him.
        There are twenty-one items.
    "#;

    let result = pipeline.check_text(text);

    // Verify no matches are in filtered regions
    for m in &result.matches {
        let matched_text = &text[m.span.clone()];
        assert!(!matched_text.contains("https://"), "URL should be filtered");
        assert!(!matched_text.contains("@example.com"), "Email should be filtered");
        assert!(!matched_text.contains("cargo"), "Code should be filtered");
        assert!(!matched_text.contains("twenty-one"), "Number should be filtered");
    }
}

/// Test French filters
#[test]
fn test_french_filters() {
    let pipeline = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_french_rules())
    .with_default_filters();

    let text = "Il a dit « bonjour » et envoyé un email à contact@example.fr.";

    let result = pipeline.check_text(text);

    // Verify guillemets and email are filtered
    for m in &result.matches {
        let matched_text = &text[m.span.clone()];
        assert!(!matched_text.contains("bonjour"), "Guillemets content should be filtered");
        assert!(!matched_text.contains("@"), "Email should be filtered");
    }
}

/// Test MaskedRegion::overlaps
#[test]
fn test_masked_region_overlaps() {
    let region = MaskedRegion::new(10..20, MaskKind::Url);

    // Test overlapping cases
    assert!(region.overlaps(&(15..25)));  // Overlap at end
    assert!(region.overlaps(&(5..15)));   // Overlap at start
    assert!(region.overlaps(&(12..18)));  // Inside
    assert!(region.overlaps(&(5..25)));   // Contains

    // Test non-overlapping cases
    assert!(!region.overlaps(&(0..10)));  // Before (adjacent)
    assert!(!region.overlaps(&(20..30))); // After (adjacent)
    assert!(!region.overlaps(&(0..5)));   // Before (gap)
    assert!(!region.overlaps(&(25..30))); // After (gap)
}
