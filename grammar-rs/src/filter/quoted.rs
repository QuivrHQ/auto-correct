//! Quoted text filter

use crate::core::filter::Filter;
use crate::core::{MaskKind, MaskedRegion};
use regex::Regex;

/// Filter that masks quoted text in various formats
pub struct QuotedTextFilter {
    // Double quotes: "text"
    double_quotes: Regex,
    // Single quotes: 'text' (only if likely a quote, not contractions)
    single_quotes: Regex,
    // French guillemets: «text» or « text »
    guillemets: Regex,
    // Smart/curly quotes: "text" or 'text'
    smart_double: Regex,
    smart_single: Regex,
}

impl QuotedTextFilter {
    pub fn new() -> Self {
        Self {
            // Standard double quotes - greedy but non-crossing lines for short quotes
            double_quotes: Regex::new(
                r#""[^"]*""#,
            ).expect("Invalid double quotes regex"),
            // Single quotes - only match if it looks like a quote (has spaces or multiple words)
            // This avoids matching contractions like don't
            single_quotes: Regex::new(
                r"'[^']*\s[^']*'",
            ).expect("Invalid single quotes regex"),
            // French guillemets
            guillemets: Regex::new(
                r"«\s*[^»]*\s*»",
            ).expect("Invalid guillemets regex"),
            // Smart double quotes (Unicode: U+201C and U+201D)
            smart_double: Regex::new(
                "[\u{201C}\u{201D}][^\u{201C}\u{201D}]*[\u{201C}\u{201D}]",
            ).expect("Invalid smart double quotes regex"),
            // Smart single quotes (Unicode: U+2018 and U+2019)
            smart_single: Regex::new(
                "[\u{2018}\u{2019}][^\u{2018}\u{2019}]*[\u{2018}\u{2019}]",
            ).expect("Invalid smart single quotes regex"),
        }
    }
}

impl Default for QuotedTextFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl Filter for QuotedTextFilter {
    fn find_masks(&self, text: &str) -> Vec<MaskedRegion> {
        let mut masks = Vec::new();

        // Find double-quoted text
        for m in self.double_quotes.find_iter(text) {
            masks.push(MaskedRegion::new(m.start()..m.end(), MaskKind::QuotedText));
        }

        // Find single-quoted text (only multi-word quotes)
        for m in self.single_quotes.find_iter(text) {
            let span = m.start()..m.end();
            if !masks.iter().any(|mask| mask.overlaps(&span)) {
                masks.push(MaskedRegion::new(span, MaskKind::QuotedText));
            }
        }

        // Find guillemets
        for m in self.guillemets.find_iter(text) {
            let span = m.start()..m.end();
            if !masks.iter().any(|mask| mask.overlaps(&span)) {
                masks.push(MaskedRegion::new(span, MaskKind::QuotedText));
            }
        }

        // Find smart double quotes
        for m in self.smart_double.find_iter(text) {
            let span = m.start()..m.end();
            if !masks.iter().any(|mask| mask.overlaps(&span)) {
                masks.push(MaskedRegion::new(span, MaskKind::QuotedText));
            }
        }

        // Find smart single quotes
        for m in self.smart_single.find_iter(text) {
            let span = m.start()..m.end();
            if !masks.iter().any(|mask| mask.overlaps(&span)) {
                masks.push(MaskedRegion::new(span, MaskKind::QuotedText));
            }
        }

        // Sort by start position
        masks.sort_by_key(|m| m.span.start);
        masks
    }

    fn description(&self) -> &'static str {
        "Quoted text"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_quotes() {
        let filter = QuotedTextFilter::new();
        let masks = filter.find_masks(r#"He said "hello world" to her."#);
        assert_eq!(masks.len(), 1);
        assert_eq!(masks[0].kind, MaskKind::QuotedText);
        assert_eq!(&r#"He said "hello world" to her."#[masks[0].span.clone()], r#""hello world""#);
    }

    #[test]
    fn test_guillemets() {
        let filter = QuotedTextFilter::new();
        let masks = filter.find_masks("Il a dit « bonjour » à tous.");
        assert_eq!(masks.len(), 1);
        assert_eq!(masks[0].kind, MaskKind::QuotedText);
    }

    #[test]
    fn test_smart_quotes() {
        let filter = QuotedTextFilter::new();
        // Using Unicode smart quotes directly
        let masks = filter.find_masks("She said \u{201C}yes\u{201D} immediately.");
        assert_eq!(masks.len(), 1);
        assert_eq!(masks[0].kind, MaskKind::QuotedText);
    }

    #[test]
    fn test_single_quotes_with_spaces() {
        let filter = QuotedTextFilter::new();
        let masks = filter.find_masks("The title 'War and Peace' is famous.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_contractions_not_matched() {
        let filter = QuotedTextFilter::new();
        // Single word contractions should not be matched
        let masks = filter.find_masks("Don't match contractions.");
        assert!(masks.is_empty());
    }

    #[test]
    fn test_multiple_quotes() {
        let filter = QuotedTextFilter::new();
        let masks = filter.find_masks(r#""First" and "second" quotes."#);
        assert_eq!(masks.len(), 2);
    }

    #[test]
    fn test_no_quotes() {
        let filter = QuotedTextFilter::new();
        let masks = filter.find_masks("This text has no quotes.");
        assert!(masks.is_empty());
    }
}
