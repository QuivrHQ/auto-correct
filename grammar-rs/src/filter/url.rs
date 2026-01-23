//! URL and email filter

use crate::core::filter::Filter;
use crate::core::{MaskKind, MaskedRegion};
use regex::Regex;

/// Filter that masks URLs and email addresses
pub struct UrlFilter {
    url_regex: Regex,
    email_regex: Regex,
}

impl UrlFilter {
    pub fn new() -> Self {
        Self {
            // URL pattern: http(s)://, ftp://, or www.
            url_regex: Regex::new(
                r#"(?i)(?:https?://|ftp://|www\.)[^\s<>\[\](){}"'`]+"#,
            ).expect("Invalid URL regex"),
            // Email pattern: local@domain.tld
            email_regex: Regex::new(
                r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
            ).expect("Invalid email regex"),
        }
    }
}

impl Default for UrlFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl Filter for UrlFilter {
    fn find_masks(&self, text: &str) -> Vec<MaskedRegion> {
        let mut masks = Vec::new();

        // Find URLs
        for m in self.url_regex.find_iter(text) {
            masks.push(MaskedRegion::new(m.start()..m.end(), MaskKind::Url));
        }

        // Find emails
        for m in self.email_regex.find_iter(text) {
            masks.push(MaskedRegion::new(m.start()..m.end(), MaskKind::Url));
        }

        masks
    }

    fn description(&self) -> &'static str {
        "URLs and email addresses"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_filter_https() {
        let filter = UrlFilter::new();
        let masks = filter.find_masks("Check out https://example.com/path for more info.");
        assert_eq!(masks.len(), 1);
        assert_eq!(masks[0].kind, MaskKind::Url);
        assert_eq!(&"Check out https://example.com/path for more info."[masks[0].span.clone()], "https://example.com/path");
    }

    #[test]
    fn test_url_filter_www() {
        let filter = UrlFilter::new();
        let masks = filter.find_masks("Visit www.example.com today.");
        assert_eq!(masks.len(), 1);
        assert_eq!(&"Visit www.example.com today."[masks[0].span.clone()], "www.example.com");
    }

    #[test]
    fn test_email_filter() {
        let filter = UrlFilter::new();
        let masks = filter.find_masks("Contact us at support@example.com for help.");
        assert_eq!(masks.len(), 1);
        assert_eq!(masks[0].kind, MaskKind::Url);
        assert_eq!(&"Contact us at support@example.com for help."[masks[0].span.clone()], "support@example.com");
    }

    #[test]
    fn test_multiple_urls() {
        let filter = UrlFilter::new();
        let masks = filter.find_masks("Check https://a.com and email@b.com");
        assert_eq!(masks.len(), 2);
    }

    #[test]
    fn test_no_urls() {
        let filter = UrlFilter::new();
        let masks = filter.find_masks("This is normal text without URLs.");
        assert!(masks.is_empty());
    }
}
