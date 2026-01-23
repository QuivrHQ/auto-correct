//! Auto-generated L2 confusion pairs for Dutch native speakers writing EN
//! Synced: 2026-01-23T13:30:57.019823679+00:00
//! Total pairs: 11
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool confusion_sets_l2_nl.txt
//! License: LGPL 2.1+
//!
//! False friends and common mistakes made by Dutch native speakers.

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

/// L2 confusion pairs for Dutch native speakers (sorted by word1)
/// Total pairs: 11
pub const EN_L2_NL_CONFUSION_PAIRS: &[L2ConfusionPair] = &[
    L2ConfusionPair { word1: "actual", word2: "current", factor: 10 },
    L2ConfusionPair { word1: "actually", word2: "currently", factor: 10 },
    L2ConfusionPair { word1: "agenda", word2: "diary", factor: 100 },
    L2ConfusionPair { word1: "antics", word2: "antique", factor: 100 },
    L2ConfusionPair { word1: "barracks", word2: "huts", factor: 100 },
    L2ConfusionPair { word1: "diary", word2: "agenda", factor: 100 },
    L2ConfusionPair { word1: "intern", word2: "domestic", factor: 1000000 },
    L2ConfusionPair { word1: "lake", word2: "sea", factor: 100 },
    L2ConfusionPair { word1: "map", word2: "folder", factor: 10 },
    L2ConfusionPair { word1: "meaning", word2: "opinion", factor: 10 },
    L2ConfusionPair { word1: "want", word2: "wall", factor: 10 },
];

use std::collections::HashMap;
use std::sync::LazyLock;

/// Lookup map for L2 confusion pairs by word1
pub static EN_L2_NL_CONFUSION_LOOKUP: LazyLock<HashMap<String, &'static L2ConfusionPair>> = LazyLock::new(|| {
	let mut map = HashMap::new();
	for pair in EN_L2_NL_CONFUSION_PAIRS {
		map.insert(pair.word1.to_lowercase(), pair);
	}
	map
});

/// Check if a word might be a false friend for Dutch native speakers
pub fn get_en_l2_nl_confusion(word: &str) -> Option<&'static L2ConfusionPair> {
	EN_L2_NL_CONFUSION_LOOKUP.get(&word.to_lowercase()).copied()
}
