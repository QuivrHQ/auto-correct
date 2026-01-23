//! POS Tagger with FST dictionary and suffix heuristics
//!
//! This module provides a part-of-speech tagger that uses:
//! 1. FST dictionary for known words (~90% accuracy)
//! 2. Suffix-based heuristics for unknown words
//!
//! Performance: O(word_length) lookup

use std::collections::HashMap;
use crate::core::{AnalyzedToken, Token, TokenKind, PosTag};
use crate::core::traits::Analyzer;

/// Suffix rule for unknown word tagging
struct SuffixRule {
    suffix: &'static str,
    tag: PosTag,
}

/// POS Tagger with dictionary and suffix heuristics
pub struct PosTagger {
    /// word -> (lemma, POS tag)
    dictionary: HashMap<String, (String, PosTag)>,
    /// Suffix rules for unknown words (English)
    en_suffix_rules: Vec<SuffixRule>,
}

impl PosTagger {
    pub fn new() -> Self {
        Self {
            dictionary: HashMap::new(),
            en_suffix_rules: Self::default_en_suffix_rules(),
        }
    }

    /// Create a PosTagger with a preloaded dictionary
    pub fn with_dictionary(dictionary: HashMap<String, (String, PosTag)>) -> Self {
        Self {
            dictionary,
            en_suffix_rules: Self::default_en_suffix_rules(),
        }
    }

