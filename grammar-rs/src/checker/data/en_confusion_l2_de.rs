//! Auto-generated L2 confusion pairs for German native speakers writing EN
//! Synced: 2026-01-24T12:08:47.345445+00:00
//! Total pairs: 75
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool confusion_sets_l2_de.txt
//! License: LGPL 2.1+
//!
//! False friends and common mistakes made by German native speakers.

/// An L2 confusion pair (false friend)
#[derive(Debug, Clone, Copy)]
pub struct L2ConfusionPair {
    /// The word often confused (false friend in L1)
    pub word1: &'static str,
    /// The correct word to use instead
    pub word2: &'static str,
    /// Confidence factor (higher = more likely confusion)
    pub factor: u64,
}

/// L2 confusion pairs for German native speakers (sorted by word1)
/// Total pairs: 75
pub const EN_L2_DE_CONFUSION_PAIRS: &[L2ConfusionPair] = &[
    L2ConfusionPair { word1: "actual", word2: "current", factor: 10000 },
    L2ConfusionPair { word1: "actual", word2: "latest", factor: 10 },
    L2ConfusionPair { word1: "actualize", word2: "update", factor: 100000 },
    L2ConfusionPair { word1: "actualized", word2: "updated", factor: 10000000 },
    L2ConfusionPair { word1: "advocate", word2: "attorney", factor: 1000 },
    L2ConfusionPair { word1: "advocate", word2: "lawyer", factor: 100 },
    L2ConfusionPair { word1: "art", word2: "kind", factor: 100 },
    L2ConfusionPair { word1: "art", word2: "manner", factor: 10 },
    L2ConfusionPair { word1: "art", word2: "way", factor: 100 },
    L2ConfusionPair { word1: "art", word2: "type", factor: 1000 },
    L2ConfusionPair { word1: "bald", word2: "soon", factor: 100 },
    L2ConfusionPair { word1: "become", word2: "get", factor: 100000 },
    L2ConfusionPair { word1: "become", word2: "receive", factor: 10 },
    L2ConfusionPair { word1: "blame", word2: "embarrass", factor: 10 },
    L2ConfusionPair { word1: "box", word2: "speaker", factor: 10 },
    L2ConfusionPair { word1: "concern", word2: "corporation", factor: 100 },
    L2ConfusionPair { word1: "concur", word2: "compete", factor: 100 },
    L2ConfusionPair { word1: "conquer", word2: "compete", factor: 10 },
    L2ConfusionPair { word1: "consequent", word2: "consistent", factor: 1000 },
    L2ConfusionPair { word1: "curious", word2: "odd", factor: 100 },
    L2ConfusionPair { word1: "curious", word2: "queer", factor: 100 },
    L2ConfusionPair { word1: "decent", word2: "unobtrusive", factor: 10 },
    L2ConfusionPair { word1: "dome", word2: "cathedral", factor: 100 },
    L2ConfusionPair { word1: "etiquette", word2: "label", factor: 1000 },
    L2ConfusionPair { word1: "eventually", word2: "possibly", factor: 10 },
    L2ConfusionPair { word1: "fantasy", word2: "imagination", factor: 100 },
    L2ConfusionPair { word1: "fast", word2: "almost", factor: 10 },
    L2ConfusionPair { word1: "formula", word2: "form", factor: 100000 },
    L2ConfusionPair { word1: "gasoline", word2: "gas", factor: 1000000 },
    L2ConfusionPair { word1: "gift", word2: "poison", factor: 10 },
    L2ConfusionPair { word1: "gross", word2: "big", factor: 1000 },
    L2ConfusionPair { word1: "gross", word2: "large", factor: 10000 },
    L2ConfusionPair { word1: "gross", word2: "tall", factor: 10 },
    L2ConfusionPair { word1: "handy", word2: "phone", factor: 10 },
    L2ConfusionPair { word1: "intern", word2: "internal", factor: 100000 },
    L2ConfusionPair { word1: "intern", word2: "domestic", factor: 10000000 },
    L2ConfusionPair { word1: "labor", word2: "laboratory", factor: 10 },
    L2ConfusionPair { word1: "local", word2: "pub", factor: 10 },
    L2ConfusionPair { word1: "local", word2: "inn", factor: 10000000 },
    L2ConfusionPair { word1: "map", word2: "folder", factor: 10 },
    L2ConfusionPair { word1: "meaning", word2: "opinion", factor: 1000 },
    L2ConfusionPair { word1: "mist", word2: "manure", factor: 10 },
    L2ConfusionPair { word1: "mist", word2: "dung", factor: 10 },
    L2ConfusionPair { word1: "note", word2: "mark", factor: 10 },
    L2ConfusionPair { word1: "note", word2: "grade", factor: 10 },
    L2ConfusionPair { word1: "ordinary", word2: "vulgar", factor: 10 },
    L2ConfusionPair { word1: "overhear", word2: "miss", factor: 10000000 },
    L2ConfusionPair { word1: "oversee", word2: "overlook", factor: 1000 },
    L2ConfusionPair { word1: "oversee", word2: "miss", factor: 100000 },
    L2ConfusionPair { word1: "oversight", word2: "overview", factor: 1000000 },
    L2ConfusionPair { word1: "pass", word2: "passport", factor: 10 },
    L2ConfusionPair { word1: "photograph", word2: "photographer", factor: 10 },
    L2ConfusionPair { word1: "preservative", word2: "condom", factor: 100 },
    L2ConfusionPair { word1: "proof", word2: "test", factor: 100000 },
    L2ConfusionPair { word1: "prospect", word2: "brochure", factor: 10 },
    L2ConfusionPair { word1: "prove", word2: "test", factor: 10000 },
    L2ConfusionPair { word1: "public", word2: "audience", factor: 10 },
    L2ConfusionPair { word1: "realize", word2: "produce", factor: 1000 },
    L2ConfusionPair { word1: "receipt", word2: "prescription", factor: 10 },
    L2ConfusionPair { word1: "rentable", word2: "profitable", factor: 10000 },
    L2ConfusionPair { word1: "rentable", word2: "viable", factor: 10000 },
    L2ConfusionPair { word1: "roman", word2: "novel", factor: 10 },
    L2ConfusionPair { word1: "sea", word2: "lake", factor: 10 },
    L2ConfusionPair { word1: "sensible", word2: "sensitive", factor: 10000 },
    L2ConfusionPair { word1: "serious", word2: "reliable", factor: 100000 },
    L2ConfusionPair { word1: "spare", word2: "save", factor: 1000000 },
    L2ConfusionPair { word1: "spend", word2: "donate", factor: 10 },
    L2ConfusionPair { word1: "stern", word2: "star", factor: 10000 },
    L2ConfusionPair { word1: "sympathetic", word2: "nice", factor: 1000000 },
    L2ConfusionPair { word1: "sympathetic", word2: "friendly", factor: 100000 },
    L2ConfusionPair { word1: "sympathy", word2: "liking", factor: 10 },
    L2ConfusionPair { word1: "sympathy", word2: "solidarity", factor: 10 },
    L2ConfusionPair { word1: "undertaker", word2: "employer", factor: 10000 },
    L2ConfusionPair { word1: "vocal", word2: "vowel", factor: 10 },
    L2ConfusionPair { word1: "vocals", word2: "vowels", factor: 1000 },
];

use std::collections::HashMap;
use std::sync::LazyLock;

/// Lookup map for L2 confusion pairs by word1
pub static EN_L2_DE_CONFUSION_LOOKUP: LazyLock<HashMap<String, &'static L2ConfusionPair>> = LazyLock::new(|| {
	let mut map = HashMap::new();
	for pair in EN_L2_DE_CONFUSION_PAIRS {
		map.insert(pair.word1.to_lowercase(), pair);
	}
	map
});

/// Check if a word might be a false friend for German native speakers
pub fn get_en_l2_de_confusion(word: &str) -> Option<&'static L2ConfusionPair> {
	EN_L2_DE_CONFUSION_LOOKUP.get(&word.to_lowercase()).copied()
}
