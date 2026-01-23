//! Style checker using Aho-Corasick for efficient multi-phrase matching
//!
//! This module implements a checker for wordiness and redundancy rules
//! imported from LanguageTool. It uses Aho-Corasick for O(N) scanning
//! of text regardless of the number of rules.

use aho_corasick::AhoCorasick;

use crate::checker::data::en_style::{StyleCategory, StyleRule, EN_STYLE_RULES};
use crate::checker::data::fr_style::FR_STYLE_RULES;
use crate::core::traits::Checker;
use crate::core::{AnalyzedToken, CheckResult, Match, Severity};

/// Style checker using Aho-Corasick for efficient phrase matching.
///
/// Detects wordy and redundant phrases from LanguageTool's wordiness.txt
/// and redundancies.txt files.
pub struct StyleChecker {
    /// Aho-Corasick automaton for all style phrases
    ac: AhoCorasick,
    /// Reference to the style rules (indexed by AC pattern ID)
    rules: &'static [StyleRule],
}

impl StyleChecker {
    /// Create a new StyleChecker with the default English style rules.
    pub fn new() -> Self {
        Self::with_rules(EN_STYLE_RULES)
    }

    /// Create a new StyleChecker with the French style rules.
    pub fn french() -> Self {
        Self::with_rules(FR_STYLE_RULES)
    }

    /// Create a new StyleChecker with custom rules.
    pub fn with_rules(rules: &'static [StyleRule]) -> Self {
        let phrases: Vec<&str> = rules.iter().map(|r| r.phrase).collect();

        let ac = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(&phrases)
            .expect("Failed to build Aho-Corasick automaton");

        Self { ac, rules }
    }

    /// Get the rule ID based on category and phrase
    fn get_rule_id(&self, rule: &StyleRule) -> String {
        match rule.category {
            StyleCategory::Wordiness => format!("WORDINESS_{}", self.phrase_to_id(rule.phrase)),
            StyleCategory::Redundancy => format!("REDUNDANCY_{}", self.phrase_to_id(rule.phrase)),
        }
    }

    /// Convert a phrase to a valid rule ID component
    fn phrase_to_id(&self, phrase: &str) -> String {
        phrase
            .to_uppercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>()
            .trim_matches('_')
            .to_string()
    }

    /// Generate a message for a style rule
    fn get_message(&self, rule: &StyleRule) -> String {
        match rule.category {
            StyleCategory::Wordiness => {
                if rule.suggestions.is_empty() || rule.suggestions[0].is_empty() {
                    format!("'{}' is wordy. Consider removing it.", rule.phrase)
                } else {
                    format!(
                        "'{}' is wordy. Consider using '{}' instead.",
                        rule.phrase, rule.suggestions[0]
                    )
                }
            }
            StyleCategory::Redundancy => {
                if rule.suggestions.is_empty() || rule.suggestions[0].is_empty() {
                    format!("'{}' is redundant. Consider removing it.", rule.phrase)
                } else {
                    format!(
                        "'{}' is redundant. Consider using '{}' instead.",
                        rule.phrase, rule.suggestions[0]
                    )
                }
            }
        }
    }

    /// Check if a match is at word boundaries
    fn is_word_boundary(&self, text: &str, start: usize, end: usize) -> bool {
        // Check start boundary
        let start_ok = start == 0 || {
            let prev_char = text[..start].chars().last().unwrap_or(' ');
            !prev_char.is_alphanumeric()
        };

        // Check end boundary
        let end_ok = end >= text.len() || {
            let next_char = text[end..].chars().next().unwrap_or(' ');
            !next_char.is_alphanumeric()
        };

        start_ok && end_ok
    }
}