    /// Load dictionary entries from lines in "word\tlemma\tPOS" format
    pub fn load_from_lines<I, S>(&mut self, lines: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for line in lines {
            let line = line.as_ref();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                let word = parts[0].to_lowercase();
                let lemma = parts[1].to_string();
                if let Some(pos) = PosTag::from_str(parts[2]) {
                    self.dictionary.insert(word, (lemma, pos));
                }
            }
        }
    }

    /// Add a single word to the dictionary
    pub fn add_word(&mut self, word: &str, lemma: &str, pos: PosTag) {
        self.dictionary.insert(word.to_lowercase(), (lemma.to_string(), pos));
    }

    /// Number of entries in the dictionary
    pub fn dictionary_size(&self) -> usize {
        self.dictionary.len()
    }

    /// Tag a single word
    pub fn tag_word(&self, word: &str) -> Option<PosTag> {
        let lower = word.to_lowercase();

        // First, try dictionary lookup
        if let Some((_, pos)) = self.dictionary.get(&lower) {
            return Some(*pos);
        }

        // Then, try suffix heuristics
        self.guess_pos_from_suffix(&lower)
    }

    /// Get lemma for a word
    pub fn get_lemma(&self, word: &str) -> Option<&str> {
        self.dictionary
            .get(&word.to_lowercase())
            .map(|(lemma, _)| lemma.as_str())
    }

    /// Guess POS from suffix (English)
    fn guess_pos_from_suffix(&self, word: &str) -> Option<PosTag> {
        // Skip very short words
        if word.len() < 3 {
            return None;
        }

        for rule in &self.en_suffix_rules {
            if word.ends_with(rule.suffix) {
                return Some(rule.tag);
            }
        }

        None
    }

    /// Default English suffix rules
    fn default_en_suffix_rules() -> Vec<SuffixRule> {
        vec![
            // Verb forms
            SuffixRule { suffix: "ing", tag: PosTag::VBG },      // running, eating
            SuffixRule { suffix: "ed", tag: PosTag::VBD },       // walked, talked (or VBN)
            SuffixRule { suffix: "ize", tag: PosTag::VB },       // organize, realize
            SuffixRule { suffix: "ise", tag: PosTag::VB },       // organise (British)
            SuffixRule { suffix: "ify", tag: PosTag::VB },       // simplify, clarify
            SuffixRule { suffix: "ate", tag: PosTag::VB },       // create, operate

            // Adjectives
            SuffixRule { suffix: "able", tag: PosTag::JJ },      // readable, doable
            SuffixRule { suffix: "ible", tag: PosTag::JJ },      // possible, visible
            SuffixRule { suffix: "ful", tag: PosTag::JJ },       // beautiful, careful
            SuffixRule { suffix: "less", tag: PosTag::JJ },      // careless, hopeless
            SuffixRule { suffix: "ous", tag: PosTag::JJ },       // dangerous, famous
            SuffixRule { suffix: "ive", tag: PosTag::JJ },       // creative, active
            SuffixRule { suffix: "al", tag: PosTag::JJ },        // natural, formal
            SuffixRule { suffix: "ish", tag: PosTag::JJ },       // childish, reddish
            SuffixRule { suffix: "ic", tag: PosTag::JJ },        // historic, basic
            SuffixRule { suffix: "ical", tag: PosTag::JJ },      // historical, political
            SuffixRule { suffix: "ent", tag: PosTag::JJ },       // different, apparent
            SuffixRule { suffix: "ant", tag: PosTag::JJ },       // important, distant

            // Comparative/superlative
            SuffixRule { suffix: "er", tag: PosTag::JJR },       // bigger, faster
            SuffixRule { suffix: "est", tag: PosTag::JJS },      // biggest, fastest

            // Adverbs
            SuffixRule { suffix: "ly", tag: PosTag::RB },        // quickly, slowly

            // Nouns
            SuffixRule { suffix: "tion", tag: PosTag::NN },      // nation, creation
            SuffixRule { suffix: "sion", tag: PosTag::NN },      // vision, decision
            SuffixRule { suffix: "ment", tag: PosTag::NN },      // movement, agreement
            SuffixRule { suffix: "ness", tag: PosTag::NN },      // happiness, darkness
            SuffixRule { suffix: "ity", tag: PosTag::NN },       // ability, activity
            SuffixRule { suffix: "ance", tag: PosTag::NN },      // importance, distance
            SuffixRule { suffix: "ence", tag: PosTag::NN },      // difference, presence
            SuffixRule { suffix: "er", tag: PosTag::NN },        // teacher, worker (also JJR)
            SuffixRule { suffix: "or", tag: PosTag::NN },        // actor, doctor
            SuffixRule { suffix: "ist", tag: PosTag::NN },       // artist, scientist
            SuffixRule { suffix: "ism", tag: PosTag::NN },       // capitalism, tourism
            SuffixRule { suffix: "ship", tag: PosTag::NN },      // friendship, leadership
            SuffixRule { suffix: "dom", tag: PosTag::NN },       // freedom, kingdom

            // Plural nouns
            SuffixRule { suffix: "ies", tag: PosTag::NNS },      // cities, parties
            SuffixRule { suffix: "es", tag: PosTag::NNS },       // boxes, buses
            SuffixRule { suffix: "s", tag: PosTag::NNS },        // cats, dogs (last resort)
        ]
    }
}

