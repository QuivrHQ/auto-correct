//! Auto-generated L2 confusion pairs for Spanish native speakers writing EN
//! Synced: 2026-01-23T13:30:57.017252854+00:00
//! Total pairs: 26
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool confusion_sets_l2_es.txt
//! License: LGPL 2.1+
//!
//! False friends and common mistakes made by Spanish native speakers.

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

/// L2 confusion pairs for Spanish native speakers (sorted by word1)
/// Total pairs: 26
pub const EN_L2_ES_CONFUSION_PAIRS: &[L2ConfusionPair] = &[
    L2ConfusionPair { word1: "accommodation", word2: "adaptation", factor: 2 },
    L2ConfusionPair { word1: "accommodation", word2: "arrangement", factor: 100 },
    L2ConfusionPair { word1: "actually", word2: "currently", factor: 1 },
    L2ConfusionPair { word1: "adept", word2: "follower", factor: 5 },
    L2ConfusionPair { word1: "adept", word2: "admirer", factor: 2 },
    L2ConfusionPair { word1: "alternatively", word2: "alternately", factor: 1 },
    L2ConfusionPair { word1: "billet", word2: "ticket", factor: 1000000 },
    L2ConfusionPair { word1: "cartoon", word2: "cardboard", factor: 5 },
    L2ConfusionPair { word1: "comprehensive", word2: "understandable", factor: 1 },
    L2ConfusionPair { word1: "consequently", word2: "consistently", factor: 1 },
    L2ConfusionPair { word1: "cymbal", word2: "decoy", factor: 1000 },
    L2ConfusionPair { word1: "eventually", word2: "possibly", factor: 5 },
    L2ConfusionPair { word1: "fabric", word2: "factory", factor: 100 },
    L2ConfusionPair { word1: "form", word2: "shape", factor: 2 },
    L2ConfusionPair { word1: "notice", word2: "news", factor: 5 },
    L2ConfusionPair { word1: "physicist", word2: "physician", factor: 100 },
    L2ConfusionPair { word1: "preservative", word2: "condom", factor: 2 },
    L2ConfusionPair { word1: "prevent", word2: "alert", factor: 1 },
    L2ConfusionPair { word1: "realize", word2: "produce", factor: 1 },
    L2ConfusionPair { word1: "regular", word2: "stable", factor: 1 },
    L2ConfusionPair { word1: "regular", word2: "steady", factor: 1 },
    L2ConfusionPair { word1: "regular", word2: "mediocre", factor: 1 },
    L2ConfusionPair { word1: "rent", word2: "pension", factor: 1 },
    L2ConfusionPair { word1: "rentable", word2: "profitable", factor: 1 },
    L2ConfusionPair { word1: "sensible", word2: "sensitive", factor: 1000 },
    L2ConfusionPair { word1: "understandable", word2: "comprehensive", factor: 10000 },
];

use std::collections::HashMap;
use std::sync::LazyLock;

/// Lookup map for L2 confusion pairs by word1
pub static EN_L2_ES_CONFUSION_LOOKUP: LazyLock<HashMap<String, &'static L2ConfusionPair>> = LazyLock::new(|| {
	let mut map = HashMap::new();
	for pair in EN_L2_ES_CONFUSION_PAIRS {
		map.insert(pair.word1.to_lowercase(), pair);
	}
	map
});

/// Check if a word might be a false friend for Spanish native speakers
pub fn get_en_l2_es_confusion(word: &str) -> Option<&'static L2ConfusionPair> {
	EN_L2_ES_CONFUSION_LOOKUP.get(&word.to_lowercase()).copied()
}
