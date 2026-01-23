//! Auto-generated synonym rules for EN from LanguageTool
//! Synced: 2026-01-23T13:17:50.283306721+00:00
//! Total rules: 25
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool synonyms.txt
//! License: LGPL 2.1+
//!
//! These rules provide synonyms for style suggestions.

/// A synonym rule mapping a word to its synonyms
#[derive(Debug, Clone)]
pub struct SynonymEntry {
    /// The original word
    pub word: &'static str,
    /// Optional POS tag pattern (e.g., "A" for adverb, "V.*" for verbs)
    pub pos_tag: Option<&'static str>,
    /// List of synonyms
    pub synonyms: &'static [&'static str],
}

/// Synonym rules for EN (sorted by word for binary search)
pub const EN_SYNONYM_RULES: &[SynonymEntry] = &[
    SynonymEntry { word: "affect", pos_tag: Some("VB.*/.-VP"), synonyms: &["impact", "influence", "alter", "modify"] },
    SynonymEntry { word: "amazing", pos_tag: Some("JJ"), synonyms: &["incredible", "stunning", "breathtaking", "astounding", "remarkable"] },
    SynonymEntry { word: "choose", pos_tag: Some("VB.*/.-VP"), synonyms: &["decide", "select", "pick"] },
    SynonymEntry { word: "definitely", pos_tag: None, synonyms: &["certainly", "absolutely", "undoubtedly"] },
    SynonymEntry { word: "excellent", pos_tag: None, synonyms: &["exceptional", "splendid", "magnificent", "exquisite"] },
    SynonymEntry { word: "fantastic", pos_tag: Some("JJ"), synonyms: &["incredible", "stunning", "breathtaking", "astounding", "remarkable"] },
    SynonymEntry { word: "form", pos_tag: Some("VB.*/.-VP"), synonyms: &["create", "establish", "constitute", "comprise"] },
    SynonymEntry { word: "generally", pos_tag: Some("RB"), synonyms: &["mostly", "mainly", "largely", "typically"] },
    SynonymEntry { word: "gorgeous", pos_tag: Some("JJ"), synonyms: &["incredible", "stunning", "breathtaking", "astounding", "remarkable"] },
    SynonymEntry { word: "important", pos_tag: Some("JJ"), synonyms: &["significant", "essential", "critical", "influential", "indispensable"] },
    SynonymEntry { word: "incredible", pos_tag: Some("JJ"), synonyms: &["unbelievable", "impressive", "astounding", "unconceivable", "remarkable"] },
    SynonymEntry { word: "interesting", pos_tag: Some("JJ"), synonyms: &["fascinating", "intriguing", "captivating", "exciting", "appealing", "compelling"] },
    SynonymEntry { word: "literally", pos_tag: None, synonyms: &["actually", "truly"] },
    SynonymEntry { word: "maybe", pos_tag: Some("RB"), synonyms: &["possibly", "potentially", "perhaps"] },
    SynonymEntry { word: "need", pos_tag: Some("VB.*/B-VP"), synonyms: &["require"] },
    SynonymEntry { word: "nice", pos_tag: Some("JJ/.-(ADJP|NP).*"), synonyms: &["pleasant", "impressive", "enjoyable", "delightful", "charming"] },
    SynonymEntry { word: "often", pos_tag: Some("RB/I-VP|.-ADVP"), synonyms: &["typically", "frequently", "regularly"] },
    SynonymEntry { word: "problem", pos_tag: None, synonyms: &["issue", "concern", "difficulty"] },
    SynonymEntry { word: "propose", pos_tag: None, synonyms: &["suggest", "recommend", "submit"] },
    SynonymEntry { word: "several", pos_tag: Some("JJ/.*-NP.*"), synonyms: &["many", "numerous", "various", "countless"] },
    SynonymEntry { word: "suggest", pos_tag: None, synonyms: &["propose", "recommend", "submit"] },
    SynonymEntry { word: "usually", pos_tag: None, synonyms: &["typically", "often", "frequently"] },
    SynonymEntry { word: "weird", pos_tag: None, synonyms: &["odd", "peculiar", "unusual"] },
    SynonymEntry { word: "whole", pos_tag: None, synonyms: &["entire", "full", "complete"] },
    SynonymEntry { word: "wonderful", pos_tag: Some("JJ"), synonyms: &["incredible", "stunning", "breathtaking", "astounding", "remarkable"] },
];

/// Get synonyms for a word (binary search)
pub fn get_en_synonyms(word: &str) -> Option<&'static [&'static str]> {
	EN_SYNONYM_RULES
		.binary_search_by_key(&word, |r| r.word)
		.ok()
		.map(|idx| EN_SYNONYM_RULES[idx].synonyms)
}