impl Default for StyleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker for StyleChecker {
    fn check(&self, text: &str, _tokens: &[AnalyzedToken]) -> CheckResult {
        let mut matches = Vec::new();
        let text_lower = text.to_lowercase();

        for ac_match in self.ac.find_iter(&text_lower) {
            let pattern_idx = ac_match.pattern().as_usize();
            let rule = &self.rules[pattern_idx];

            // Verify word boundaries
            if !self.is_word_boundary(&text_lower, ac_match.start(), ac_match.end()) {
                continue;
            }

            let span = ac_match.start()..ac_match.end();
            let suggestions: Vec<String> = rule
                .suggestions
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            matches.push(Match {
                span,
                rule_id: self.get_rule_id(rule),
                message: self.get_message(rule),
                suggestions,
                severity: Severity::Hint, // Style issues are hints
            });
        }

        CheckResult { matches }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordiness_detection() {
        let checker = StyleChecker::new();
        let text = "In order to succeed, you need a number of things.";
        let tokens = vec![]; // StyleChecker doesn't use tokens

        let result = checker.check(text, &tokens);

        // Should detect "in order to" and "a number of"
        assert!(result.matches.len() >= 1, "Expected at least 1 match");

        let rule_ids: Vec<_> = result.matches.iter().map(|m| &m.rule_id).collect();
        assert!(
            rule_ids.iter().any(|id| id.contains("WORDINESS")),
            "Expected wordiness rule: {:?}",
            rule_ids
        );
    }

    #[test]
    fn test_redundancy_detection() {
        let checker = StyleChecker::new();
        // "12 noon" is definitely a redundancy
        let text = "Meet me at 12 noon for lunch.";
        let tokens = vec![];

        let result = checker.check(text, &tokens);

        // Should detect "12 noon"
        assert!(result.matches.len() >= 1, "Expected at least 1 match");

        let has_redundancy = result.matches.iter().any(|m| m.rule_id.contains("REDUNDANCY"));
        assert!(has_redundancy, "Expected redundancy rule, got: {:?}",
            result.matches.iter().map(|m| &m.rule_id).collect::<Vec<_>>());
    }

    #[test]
    fn test_case_insensitive() {
        let checker = StyleChecker::new();
        let text = "ABSOLUTELY ESSENTIAL for success.";
        let tokens = vec![];

        let result = checker.check(text, &tokens);
        assert!(!result.matches.is_empty(), "Should match case-insensitively");
    }

    #[test]
    fn test_word_boundaries() {
        let checker = StyleChecker::new();

        // "12 noon" should be detected
        let result1 = checker.check("Meet at 12 noon tomorrow.", &[]);
        let has_noon = result1.matches.iter().any(|m| m.rule_id.contains("12_NOON"));
        assert!(has_noon, "Should detect '12 noon'");

        // But "noon" inside another word shouldn't match "12 noon"
        let result2 = checker.check("The afternoon is nice.", &[]);
        let false_match = result2
            .matches
            .iter()
            .any(|m| m.rule_id.contains("12_NOON"));
        assert!(!false_match, "Should not match 'noon' in 'afternoon'");
    }

    #[test]
    fn test_suggestions() {
        let checker = StyleChecker::new();
        let text = "a number of people";
        let tokens = vec![];

        let result = checker.check(text, &tokens);
        assert!(!result.matches.is_empty());

        let first_match = &result.matches[0];
        assert!(
            !first_match.suggestions.is_empty(),
            "Should have suggestions"
        );
    }

    #[test]
    fn test_no_false_positives() {
        let checker = StyleChecker::new();
        let text = "The quick brown fox jumps over the lazy dog.";
        let tokens = vec![];

        let result = checker.check(text, &tokens);
        assert!(
            result.matches.is_empty(),
            "No matches expected for clean text"
        );
    }

    #[test]
    fn test_multiple_matches() {
        let checker = StyleChecker::new();
        let text = "In order to get the added bonus, it is absolutely essential.";
        let tokens = vec![];

        let result = checker.check(text, &tokens);
        assert!(
            result.matches.len() >= 2,
            "Expected at least 2 matches, got {}",
            result.matches.len()
        );
    }

    #[test]
    fn test_french_redundancy_detection() {
        let checker = StyleChecker::french();
        // "monter en haut" is a classic French pleonasm
        let text = "Il faut monter en haut de la tour.";
        let tokens = vec![];

        let result = checker.check(text, &tokens);
        assert!(result.matches.len() >= 1, "Should detect French redundancy 'monter en haut'");

        let has_redundancy = result.matches.iter().any(|m| m.rule_id.contains("REDUNDANCY"));
        assert!(has_redundancy, "Expected redundancy rule, got: {:?}",
            result.matches.iter().map(|m| &m.rule_id).collect::<Vec<_>>());
    }

    #[test]
    fn test_french_pleonasm_descendre_en_bas() {
        let checker = StyleChecker::french();
        // Use the exact phrase from the rules
        let text = "Il faut descendre en bas pour ouvrir la porte.";
        let tokens = vec![];

        let result = checker.check(text, &tokens);
        assert!(!result.matches.is_empty(), "Should detect 'descendre en bas'");
    }

    #[test]
    fn test_french_no_false_positives() {
        let checker = StyleChecker::french();
        let text = "Le chat monte sur la table et descend par l'escalier.";
        let tokens = vec![];

        let result = checker.check(text, &tokens);
        assert!(result.matches.is_empty(), "No matches expected for clean French text");
    }
}
