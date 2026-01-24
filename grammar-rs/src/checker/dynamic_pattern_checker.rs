//! Dynamic Pattern Checker
//!
//! This module provides a checker for complex grammar rules loaded from JSON at runtime.
//! It supports advanced pattern features that can't be compiled to static Rust code:
//! - Regex-based token matching
//! - POS tag regex matching
//! - Optional tokens (min/max)
//! - Skip gaps (match with N tokens between)
//! - Antipatterns (exceptions to rules)

use crate::core::traits::Checker;
use crate::core::{AnalyzedToken, CheckResult, Match, Severity, TokenKind};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// ═══════════════════════════════════════════════════════════════════════════════
// JSON Structures (matching sync-lt output)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexPatternToken {
    pub text: Option<String>,
    pub regexp: Option<String>,
    pub postag: Option<String>,
    pub postag_regexp: bool,
    pub inflected: bool,
    pub case_sensitive: bool,
    pub negation: bool,
    pub min: u32,
    pub max: u32,
    pub skip: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexAntipattern {
    pub tokens: Vec<ComplexPatternToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexExample {
    pub text: String,
    pub is_correct: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub correction: Option<String>,
}

/// Dynamic suggestion with parts that can reference matched tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexSuggestion {
    pub parts: Vec<SuggestionPart>,
}

