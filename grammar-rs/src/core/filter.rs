//! Filter trait for marking regions to ignore
//!
//! Filters are used to identify regions of text that should be
//! excluded from grammar checking (URLs, code blocks, dates, etc.)

use super::MaskedRegion;

/// Trait for identifying regions to mask from grammar checking
pub trait Filter: Send + Sync {
    /// Find all regions in the text that should be masked
    fn find_masks(&self, text: &str) -> Vec<MaskedRegion>;

    /// Get a description of what this filter masks
    fn description(&self) -> &'static str;
}

/// A collection of filters that can be applied together
#[derive(Default)]
pub struct FilterChain {
    filters: Vec<Box<dyn Filter>>,
}

impl FilterChain {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a filter to the chain
    pub fn with_filter(mut self, filter: impl Filter + 'static) -> Self {
        self.filters.push(Box::new(filter));
        self
    }

    /// Add a boxed filter to the chain
    pub fn with_boxed_filter(mut self, filter: Box<dyn Filter>) -> Self {
        self.filters.push(filter);
        self
    }

    /// Find all masked regions from all filters
    pub fn find_all_masks(&self, text: &str) -> Vec<MaskedRegion> {
        let mut masks = Vec::new();
        for filter in &self.filters {
            masks.extend(filter.find_masks(text));
        }
        // Sort by start position
        masks.sort_by_key(|m| m.span.start);
        masks
    }

    /// Check if any filters are registered
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    /// Number of filters
    pub fn len(&self) -> usize {
        self.filters.len()
    }
}
