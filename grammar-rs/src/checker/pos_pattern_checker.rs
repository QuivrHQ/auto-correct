//! POS Pattern Rule Checker
//!
//! This module provides a checker for rules that require POS tag matching.
//! Rules can specify patterns like:
//! - "NN.*" - any noun
//! - "VB.*" - any verb form
//! - "JJ" - adjective
//!
//! This unlocks ~5000+ additional rules from LanguageTool that require POS tagging.

use crate::core::traits::Checker;
use crate::core::{AnalyzedToken, CheckResult, Match, PosTag, Severity};

/// A pattern element that can match a token (static version for const arrays)
#[derive(Debug, Clone, Copy)]
pub struct PosPatternElement {
    /// Literal text to match (optional, case-insensitive)
    pub text: Option<&'static str>,
    /// POS tag pattern to match (e.g., "NN.*", "VB", etc.)
    pub pos_pattern: Option<&'static str>,
    /// Whether to negate the match
    pub negation: bool,
}

impl PosPatternElement {
    /// Check if this element matches a token
    pub fn matches(&self, token: &AnalyzedToken) -> bool {
        let text_match = self.text.map_or(true, |t| {
            token.token.text.eq_ignore_ascii_case(t)
        });

        let pos_match = self.pos_pattern.map_or(true, |pattern| {
            token.pos.as_ref().map_or(false, |pos| pos.matches_pattern(pattern))
        });

        let result = text_match && pos_match;
        if self.negation { !result } else { result }
    }
}

/// A rule with POS-based pattern matching (static version for const arrays)
#[derive(Debug, Clone, Copy)]
pub struct PosPatternRule {
    pub id: &'static str,
    pub pattern: &'static [PosPatternElement],
    pub message: &'static str,
    pub suggestions: &'static [&'static str],
}

/// Checker for POS-based pattern rules
pub struct PosPatternChecker {
    rules: &'static [PosPatternRule],
}

impl PosPatternChecker {
    pub fn new() -> Self {
        Self { rules: &[] }
    }

    /// Create a checker with the given static rules
    pub fn with_rules(rules: &'static [PosPatternRule]) -> Self {
        Self { rules }
    }

    /// Number of rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Check if a sequence of tokens matches a pattern starting at position
    fn matches_pattern_at(&self, tokens: &[AnalyzedToken], start: usize, pattern: &[PosPatternElement]) -> bool {
        if start + pattern.len() > tokens.len() {
            return false;
        }

        for (i, element) in pattern.iter().enumerate() {
            if !element.matches(&tokens[start + i]) {
                return false;
            }
        }

        true
    }

    /// Check if a sequence of word tokens (non-whitespace) matches a pattern
    fn matches_word_pattern_at(
        &self,
        word_tokens: &[(usize, &AnalyzedToken)],
        start: usize,
        pattern: &[PosPatternElement],
    ) -> bool {
        if start + pattern.len() > word_tokens.len() {
            return false;
        }

        for (i, element) in pattern.iter().enumerate() {
            if !element.matches(word_tokens[start + i].1) {
                return false;
            }
        }

        true
    }
}

impl Default for PosPatternChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker for PosPatternChecker {
    fn check(&self, _text: &str, tokens: &[AnalyzedToken]) -> CheckResult {
        let mut matches = Vec::new();

        // Filter out whitespace tokens for pattern matching
        let word_tokens: Vec<(usize, &AnalyzedToken)> = tokens
            .iter()
            .enumerate()
            .filter(|(_, t)| t.token.kind != crate::core::TokenKind::Whitespace)
            .collect();

        for rule in self.rules {
            // Slide the pattern across all word positions
            for start in 0..word_tokens.len() {
                if self.matches_word_pattern_at(&word_tokens, start, rule.pattern) {
                    // Calculate span from first to last matched token
                    let end = start + rule.pattern.len();
                    let span_start = word_tokens[start].1.token.span.start;
                    let span_end = word_tokens[end - 1].1.token.span.end;

                    matches.push(Match {
                        span: span_start..span_end,
                        message: rule.message.to_string(),
                        rule_id: rule.id.to_string(),
                        suggestions: rule.suggestions.iter().map(|s| s.to_string()).collect(),
                        severity: Severity::Warning,
                    });
                }
            }
        }

        CheckResult { matches }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Token, TokenKind};

    fn make_analyzed_token(text: &str, pos: Option<PosTag>) -> AnalyzedToken {
        AnalyzedToken {
            token: Token {
                text,
                span: 0..text.len(),
                kind: TokenKind::Word,
            },
            lemma: None,
            pos,
        }
    }