/// Part of a dynamic suggestion - either literal text or a reference to a matched token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SuggestionPart {
    Literal { text: String },
    MatchRef {
        /// Token index (1-indexed as in LanguageTool)
        index: usize,
        /// Regex pattern to match on token text
        #[serde(skip_serializing_if = "Option::is_none", default)]
        regexp_match: Option<String>,
        /// Replacement for regex match
        #[serde(skip_serializing_if = "Option::is_none", default)]
        regexp_replace: Option<String>,
        /// POS tag pattern (with postag_regexp="yes")
        #[serde(skip_serializing_if = "Option::is_none", default)]
        postag: Option<String>,
        /// POS tag replacement pattern (e.g., "$1 f s")
        #[serde(skip_serializing_if = "Option::is_none", default)]
        postag_replace: Option<String>,
        /// Case conversion: "startlower", "startupper", "alllower", "allupper", "preserve"
        #[serde(skip_serializing_if = "Option::is_none", default)]
        case_conversion: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexRule {
    pub id: String,
    pub name: String,
    pub category: String,
    pub pattern: Vec<ComplexPatternToken>,
    pub antipatterns: Vec<ComplexAntipattern>,
    pub message: String,
    /// Static suggestions (simple text)
    pub suggestions: Vec<String>,
    /// Dynamic suggestions with <match> references
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub dynamic_suggestions: Vec<ComplexSuggestion>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub examples: Vec<ComplexExample>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Compiled structures (runtime)
// ═══════════════════════════════════════════════════════════════════════════════

/// Compiled token pattern for efficient matching
struct CompiledToken {
    text: Option<String>,
    text_regex: Option<Regex>,
    postag: Option<String>,
    postag_regex: Option<Regex>,
    case_sensitive: bool,
    negation: bool,
    min: u32,
    max: u32,
    skip: Option<i32>,
}

impl CompiledToken {
    fn from_json(token: &ComplexPatternToken) -> Option<Self> {
        // Compile text regex if present
        let text_regex = if let Some(ref regexp) = token.regexp {
            // Make pattern case-insensitive by default unless case_sensitive
            let pattern = if token.case_sensitive {
                regexp.clone()
            } else {
                format!("(?i){}", regexp)
            };
            Regex::new(&format!("^(?:{})$", pattern)).ok()
        } else {
            None
        };

        // Compile POS tag regex if postag_regexp is true
        let postag_regex = if token.postag_regexp {
            token.postag.as_ref().and_then(|p| {
                // LanguageTool uses Java regex patterns, convert common patterns
                let pattern = convert_lt_postag_regex(p);
                Regex::new(&format!("^(?:{})$", pattern)).ok()
            })
        } else {
            None
        };

        Some(CompiledToken {
            text: token.text.clone(),
            text_regex,
            postag: if token.postag_regexp { None } else { token.postag.clone() },
            postag_regex,
            case_sensitive: token.case_sensitive,
            negation: token.negation,
            min: token.min,
            max: token.max,
            skip: token.skip,
        })
    }

    /// Check if this compiled token matches the given analyzed token
    fn matches(&self, token: &AnalyzedToken) -> bool {
        // Text matching
        let text_match = if let Some(ref text) = self.text {
            if self.case_sensitive {
                token.token.text == text
            } else {
                token.token.text.eq_ignore_ascii_case(text)
            }
        } else if let Some(ref regex) = self.text_regex {
            regex.is_match(token.token.text)
        } else {
            true // No text constraint
        };

        // POS tag matching
        let pos_match = if let Some(ref postag) = self.postag {
            // Exact or prefix match
            token.pos.as_ref().map_or(false, |pos| {
                let pos_str = pos.as_str();
                pos_str == postag
                    || pos_str.starts_with(postag)
                    || pos.matches_french_pattern(postag)
            })
        } else if let Some(ref regex) = self.postag_regex {
            token.pos.as_ref().map_or(false, |pos| {
                regex.is_match(pos.as_str())
            })
        } else {
            true // No POS constraint
        };

        let result = text_match && pos_match;
        if self.negation { !result } else { result }
    }
}

/// Compiled antipattern for efficient matching
struct CompiledAntipattern {
    tokens: Vec<CompiledToken>,
}

impl CompiledAntipattern {
    fn from_json(antipattern: &ComplexAntipattern) -> Option<Self> {
        let tokens: Option<Vec<_>> = antipattern
            .tokens
            .iter()
            .map(CompiledToken::from_json)
            .collect();
        tokens.map(|t| CompiledAntipattern { tokens: t })
    }
}

/// Compiled rule ready for efficient matching
struct CompiledRule {
    id: String,
    pattern: Vec<CompiledToken>,
    antipatterns: Vec<CompiledAntipattern>,
    message: String,
    suggestions: Vec<String>,
    /// Dynamic suggestions with match references (kept as-is for runtime generation)
    dynamic_suggestions: Vec<ComplexSuggestion>,
}

impl CompiledRule {
    fn from_json(rule: &ComplexRule) -> Option<Self> {
        let pattern: Option<Vec<_>> = rule
            .pattern
            .iter()
            .map(CompiledToken::from_json)
            .collect();

        let antipatterns: Vec<_> = rule
            .antipatterns
            .iter()
            .filter_map(CompiledAntipattern::from_json)
            .collect();

        pattern.map(|p| CompiledRule {
            id: rule.id.clone(),
            pattern: p,
            antipatterns,
            message: rule.message.clone(),
            suggestions: rule.suggestions.clone(),
            dynamic_suggestions: rule.dynamic_suggestions.clone(),
        })
    }
}

/// Convert LanguageTool POS tag regex patterns to Rust regex
fn convert_lt_postag_regex(pattern: &str) -> String {
    // LanguageTool uses Java regex, most patterns are compatible
    // Common patterns:
    // - "V.*" -> matches any verb
    // - "N . ." -> noun with any gender/number
    // - "V ind.*:3s" -> indicative verb 3rd person singular
    pattern
        .replace("\\", "\\\\") // Escape backslashes
        .replace(".", "\\.") // Escape dots (literal in LT)
        .replace("\\\\.", ".") // But .* means "any char" - convert back
        .replace(" ", "\\s+") // Spaces in French tags
}

// ═══════════════════════════════════════════════════════════════════════════════
// Dynamic Pattern Checker
// ═══════════════════════════════════════════════════════════════════════════════

/// Checker for complex grammar rules loaded at runtime
pub struct DynamicPatternChecker {
    rules: Vec<CompiledRule>,
}

impl DynamicPatternChecker {
    /// Create an empty checker
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Load rules from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let raw_rules: Vec<ComplexRule> = serde_json::from_str(json)?;
        let rules: Vec<_> = raw_rules
            .iter()
            .filter_map(CompiledRule::from_json)
            .collect();

        Ok(Self { rules })
    }

    /// Load rules from a slice of ComplexRule
    pub fn from_rules(raw_rules: &[ComplexRule]) -> Self {
        let rules: Vec<_> = raw_rules
            .iter()
            .filter_map(CompiledRule::from_json)
            .collect();

        Self { rules }
    }

    /// Number of loaded rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Check if a pattern matches at a given position in word tokens
    /// Returns the end index if matched, None otherwise
    fn try_match_pattern(
        &self,
        word_tokens: &[(usize, &AnalyzedToken)],
        start: usize,
        pattern: &[CompiledToken],
    ) -> Option<usize> {
        let mut token_idx = start;
        let mut pattern_idx = 0;

        while pattern_idx < pattern.len() {
            let pat_token = &pattern[pattern_idx];

            // Handle optional tokens (min=0)
            if pat_token.min == 0 {
                // Try matching with and without this token
                if token_idx < word_tokens.len() && pat_token.matches(word_tokens[token_idx].1) {
                    // Token matched, continue
                    token_idx += 1;
                }
                // Move to next pattern element regardless (it's optional)
                pattern_idx += 1;
                continue;
            }

            // Handle skip (gap matching)
            if let Some(skip) = pat_token.skip {
                // Match current token
                if token_idx >= word_tokens.len() || !pat_token.matches(word_tokens[token_idx].1) {
                    return None;
                }
                token_idx += 1;

                // Skip up to N tokens before matching next pattern element
                if pattern_idx + 1 < pattern.len() {
                    let next_pat = &pattern[pattern_idx + 1];
                    let max_skip = if skip < 0 {
                        word_tokens.len() - token_idx
                    } else {
                        skip as usize
                    };

                    let mut found = false;
                    for offset in 0..=max_skip {
                        if token_idx + offset < word_tokens.len()
                            && next_pat.matches(word_tokens[token_idx + offset].1)
                        {
                            token_idx += offset;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return None;
                    }
                }
                pattern_idx += 1;
                continue;
            }

            // Regular matching (min=1, max=1)
            if token_idx >= word_tokens.len() {
                return None;
            }

            if !pat_token.matches(word_tokens[token_idx].1) {
                return None;
            }

            token_idx += 1;
            pattern_idx += 1;
        }

        Some(token_idx)
    }

    /// Check if any antipattern matches the given token range
    fn matches_antipattern(
        &self,
        word_tokens: &[(usize, &AnalyzedToken)],
        start: usize,
        end: usize,
        antipatterns: &[CompiledAntipattern],
    ) -> bool {
        for ap in antipatterns {
            // Antipattern must match starting from the same position
            if let Some(ap_end) = self.try_match_pattern(word_tokens, start, &ap.tokens) {
                // Antipattern matched
                if ap_end >= end {
                    return true;
                }
            }
        }
        false
    }

    /// Generate suggestions from dynamic suggestion templates using matched tokens
    fn generate_dynamic_suggestions(
        &self,
        matched_tokens: &[&AnalyzedToken],
        dynamic_suggestions: &[ComplexSuggestion],
    ) -> Vec<String> {
        let mut result = Vec::new();

        for suggestion in dynamic_suggestions {
            let mut generated = String::new();

            for part in &suggestion.parts {
                match part {
                    SuggestionPart::Literal { text } => {
                        generated.push_str(text);
                    }
                    SuggestionPart::MatchRef {
                        index,
                        regexp_match,
                        regexp_replace,
                        case_conversion,
                        ..
                    } => {
                        // index is 1-based in LanguageTool
                        if *index > 0 && *index <= matched_tokens.len() {
                            let token = matched_tokens[*index - 1];
                            let mut text = token.token.text.to_string();

                            // Apply regex replacement if present
                            if let (Some(pattern), Some(replacement)) = (regexp_match, regexp_replace) {
                                if let Ok(re) = Regex::new(&format!("(?i){}", pattern)) {
                                    text = re.replace_all(&text, replacement.as_str()).to_string();
                                }
                            }

                            // Apply case conversion if specified
                            if let Some(conversion) = case_conversion {
                                text = match conversion.as_str() {
                                    "alllower" => text.to_lowercase(),
                                    "allupper" => text.to_uppercase(),
                                    "startlower" => {
                                        let mut chars = text.chars();
                                        match chars.next() {
                                            Some(c) => {
                                                let lower: String = c.to_lowercase().collect();
                                                lower + chars.as_str()
                                            }
                                            None => text,
                                        }
                                    }
                                    "startupper" => {
                                        let mut chars = text.chars();
                                        match chars.next() {
                                            Some(c) => {
                                                let upper: String = c.to_uppercase().collect();
                                                upper + chars.as_str()
                                            }
                                            None => text,
                                        }
                                    }
                                    _ => text, // preserve or unknown
                                };
                            }

                            generated.push_str(&text);
                        }
                    }
                }
            }

            if !generated.is_empty() {
                result.push(generated);
            }
        }

        result
    }
}

impl Default for DynamicPatternChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker for DynamicPatternChecker {
    fn check(&self, _text: &str, tokens: &[AnalyzedToken]) -> CheckResult {
        let mut matches = Vec::new();

        // Filter out whitespace tokens for pattern matching
        let word_tokens: Vec<(usize, &AnalyzedToken)> = tokens
            .iter()
            .enumerate()
            .filter(|(_, t)| t.token.kind != TokenKind::Whitespace)
            .collect();

        for rule in &self.rules {
            // Slide pattern across all word positions
            for start in 0..word_tokens.len() {
                if let Some(end) = self.try_match_pattern(&word_tokens, start, &rule.pattern) {
                    // Check if any antipattern matches (exception)
                    if self.matches_antipattern(&word_tokens, start, end, &rule.antipatterns) {
                        continue;
                    }

                    // Calculate span from first to last matched token
                    let span_start = word_tokens[start].1.token.span.start;
                    let span_end = word_tokens[end - 1].1.token.span.end;

                    // Collect matched tokens for dynamic suggestion generation
                    let matched_tokens: Vec<&AnalyzedToken> = (start..end)
                        .map(|i| word_tokens[i].1)
                        .collect();

                    // Generate suggestions: static first, then dynamic
                    let mut suggestions = rule.suggestions.clone();
                    if !rule.dynamic_suggestions.is_empty() {
                        let dynamic = self.generate_dynamic_suggestions(
                            &matched_tokens,
                            &rule.dynamic_suggestions,
                        );
                        suggestions.extend(dynamic);
                    }

                    matches.push(Match {
                        span: span_start..span_end,
                        message: rule.message.clone(),
                        rule_id: rule.id.clone(),
                        suggestions,
                        severity: Severity::Warning,
                    });
                }
            }
        }

        CheckResult { matches }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Static loading for embedded JSON
// ═══════════════════════════════════════════════════════════════════════════════

/// Embedded French complex patterns JSON
static FR_COMPLEX_PATTERNS_JSON: &str = include_str!("data/fr_complex_patterns.json");

/// Embedded English complex patterns JSON
static EN_COMPLEX_PATTERNS_JSON: &str = include_str!("data/en_complex_patterns.json");

/// Lazily compiled French checker
static FR_DYNAMIC_CHECKER: OnceLock<DynamicPatternChecker> = OnceLock::new();

/// Lazily compiled English checker
static EN_DYNAMIC_CHECKER: OnceLock<DynamicPatternChecker> = OnceLock::new();

/// Get the French dynamic pattern checker (lazily compiled)
pub fn get_fr_dynamic_checker() -> &'static DynamicPatternChecker {
    FR_DYNAMIC_CHECKER.get_or_init(|| {
        DynamicPatternChecker::from_json(FR_COMPLEX_PATTERNS_JSON)
            .expect("Failed to parse fr_complex_patterns.json")
    })
}

/// Get the English dynamic pattern checker (lazily compiled)
pub fn get_en_dynamic_checker() -> &'static DynamicPatternChecker {
    EN_DYNAMIC_CHECKER.get_or_init(|| {
        DynamicPatternChecker::from_json(EN_COMPLEX_PATTERNS_JSON)
            .expect("Failed to parse en_complex_patterns.json")
    })
}

/// Create a new French dynamic pattern checker
pub fn create_fr_dynamic_checker() -> DynamicPatternChecker {
    DynamicPatternChecker::from_json(FR_COMPLEX_PATTERNS_JSON)
        .expect("Failed to parse fr_complex_patterns.json")
}

/// Create a new English dynamic pattern checker
pub fn create_en_dynamic_checker() -> DynamicPatternChecker {
    DynamicPatternChecker::from_json(EN_COMPLEX_PATTERNS_JSON)
        .expect("Failed to parse en_complex_patterns.json")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{PosTag, Token};

    fn make_token<'a>(text: &'a str, pos: Option<PosTag>, start: usize) -> AnalyzedToken<'a> {
        AnalyzedToken {
            token: Token {
                text,
                span: start..start + text.len(),
                kind: TokenKind::Word,
            },
            lemma: None,
            pos,
        }
    }

    #[test]
    fn test_load_fr_patterns() {
        let checker = create_fr_dynamic_checker();
        assert!(checker.rule_count() > 0, "Should have loaded FR rules");
        println!("Loaded {} FR complex pattern rules", checker.rule_count());
    }

    #[test]
    fn test_load_en_patterns() {
        let checker = create_en_dynamic_checker();
        assert!(checker.rule_count() > 0, "Should have loaded EN rules");
        println!("Loaded {} EN complex pattern rules", checker.rule_count());
    }

    #[test]
    fn test_simple_text_match() {
        let json = r#"[{
            "id": "TEST_RULE",
            "name": "Test",
            "category": "Test",
            "pattern": [
                {"text": "test", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null}
            ],
            "antipatterns": [],
            "message": "Found test",
            "suggestions": []
        }]"#;

        let checker = DynamicPatternChecker::from_json(json).unwrap();
        assert_eq!(checker.rule_count(), 1);

        let tokens = vec![make_token("test", None, 0)];
        let result = checker.check("test", &tokens);
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].rule_id, "TEST_RULE");
    }

    #[test]
    fn test_regex_match() {
        let json = r#"[{
            "id": "REGEX_RULE",
            "name": "Regex Test",
            "category": "Test",
            "pattern": [
                {"text": null, "regexp": "test|essai", "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null}
            ],
            "antipatterns": [],
            "message": "Found test or essai",
            "suggestions": []
        }]"#;

        let checker = DynamicPatternChecker::from_json(json).unwrap();

        let tokens1 = vec![make_token("test", None, 0)];
        let result1 = checker.check("test", &tokens1);
        assert_eq!(result1.matches.len(), 1);

        let tokens2 = vec![make_token("essai", None, 0)];
        let result2 = checker.check("essai", &tokens2);
        assert_eq!(result2.matches.len(), 1);

        let tokens3 = vec![make_token("other", None, 0)];
        let result3 = checker.check("other", &tokens3);
        assert_eq!(result3.matches.len(), 0);
    }

    #[test]
    fn test_optional_token() {
        let json = r#"[{
            "id": "OPTIONAL_RULE",
            "name": "Optional Token Test",
            "category": "Test",
            "pattern": [
                {"text": "very", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 0, "max": 1, "skip": null},
                {"text": "good", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null}
            ],
            "antipatterns": [],
            "message": "Found (very) good",
            "suggestions": []
        }]"#;

        let checker = DynamicPatternChecker::from_json(json).unwrap();

        // "good" alone should match (very is optional)
        let tokens1 = vec![make_token("good", None, 0)];
        let result1 = checker.check("good", &tokens1);
        assert_eq!(result1.matches.len(), 1);

        // "very good" matches twice:
        // 1. At position 0: "very" (optional, consumed) + "good" (matched)
        // 2. At position 1: "very" (optional, skipped) + "good" (matched)
        // This is correct pattern matching behavior
        let tokens2 = vec![
            make_token("very", None, 0),
            make_token("good", None, 5),
        ];
        let result2 = checker.check("very good", &tokens2);
        assert_eq!(result2.matches.len(), 2);

        // Verify the spans are different
        let spans: Vec<_> = result2.matches.iter().map(|m| m.span.clone()).collect();
        assert!(spans.contains(&(0..9))); // "very good"
        assert!(spans.contains(&(5..9))); // "good"
    }

    #[test]
    fn test_antipattern() {
        let json = r#"[{
            "id": "ANTIPATTERN_RULE",
            "name": "Antipattern Test",
            "category": "Test",
            "pattern": [
                {"text": "test", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null},
                {"text": "case", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null}
            ],
            "antipatterns": [{
                "tokens": [
                    {"text": "test", "regexp": null, "postag": null, "postag_regexp": false,
                     "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null},
                    {"text": "case", "regexp": null, "postag": null, "postag_regexp": false,
                     "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null},
                    {"text": "ok", "regexp": null, "postag": null, "postag_regexp": false,
                     "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null}
                ]
            }],
            "message": "Found test case",
            "suggestions": []
        }]"#;

        let checker = DynamicPatternChecker::from_json(json).unwrap();

        // "test case" should match (no antipattern)
        let tokens1 = vec![
            make_token("test", None, 0),
            make_token("case", None, 5),
        ];
        let result1 = checker.check("test case", &tokens1);
        assert_eq!(result1.matches.len(), 1);

        // "test case ok" should NOT match (antipattern)
        let tokens2 = vec![
            make_token("test", None, 0),
            make_token("case", None, 5),
            make_token("ok", None, 10),
        ];
        let result2 = checker.check("test case ok", &tokens2);
        assert_eq!(result2.matches.len(), 0);
    }

    #[test]
    fn test_dynamic_suggestions() {
        // Test rule: "a error" -> "\1n \2" (insert 'n' after 'a')
        let json = r#"[{
            "id": "A_ERROR",
            "name": "A/An before vowel",
            "category": "Grammar",
            "pattern": [
                {"text": "a", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null},
                {"text": "error", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null}
            ],
            "antipatterns": [],
            "message": "Use 'an' before vowel sounds",
            "suggestions": [],
            "dynamic_suggestions": [
                {
                    "parts": [
                        {"type": "MatchRef", "index": 1, "regexp_match": "a", "regexp_replace": "an"},
                        {"type": "Literal", "text": " "},
                        {"type": "MatchRef", "index": 2}
                    ]
                }
            ]
        }]"#;

        let checker = DynamicPatternChecker::from_json(json).unwrap();

        let tokens = vec![
            make_token("a", None, 0),
            make_token("error", None, 2),
        ];
        let result = checker.check("a error", &tokens);
        assert_eq!(result.matches.len(), 1);

        // Check that the suggestion was dynamically generated
        let m = &result.matches[0];
        assert!(!m.suggestions.is_empty(), "Should have a suggestion");
        assert_eq!(m.suggestions[0], "an error", "Suggestion should be 'an error'");
    }

    #[test]
    fn test_dynamic_suggestions_case_conversion() {
        // Test case conversion: startlower
        let json = r#"[{
            "id": "CASE_TEST",
            "name": "Case conversion test",
            "category": "Test",
            "pattern": [
                {"text": "THE", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null},
                {"text": "test", "regexp": null, "postag": null, "postag_regexp": false,
                 "inflected": false, "case_sensitive": false, "negation": false, "min": 1, "max": 1, "skip": null}
            ],
            "antipatterns": [],
            "message": "Test case conversion",
            "suggestions": [],
            "dynamic_suggestions": [
                {
                    "parts": [
                        {"type": "MatchRef", "index": 1, "case_conversion": "alllower"},
                        {"type": "Literal", "text": " "},
                        {"type": "MatchRef", "index": 2, "case_conversion": "startupper"}
                    ]
                }
            ]
        }]"#;

        let checker = DynamicPatternChecker::from_json(json).unwrap();

        let tokens = vec![
            make_token("THE", None, 0),
            make_token("test", None, 4),
        ];
        let result = checker.check("THE test", &tokens);
        assert_eq!(result.matches.len(), 1);

        let m = &result.matches[0];
        assert!(!m.suggestions.is_empty(), "Should have a suggestion");
        assert_eq!(m.suggestions[0], "the Test", "Case conversion should work");
    }
}
