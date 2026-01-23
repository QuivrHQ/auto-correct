//! Compound word checker
//!
//! Detects compound word errors:
//! - Spaced compounds that should be hyphenated: "well being" → "well-being"
//! - Spaced compounds that should be joined: "air plane" → "airplane"
//! - Hyphenated words that should be joined: "air-plane" → "airplane"

use crate::core::{AnalyzedToken, CheckResult, Match, Severity, TokenKind};
use crate::core::traits::Checker;
use crate::checker::data::en_compounds::{EN_COMPOUND_RULES, CompoundRule, get_en_compound};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Lookup table for compound words by their first component
static COMPOUND_FIRST_WORD: LazyLock<HashMap<String, Vec<&'static CompoundRule>>> = LazyLock::new(|| {
    let mut map: HashMap<String, Vec<&'static CompoundRule>> = HashMap::new();

    for rule in EN_COMPOUND_RULES {
        if let Some(first) = rule.word.split('-').next() {
            let first_lower = first.to_lowercase();
            map.entry(first_lower).or_default().push(rule);
        }
    }

    map
});

pub struct CompoundWordChecker;

impl CompoundWordChecker {
    pub fn new() -> Self {
        Self
    }

    /// Check if two consecutive words form a compound
    fn check_spaced_compound(&self, word1: &str, word2: &str) -> Option<(String, String)> {
        let combined_hyphen = format!("{}-{}", word1.to_lowercase(), word2.to_lowercase());

        // Check if this hyphenated form exists in our rules
        if let Some(rule) = get_en_compound(&combined_hyphen) {
            let suggestion = if rule.lowercase_joined {
                // Suggest joined form
                combined_hyphen.replace("-", "")
            } else {
                // Suggest hyphenated form
                combined_hyphen.clone()
            };

            let message = if rule.lowercase_joined {
                format!("'{}' should be written as one word: '{}'",
                        format!("{} {}", word1, word2), suggestion)
            } else {
                format!("'{}' should be hyphenated: '{}'",
                        format!("{} {}", word1, word2), suggestion)
            };

            return Some((suggestion, message));
        }

        None
    }

    /// Check if a hyphenated word should be joined
    fn check_hyphenated(&self, word: &str) -> Option<(String, String)> {
        let lower = word.to_lowercase();

        if let Some(rule) = get_en_compound(&lower) {
            if rule.lowercase_joined {
                let joined = lower.replace("-", "");
                let message = format!("'{}' is typically written as one word: '{}'", word, joined);
                return Some((joined, message));
            }
        }

        None
    }
}

impl Default for CompoundWordChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker for CompoundWordChecker {
    fn check(&self, _text: &str, tokens: &[AnalyzedToken]) -> CheckResult {
        let mut result = CheckResult::new();
        let mut skip_next = false;

        for i in 0..tokens.len() {
            if skip_next {
                skip_next = false;
                continue;
            }

            let token = &tokens[i].token;

            // Skip non-words
            if token.kind != TokenKind::Word {
                continue;
            }

            let word_lower = token.text.to_lowercase();

            // Check for hyphenated words that should be joined
            if token.text.contains('-') {
                if let Some((suggestion, message)) = self.check_hyphenated(token.text) {
                    result.matches.push(Match {
                        span: token.span.clone(),
                        message,
                        rule_id: "COMPOUND_HYPHEN".to_string(),
                        suggestions: vec![suggestion],
                        severity: Severity::Hint,
                    });
                }
                continue;
            }

            // Check if this word could be the first part of a compound
            if !COMPOUND_FIRST_WORD.contains_key(&word_lower) {
                continue;
            }

            // Look for the next word
            let mut next_idx = i + 1;
            while next_idx < tokens.len() && tokens[next_idx].token.kind != TokenKind::Word {
                // Check for hyphen between words
                if tokens[next_idx].token.text == "-" {
                    // Already hyphenated, skip
                    continue;
                }
                next_idx += 1;
            }

            if next_idx < tokens.len() {
                let next_token = &tokens[next_idx].token;
                if next_token.kind == TokenKind::Word {
                    // Check if these two words form a compound
                    if let Some((suggestion, message)) = self.check_spaced_compound(token.text, next_token.text) {
                        // Create a span covering both words
                        let combined_span = token.span.start..next_token.span.end;
                        result.matches.push(Match {
                            span: combined_span,
                            message,
                            rule_id: "COMPOUND_SPACE".to_string(),
                            suggestions: vec![suggestion],
                            severity: Severity::Hint,
                        });
                        skip_next = true;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::SimpleTokenizer;
    use crate::analyzer::PassthroughAnalyzer;
    use crate::core::traits::{Tokenizer, Analyzer};

    fn check_text(text: &str) -> CheckResult {
        let checker = CompoundWordChecker::new();
        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();
        let tokens = tokenizer.tokenize(text);
        let analyzed = analyzer.analyze(tokens);
        checker.check(text, &analyzed)
    }

    #[test]
    fn test_spaced_compound_join() {
        // "air plane" should be "airplane"
        let result = check_text("The air plane landed safely.");
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].suggestions[0], "airplane");
    }

    #[test]
    fn test_hyphenated_should_join() {
        // "air-plane" should be "airplane"
        let result = check_text("The air-plane landed.");
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].suggestions[0], "airplane");
    }

    #[test]
    fn test_spaced_compound_hyphenate() {
        // "well being" should be "well-being"
        let result = check_text("Your well being matters.");
        assert_eq!(result.matches.len(), 1);
        assert!(result.matches[0].suggestions[0].contains("-"));
    }

    #[test]
    fn test_no_false_positive() {
        // Regular words should not trigger
        let result = check_text("The cat sat on the mat.");
        assert_eq!(result.matches.len(), 0);
    }

    #[test]
    fn test_correct_compound() {
        // Already correct compound should not trigger
        let result = check_text("The airplane landed.");
        assert_eq!(result.matches.len(), 0);
    }
}
