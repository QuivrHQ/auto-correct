//! Code block filter

use crate::core::filter::Filter;
use crate::core::{MaskKind, MaskedRegion};
use regex::Regex;

/// Filter that masks inline code and code blocks
pub struct CodeBlockFilter {
    // Triple backtick code blocks (multiline)
    triple_backtick: Regex,
    // Single backtick inline code
    single_backtick: Regex,
}

impl CodeBlockFilter {
    pub fn new() -> Self {
        Self {
            // Triple backticks with optional language identifier
            triple_backtick: Regex::new(
                r"```(?:\w+)?\s*[\s\S]*?```",
            ).expect("Invalid triple backtick regex"),
            // Single backticks for inline code
            single_backtick: Regex::new(
                r"`[^`\n]+`",
            ).expect("Invalid single backtick regex"),
        }
    }
}

impl Default for CodeBlockFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl Filter for CodeBlockFilter {
    fn find_masks(&self, text: &str) -> Vec<MaskedRegion> {
        let mut masks = Vec::new();

        // Find triple backtick blocks first (they take precedence)
        for m in self.triple_backtick.find_iter(text) {
            masks.push(MaskedRegion::new(m.start()..m.end(), MaskKind::CodeBlock));
        }

        // Find single backtick inline code (only if not inside triple backticks)
        for m in self.single_backtick.find_iter(text) {
            let span = m.start()..m.end();
            // Check if this span overlaps with any triple backtick block
            let overlaps = masks.iter().any(|mask| mask.overlaps(&span));
            if !overlaps {
                masks.push(MaskedRegion::new(span, MaskKind::CodeBlock));
            }
        }

        // Sort by start position
        masks.sort_by_key(|m| m.span.start);
        masks
    }

    fn description(&self) -> &'static str {
        "Code blocks and inline code"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_code() {
        let filter = CodeBlockFilter::new();
        let masks = filter.find_masks("Use the `println!` macro for output.");
        assert_eq!(masks.len(), 1);
        assert_eq!(masks[0].kind, MaskKind::CodeBlock);
        assert_eq!(&"Use the `println!` macro for output."[masks[0].span.clone()], "`println!`");
    }

    #[test]
    fn test_code_block() {
        let filter = CodeBlockFilter::new();
        let text = "Example:\n```rust\nfn main() {}\n```\nDone.";
        let masks = filter.find_masks(text);
        assert_eq!(masks.len(), 1);
        assert!(text[masks[0].span.clone()].starts_with("```"));
        assert!(text[masks[0].span.clone()].ends_with("```"));
    }

    #[test]
    fn test_multiple_inline_codes() {
        let filter = CodeBlockFilter::new();
        let masks = filter.find_masks("Use `foo` and `bar` together.");
        assert_eq!(masks.len(), 2);
    }

    #[test]
    fn test_no_code() {
        let filter = CodeBlockFilter::new();
        let masks = filter.find_masks("This is normal text.");
        assert!(masks.is_empty());
    }

    #[test]
    fn test_unclosed_backtick() {
        let filter = CodeBlockFilter::new();
        // Unclosed backticks should not match
        let masks = filter.find_masks("This has an unclosed `backtick");
        assert!(masks.is_empty());
    }
}
