//! Date filter

use crate::core::filter::Filter;
use crate::core::{MaskKind, MaskedRegion};
use regex::Regex;

/// Filter that masks date formats
pub struct DateFilter {
    // ISO format: 2024-01-15, 2024/01/15
    iso_date: Regex,
    // US format: 01/15/2024, 01-15-2024
    us_date: Regex,
    // European format: 15/01/2024, 15.01.2024
    eu_date: Regex,
    // Named months: Jan 15, January 15th, 15 January 2024
    named_month: Regex,
    // Ordinal dates: 1st, 2nd, 3rd, 4th, etc.
    ordinal: Regex,
}

impl DateFilter {
    pub fn new() -> Self {
        Self {
            // ISO: YYYY-MM-DD or YYYY/MM/DD
            iso_date: Regex::new(
                r"\b\d{4}[-/]\d{1,2}[-/]\d{1,2}\b",
            ).expect("Invalid ISO date regex"),
            // US: MM/DD/YYYY or MM-DD-YYYY
            us_date: Regex::new(
                r"\b\d{1,2}[-/]\d{1,2}[-/]\d{2,4}\b",
            ).expect("Invalid US date regex"),
            // European: DD.MM.YYYY
            eu_date: Regex::new(
                r"\b\d{1,2}\.\d{1,2}\.\d{2,4}\b",
            ).expect("Invalid EU date regex"),
            // Named months (English and French)
            named_month: Regex::new(
                r"(?i)\b(?:(?:Jan(?:uary|vier)?|Feb(?:ruary|rier)?|Mar(?:ch|s)?|Apr(?:il)?|Avr(?:il)?|May|Mai|Jun(?:e)?|Juin|Jul(?:y)?|Juil(?:let)?|Aug(?:ust)?|Août|Sep(?:tember|tembre)?|Oct(?:ober|obre)?|Nov(?:ember|embre)?|Dec(?:ember|embre)?)\s+\d{1,2}(?:st|nd|rd|th)?(?:,?\s+\d{4})?|\d{1,2}(?:st|nd|rd|th)?\s+(?:Jan(?:uary|vier)?|Feb(?:ruary|rier)?|Mar(?:ch|s)?|Apr(?:il)?|Avr(?:il)?|May|Mai|Jun(?:e)?|Juin|Jul(?:y)?|Juil(?:let)?|Aug(?:ust)?|Août|Sep(?:tember|tembre)?|Oct(?:ober|obre)?|Nov(?:ember|embre)?|Dec(?:ember|embre)?)(?:,?\s+\d{4})?)\b",
            ).expect("Invalid named month regex"),
            // Ordinal numbers in dates context
            ordinal: Regex::new(
                r"\b\d{1,2}(?:st|nd|rd|th)\b",
            ).expect("Invalid ordinal regex"),
        }
    }
}

impl Default for DateFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl Filter for DateFilter {
    fn find_masks(&self, text: &str) -> Vec<MaskedRegion> {
        let mut masks = Vec::new();

        // Find ISO dates
        for m in self.iso_date.find_iter(text) {
            masks.push(MaskedRegion::new(m.start()..m.end(), MaskKind::Date));
        }

        // Find US dates
        for m in self.us_date.find_iter(text) {
            let span = m.start()..m.end();
            if !masks.iter().any(|mask| mask.overlaps(&span)) {
                masks.push(MaskedRegion::new(span, MaskKind::Date));
            }
        }

        // Find EU dates
        for m in self.eu_date.find_iter(text) {
            let span = m.start()..m.end();
            if !masks.iter().any(|mask| mask.overlaps(&span)) {
                masks.push(MaskedRegion::new(span, MaskKind::Date));
            }
        }

        // Find named month dates
        for m in self.named_month.find_iter(text) {
            let span = m.start()..m.end();
            if !masks.iter().any(|mask| mask.overlaps(&span)) {
                masks.push(MaskedRegion::new(span, MaskKind::Date));
            }
        }

        // Sort by start position
        masks.sort_by_key(|m| m.span.start);
        masks
    }

    fn description(&self) -> &'static str {
        "Dates"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_date() {
        let filter = DateFilter::new();
        let masks = filter.find_masks("Meeting on 2024-01-15 at noon.");
        assert_eq!(masks.len(), 1);
        assert_eq!(masks[0].kind, MaskKind::Date);
        assert_eq!(&"Meeting on 2024-01-15 at noon."[masks[0].span.clone()], "2024-01-15");
    }

    #[test]
    fn test_us_date() {
        let filter = DateFilter::new();
        let masks = filter.find_masks("Due by 01/15/2024.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_eu_date() {
        let filter = DateFilter::new();
        let masks = filter.find_masks("Le 15.01.2024 à midi.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_named_month() {
        let filter = DateFilter::new();
        let masks = filter.find_masks("Born on January 15th, 2024.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_named_month_day_first() {
        let filter = DateFilter::new();
        let masks = filter.find_masks("Born on 15 January 2024.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_french_month() {
        let filter = DateFilter::new();
        let masks = filter.find_masks("Rendez-vous le 15 janvier 2024.");
        assert_eq!(masks.len(), 1);
    }

    #[test]
    fn test_multiple_dates() {
        let filter = DateFilter::new();
        let masks = filter.find_masks("From 2024-01-01 to 2024-12-31.");
        assert_eq!(masks.len(), 2);
    }

    #[test]
    fn test_no_dates() {
        let filter = DateFilter::new();
        let masks = filter.find_masks("This text has no dates.");
        assert!(masks.is_empty());
    }
}
