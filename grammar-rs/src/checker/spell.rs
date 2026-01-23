//! Spell checker
//!
//! Supports two dictionary backends:
//! - HashSet: Simple, good for small dictionaries
//! - FstDictionary: Memory-efficient, fast, good for large dictionaries

use crate::core::{AnalyzedToken, CheckResult, Match, Severity, TokenKind};
use crate::core::traits::{Checker, Suggester};
use crate::dictionary::FstDictionary;
use std::collections::HashSet;
use std::sync::Arc;

/// Dictionary backend for spell checking
enum DictionaryBackend {
    HashSet(HashSet<String>),
    Fst(Arc<FstDictionary>),
}

pub struct SpellChecker {
    backend: DictionaryBackend,
    max_edit_distance: usize,
    skip_words: HashSet<String>,
}

impl SpellChecker {
    /// Create a new spell checker with empty HashSet backend
    pub fn new() -> Self {
        Self {
            backend: DictionaryBackend::HashSet(HashSet::new()),
            max_edit_distance: 2,
            skip_words: HashSet::new(),
        }
    }

    /// Create a spell checker with an FST dictionary
    pub fn with_fst_dictionary(dict: FstDictionary) -> Self {
        Self {
            backend: DictionaryBackend::Fst(Arc::new(dict)),
            max_edit_distance: 2,
            skip_words: HashSet::new(),
        }
    }

    /// Create a spell checker with a shared FST dictionary
    pub fn with_shared_fst(dict: Arc<FstDictionary>) -> Self {
        Self {
            backend: DictionaryBackend::Fst(dict),
            max_edit_distance: 2,
            skip_words: HashSet::new(),
        }
    }

    /// Add words to skip during spell checking (proper nouns, acronyms, etc.)
    pub fn with_skip_words<I, S>(mut self, words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for word in words {
            self.skip_words.insert(word.as_ref().to_lowercase());
        }
        self
    }

    /// Check if a word should be skipped
    fn should_skip(&self, word: &str) -> bool {
        self.skip_words.contains(&word.to_lowercase())
    }

    /// Charge un dictionnaire depuis une liste de mots (HashSet backend)
    pub fn load_words<I, S>(&mut self, words: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        match &mut self.backend {
            DictionaryBackend::HashSet(set) => {
                for word in words {
                    set.insert(word.as_ref().to_lowercase());
                }
            }
            DictionaryBackend::Fst(_) => {
                // Convert to HashSet if loading words into FST backend
                let mut set = HashSet::new();
                for word in words {
                    set.insert(word.as_ref().to_lowercase());
                }
                self.backend = DictionaryBackend::HashSet(set);
            }
        }
    }

    pub fn with_words<I, S>(mut self, words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.load_words(words);
        self
    }

    /// Set max edit distance for suggestions
    pub fn with_max_edit_distance(mut self, distance: usize) -> Self {
        self.max_edit_distance = distance;
        self
    }

    fn is_valid(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        match &self.backend {
            DictionaryBackend::HashSet(set) => set.contains(&lower),
            DictionaryBackend::Fst(dict) => dict.contains_lowercase(&lower),
        }
    }

    /// Get dictionary size
    pub fn dictionary_size(&self) -> usize {
        match &self.backend {
            DictionaryBackend::HashSet(set) => set.len(),
            DictionaryBackend::Fst(dict) => dict.len(),
        }
    }
}

impl Default for SpellChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker for SpellChecker {
    fn check(&self, _text: &str, tokens: &[AnalyzedToken]) -> CheckResult {
        let mut result = CheckResult::new();

        for analyzed in tokens {
            let token = &analyzed.token;

            // Skip non-words
            if token.kind != TokenKind::Word {
                continue;
            }

            // Skip short words and numbers
            if token.text.len() < 2 {
                continue;
            }

            // Skip if starts with uppercase (proper noun heuristic)
            if token.text.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                && token.span.start > 0
            {
                continue;
            }

            // Skip words in the skip list (acronyms, proper nouns, etc.)
            if self.should_skip(token.text) {
                continue;
            }

            if !self.is_valid(token.text) {
                let suggestions = self.suggest(token.text, 3);
                result.matches.push(Match {
                    span: token.span.clone(),
                    message: format!("Possible spelling mistake: '{}'", token.text),
                    rule_id: "SPELL".to_string(),
                    suggestions,
                    severity: Severity::Error,
                });
            }
        }

        result
    }
}