    #[test]
    fn test_pos_pattern_element_matches_text() {
        let element = PosPatternElement {
            text: Some("the"),
            pos_pattern: None,
            negation: false,
        };

        let token = make_analyzed_token("the", Some(PosTag::DT));
        assert!(element.matches(&token));

        let token2 = make_analyzed_token("a", Some(PosTag::DT));
        assert!(!element.matches(&token2));
    }

    #[test]
    fn test_pos_pattern_element_matches_pos() {
        let element = PosPatternElement {
            text: None,
            pos_pattern: Some("NN.*"),
            negation: false,
        };

        let token_nn = make_analyzed_token("dog", Some(PosTag::NN));
        assert!(element.matches(&token_nn));

        let token_nns = make_analyzed_token("dogs", Some(PosTag::NNS));
        assert!(element.matches(&token_nns));

        let token_vb = make_analyzed_token("run", Some(PosTag::VB));
        assert!(!element.matches(&token_vb));
    }

    #[test]
    fn test_pos_pattern_element_negation() {
        let element = PosPatternElement {
            text: None,
            pos_pattern: Some("VB.*"),
            negation: true,
        };

        let token_nn = make_analyzed_token("dog", Some(PosTag::NN));
        assert!(element.matches(&token_nn)); // Not a verb, so matches

        let token_vb = make_analyzed_token("run", Some(PosTag::VB));
        assert!(!element.matches(&token_vb)); // Is a verb, so doesn't match
    }

    // Static test rule definitions for testing
    static TEST_DT_NN_PATTERN: &[PosPatternElement] = &[
        PosPatternElement { text: None, pos_pattern: Some("DT"), negation: false },
        PosPatternElement { text: None, pos_pattern: Some("NN"), negation: false },
    ];

    static TEST_DT_NN_RULE: PosPatternRule = PosPatternRule {
        id: "DT_NN",
        pattern: TEST_DT_NN_PATTERN,
        message: "Detected determiner + noun",
        suggestions: &[],
    };

    static TEST_DT_NN_RULES: &[PosPatternRule] = &[TEST_DT_NN_RULE];

    static TEST_DT_VB_PATTERN: &[PosPatternElement] = &[
        PosPatternElement { text: None, pos_pattern: Some("DT"), negation: false },
        PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
    ];

    static TEST_DT_VB_RULE: PosPatternRule = PosPatternRule {
        id: "DT_VB",
        pattern: TEST_DT_VB_PATTERN,
        message: "Detected determiner + verb",
        suggestions: &[],
    };

    static TEST_DT_VB_RULES: &[PosPatternRule] = &[TEST_DT_VB_RULE];

    #[test]
    fn test_pos_pattern_checker_basic() {
        let checker = PosPatternChecker::with_rules(TEST_DT_NN_RULES);

        // Create tokens: "the dog" with whitespace between
        let tokens = vec![
            AnalyzedToken {
                token: Token { text: "the", span: 0..3, kind: TokenKind::Word },
                lemma: None,
                pos: Some(PosTag::DT),
            },
            AnalyzedToken {
                token: Token { text: " ", span: 3..4, kind: TokenKind::Whitespace },
                lemma: None,
                pos: None,
            },
            AnalyzedToken {
                token: Token { text: "dog", span: 4..7, kind: TokenKind::Word },
                lemma: None,
                pos: Some(PosTag::NN),
            },
        ];

        let result = checker.check("the dog", &tokens);

        // Now matches because whitespace is skipped during pattern matching
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].rule_id, "DT_NN");
        assert_eq!(result.matches[0].span, 0..7); // "the dog"
    }

    #[test]
    fn test_pos_pattern_checker_no_match() {
        let checker = PosPatternChecker::with_rules(TEST_DT_VB_RULES);

        // Create tokens: "the dog" - should NOT match DT_VB pattern
        let tokens = vec![
            AnalyzedToken {
                token: Token { text: "the", span: 0..3, kind: TokenKind::Word },
                lemma: None,
                pos: Some(PosTag::DT),
            },
            AnalyzedToken {
                token: Token { text: " ", span: 3..4, kind: TokenKind::Whitespace },
                lemma: None,
                pos: None,
            },
            AnalyzedToken {
                token: Token { text: "dog", span: 4..7, kind: TokenKind::Word },
                lemma: None,
                pos: Some(PosTag::NN),
            },
        ];

        let result = checker.check("the dog", &tokens);
        assert_eq!(result.matches.len(), 0);
    }
}
