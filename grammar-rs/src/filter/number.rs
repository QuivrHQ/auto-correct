//! Hyphenated number filter

use crate::core::filter::Filter;
use crate::core::{MaskKind, MaskedRegion};
use regex::Regex;

/// Filter that masks hyphenated number words
pub struct NumberFilter {
    // Hyphenated numbers: twenty-one, thirty-five, etc.
    hyphenated_number: Regex,
}

impl NumberFilter {
    pub fn new() -> Self {
        // Number words that can be hyphenated
        let units = "one|two|three|four|five|six|seven|eight|nine";
        let tens = "twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety";

        // French number words
        let fr_units = "un|deux|trois|quatre|cinq|six|sept|huit|neuf";
        let fr_tens = "vingt|trente|quarante|cinquante|soixante";

        // Pattern for hyphenated numbers (21-99 in words)
        // Only matches hyphenated compound numbers like "twenty-one", not standalone "ten"
        let pattern = format!(
            r"(?i)\b(?:(?:{tens})-(?:{units})|(?:{fr_tens})-(?:{fr_units}))\b",
            tens = tens,
            units = units,
            fr_tens = fr_tens,
            fr_units = fr_units,
        );

        Self {
            hyphenated_number: Regex::new(&pattern).expect("Invalid hyphenated number regex"),
        }
    }
}

impl Default for NumberFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl Filter for NumberFilter {
    fn find_masks(&self, text: &str) -> Vec<MaskedRegion> {
        let mut masks = Vec::new();

        for m in self.hyphenated_number.find_iter(text) {
            masks.push(MaskedRegion::new(m.start()..m.end(), MaskKind::HyphenatedNumber));
        }

        masks
    }

    fn description(&self) -> &'static str {
        "Hyphenated numbers"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twenty_one() {
        let filter = NumberFilter::new();
        let masks = filter.find_masks("There are twenty-one items.");
        assert_eq!(masks.len(), 1);
        assert_eq!(masks[0].kind, MaskKind::HyphenatedNumber);
        assert_eq!(&"There are twenty-one items."[masks[0].span.clone()], "twenty-one");
    }

    #[test]
    fn test_thirty_five() {
        let filter = NumberFilter::new();
        let masks = filter.find_masks("She is thirty-five years old.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_ninety_nine() {
        let filter = NumberFilter::new();
        let masks = filter.find_masks("Only ninety-nine bottles.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_french_number() {
        let filter = NumberFilter::new();
        let masks = filter.find_masks("Il y a vingt-trois personnes.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_case_insensitive() {
        let filter = NumberFilter::new();
        let masks = filter.find_masks("TWENTY-ONE and Twenty-Two.");
        assert_eq!(masks.len(), 2);
    }

    #[test]
    fn test_no_hyphenated_numbers() {
        let filter = NumberFilter::new();
        let masks = filter.find_masks("Ten plus ten equals twenty.");
        // No hyphenated numbers, just standalone number words
        assert!(masks.is_empty());
    }

    #[test]
    fn test_multiple_numbers() {
        let filter = NumberFilter::new();
        let masks = filter.find_masks("Twenty-one, twenty-two, twenty-three.");
        assert_eq!(masks.len(), 3);
    }
}