impl Default for PosTagger {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer for PosTagger {
    fn analyze<'a>(&self, tokens: Vec<Token<'a>>) -> Vec<AnalyzedToken<'a>> {
        tokens
            .into_iter()
            .map(|token| {
                let lower = token.text.to_lowercase();
                let lookup = self.dictionary.get(&lower);

                let lemma = lookup.map(|(l, _)| l.clone());
                let pos = lookup.map(|(_, p)| *p).or_else(|| {
                    match token.kind {
                        TokenKind::Punctuation => Some(PosTag::Punctuation),
                        TokenKind::Number => Some(PosTag::CD),
                        TokenKind::Word => self.guess_pos_from_suffix(&lower),
                        _ => None,
                    }
                });

                AnalyzedToken {
                    token,
                    lemma,
                    pos,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_lookup() {
        let mut tagger = PosTagger::new();
        tagger.add_word("dog", "dog", PosTag::NN);
        tagger.add_word("dogs", "dog", PosTag::NNS);
        tagger.add_word("run", "run", PosTag::VB);
        tagger.add_word("runs", "run", PosTag::VBZ);

        assert_eq!(tagger.tag_word("dog"), Some(PosTag::NN));
        assert_eq!(tagger.tag_word("dogs"), Some(PosTag::NNS));
        assert_eq!(tagger.tag_word("run"), Some(PosTag::VB));
        assert_eq!(tagger.tag_word("runs"), Some(PosTag::VBZ));
    }

    #[test]
    fn test_case_insensitive() {
        let mut tagger = PosTagger::new();
        tagger.add_word("dog", "dog", PosTag::NN);

        assert_eq!(tagger.tag_word("Dog"), Some(PosTag::NN));
        assert_eq!(tagger.tag_word("DOG"), Some(PosTag::NN));
    }

    #[test]
    fn test_suffix_heuristics() {
        let tagger = PosTagger::new();

        // Verbs
        assert_eq!(tagger.tag_word("running"), Some(PosTag::VBG));
        assert_eq!(tagger.tag_word("walked"), Some(PosTag::VBD));
        assert_eq!(tagger.tag_word("organize"), Some(PosTag::VB));

        // Adjectives
        assert_eq!(tagger.tag_word("readable"), Some(PosTag::JJ));
        assert_eq!(tagger.tag_word("beautiful"), Some(PosTag::JJ));
        assert_eq!(tagger.tag_word("dangerous"), Some(PosTag::JJ));

        // Adverbs
        assert_eq!(tagger.tag_word("quickly"), Some(PosTag::RB));
        assert_eq!(tagger.tag_word("slowly"), Some(PosTag::RB));

        // Nouns
        assert_eq!(tagger.tag_word("creation"), Some(PosTag::NN));
        assert_eq!(tagger.tag_word("happiness"), Some(PosTag::NN));
        assert_eq!(tagger.tag_word("artist"), Some(PosTag::NN));
    }

    #[test]
    fn test_lemma_lookup() {
        let mut tagger = PosTagger::new();
        tagger.add_word("running", "run", PosTag::VBG);
        tagger.add_word("dogs", "dog", PosTag::NNS);

        assert_eq!(tagger.get_lemma("running"), Some("run"));
        assert_eq!(tagger.get_lemma("dogs"), Some("dog"));
        assert_eq!(tagger.get_lemma("unknown"), None);
    }

    #[test]
    fn test_load_from_lines() {
        let mut tagger = PosTagger::new();
        let lines = vec![
            "dog\tdog\tNN",
            "dogs\tdog\tNNS",
            "# comment line",
            "run\trun\tVB",
            "",
            "running\trun\tVBG",
        ];
        tagger.load_from_lines(lines);

        assert_eq!(tagger.dictionary_size(), 4);
        assert_eq!(tagger.tag_word("dog"), Some(PosTag::NN));
        assert_eq!(tagger.tag_word("running"), Some(PosTag::VBG));
    }

    #[test]
    fn test_analyze_tokens() {
        use crate::tokenizer::SimpleTokenizer;
        use crate::core::traits::Tokenizer;

        let tokenizer = SimpleTokenizer::new();
        let mut tagger = PosTagger::new();
        tagger.add_word("the", "the", PosTag::DT);
        tagger.add_word("cat", "cat", PosTag::NN);

        let tokens = tokenizer.tokenize("The cat");
        let analyzed = tagger.analyze(tokens);

        // Check that tokens are analyzed
        assert_eq!(analyzed.len(), 3); // "The", " ", "cat"
        assert_eq!(analyzed[0].pos, Some(PosTag::DT)); // "The" -> DT
        assert_eq!(analyzed[2].pos, Some(PosTag::NN)); // "cat" -> NN
    }

    #[test]
    fn test_matches_pattern() {
        assert!(PosTag::NN.matches_pattern("NN"));
        assert!(PosTag::NN.matches_pattern("NN.*"));
        assert!(PosTag::NNS.matches_pattern("NN.*"));
        assert!(PosTag::NNP.matches_pattern("NN.*"));
        assert!(PosTag::VBG.matches_pattern("VB.*"));
        assert!(!PosTag::JJ.matches_pattern("NN.*"));
    }
}
