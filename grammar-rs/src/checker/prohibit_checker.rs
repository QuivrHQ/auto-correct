//! Prohibited word checker
//!
//! Flags words/phrases that are always wrong and should be replaced.
//! Examples: "1-moth" → "1-month", "GDPR-complaint" → "GDPR-compliant"

use crate::core::{AnalyzedToken, CheckResult, Match, Severity, TokenKind};
use crate::core::traits::Checker;
use crate::checker::data::en_prohibit::{EN_PROHIBIT, is_en_prohibit};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Map from prohibited word to suggested replacement
static EN_PROHIBIT_SUGGESTIONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Common replacements based on patterns
    for word in EN_PROHIBIT {
        let suggestion = suggest_replacement(word);
        if let Some(s) = suggestion {
            map.insert(*word, s);
        }
    }

    map
});

/// Generate replacement suggestion for a prohibited word
fn suggest_replacement(word: &str) -> Option<&'static str> {
    // Pattern: X-moth(s) → X-month(s)
    if word.ends_with("-moth") {
        return None; // Would need dynamic string, skip for now
    }
    if word.ends_with("-moths") {
        return None;
    }

    // Known specific replacements
    match word {
        "Christoper" => Some("Christopher"),
        "GDPR-complaint" => Some("GDPR-compliant"),
        "HIPAA-complaint" => Some("HIPAA-compliant"),
        "HIPPA-complaint" => Some("HIPAA-compliant"),
        "HIPPA-compliant" => Some("HIPAA-compliant"),
        "PCI-complaint" => Some("PCI-compliant"),
        "Nescafe" => Some("Nescafé"),
        "Hanuka" => Some("Hanukkah"),
        _ => None,
    }
}

pub struct ProhibitChecker;

impl ProhibitChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProhibitChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker for ProhibitChecker {
    fn check(&self, _text: &str, tokens: &[AnalyzedToken]) -> CheckResult {
        let mut result = CheckResult::new();

        for analyzed in tokens {
            let token = &analyzed.token;

            // Only check words
            if token.kind != TokenKind::Word {
                continue;
            }

            // Check if word is prohibited
            if is_en_prohibit(token.text) {
                let suggestion = EN_PROHIBIT_SUGGESTIONS.get(token.text);

                let message = if let Some(s) = suggestion {
                    format!("'{}' is incorrect. Did you mean '{}'?", token.text, s)
                } else {
                    format!("'{}' appears to be a misspelling.", token.text)
                };

                let suggestions = suggestion
                    .map(|s| vec![s.to_string()])
                    .unwrap_or_default();

                result.matches.push(Match {
                    span: token.span.clone(),
                    message,
                    rule_id: "PROHIBIT".to_string(),
                    suggestions,
                    severity: Severity::Error,
                });
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
        let checker = ProhibitChecker::new();
        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();
        let tokens = tokenizer.tokenize(text);
        let analyzed = analyzer.analyze(tokens);
        checker.check(text, &analyzed)
    }

    #[test]
    fn test_christoper() {
        let result = check_text("Christoper went to the store.");
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].suggestions[0], "Christopher");
    }

    #[test]
    fn test_no_false_positive() {
        let result = check_text("Christopher went to the store.");
        assert_eq!(result.matches.len(), 0);
    }
}