impl Suggester for SpellChecker {
    fn suggest(&self, word: &str, max: usize) -> Vec<String> {
        let lower = word.to_lowercase();

        match &self.backend {
            DictionaryBackend::HashSet(set) => {
                let mut candidates: Vec<(String, usize)> = Vec::new();

                for dict_word in set {
                    let dist = levenshtein(&lower, dict_word);
                    if dist <= self.max_edit_distance {
                        candidates.push((dict_word.clone(), dist));
                    }
                }

                // Trier par distance puis alphabétiquement
                candidates.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

                candidates
                    .into_iter()
                    .take(max)
                    .map(|(w, _)| w)
                    .collect()
            }
            DictionaryBackend::Fst(dict) => {
                // For FST, we use prefix-based suggestions + edit distance
                // This is a simplified version - a full implementation would use SymSpell
                let mut candidates: Vec<(String, usize)> = Vec::new();

                // Try prefix matches first (fast)
                if lower.len() >= 2 {
                    let prefix = &lower[..2.min(lower.len())];
                    for dict_word in dict.words_with_prefix(prefix) {
                        let dist = levenshtein(&lower, &dict_word);
                        if dist <= self.max_edit_distance {
                            candidates.push((dict_word, dist));
                        }
                    }
                }

                // Also try with first char only for more coverage
                if candidates.len() < max && !lower.is_empty() {
                    let prefix = &lower[..1];
                    for dict_word in dict.words_with_prefix(prefix) {
                        let dist = levenshtein(&lower, &dict_word);
                        if dist <= self.max_edit_distance {
                            if !candidates.iter().any(|(w, _)| w == &dict_word) {
                                candidates.push((dict_word, dist));
                            }
                        }
                    }
                }

                candidates.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

                candidates
                    .into_iter()
                    .take(max)
                    .map(|(w, _)| w)
                    .collect()
            }
        }
    }
}

/// Distance de Levenshtein simple
/// Version 2: utiliser triple_accel ou strsim crate
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let mut matrix = vec![vec![0usize; b.len() + 1]; a.len() + 1];

    for i in 0..=a.len() {
        matrix[i][0] = i;
    }
    for j in 0..=b.len() {
        matrix[0][j] = j;
    }

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::SimpleTokenizer;
    use crate::analyzer::PassthroughAnalyzer;
    use crate::core::traits::{Tokenizer, Analyzer};

    #[test]
    fn test_spell_check() {
        let checker = SpellChecker::new()
            .with_words(["hello", "world", "test"]);

        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();

        let tokens = tokenizer.tokenize("helo world");
        let analyzed = analyzer.analyze(tokens);
        let result = checker.check("helo world", &analyzed);

        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].span, 0..4);
        assert!(result.matches[0].suggestions.contains(&"hello".to_string()));
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("hello", "hello"), 0);
        assert_eq!(levenshtein("hello", "helo"), 1);
        assert_eq!(levenshtein("hello", "world"), 4);
    }

    #[test]
    fn test_skip_words() {
        let checker = SpellChecker::new()
            .with_words(["hello", "world", "test"])
            .with_skip_words(["IBM", "JSON", "API"]);

        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();

        // Test that skip words are not flagged
        let tokens = tokenizer.tokenize("IBM uses JSON API");
        let analyzed = analyzer.analyze(tokens);
        let result = checker.check("IBM uses JSON API", &analyzed);

        // "uses" is not in dictionary but skip words should not be flagged
        // Only "uses" should be flagged (not IBM, JSON, API)
        for m in &result.matches {
            assert!(!["ibm", "json", "api"].contains(&m.suggestions.first().map(|s| s.as_str()).unwrap_or("")));
        }
    }

    #[test]
    fn test_skip_words_case_insensitive() {
        let checker = SpellChecker::new()
            .with_words(["hello", "world"])
            .with_skip_words(["NASA"]);

        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();

        // Test case insensitivity: "nasa" should also be skipped
        let tokens = tokenizer.tokenize("nasa hello");
        let analyzed = analyzer.analyze(tokens);
        let result = checker.check("nasa hello", &analyzed);

        // "nasa" should be skipped, "hello" is valid
        assert_eq!(result.matches.len(), 0);
    }

    #[test]
    fn test_should_skip() {
        let checker = SpellChecker::new()
            .with_skip_words(["IBM", "JSON"]);

        assert!(checker.should_skip("IBM"));
        assert!(checker.should_skip("ibm"));  // case insensitive
        assert!(checker.should_skip("JSON"));
        assert!(!checker.should_skip("hello"));
    }
}
