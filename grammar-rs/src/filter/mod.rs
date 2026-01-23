//! Filters for reducing false positives
//!
//! This module provides filters that identify text regions to exclude
//! from grammar checking, such as URLs, code blocks, dates, etc.

mod url;
mod code;
mod quoted;
mod date;
mod number;

pub use url::UrlFilter;
pub use code::CodeBlockFilter;
pub use quoted::QuotedTextFilter;
pub use date::DateFilter;
pub use number::NumberFilter;

use crate::core::filter::{Filter, FilterChain};

/// Create a filter chain with all default filters
pub fn default_filters() -> FilterChain {
    FilterChain::new()
        .with_filter(UrlFilter::new())
        .with_filter(CodeBlockFilter::new())
        .with_filter(QuotedTextFilter::new())
        .with_filter(DateFilter::new())
        .with_filter(NumberFilter::new())
}

/// Create a filter chain with specific filters
pub struct FilterBuilder {
    chain: FilterChain,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            chain: FilterChain::new(),
        }
    }

    pub fn with_url_filter(self) -> Self {
        Self {
            chain: self.chain.with_filter(UrlFilter::new()),
        }
    }

    pub fn with_code_filter(self) -> Self {
        Self {
            chain: self.chain.with_filter(CodeBlockFilter::new()),
        }
    }

    pub fn with_quoted_filter(self) -> Self {
        Self {
            chain: self.chain.with_filter(QuotedTextFilter::new()),
        }
    }

    pub fn with_date_filter(self) -> Self {
        Self {
            chain: self.chain.with_filter(DateFilter::new()),
        }
    }

    pub fn with_number_filter(self) -> Self {
        Self {
            chain: self.chain.with_filter(NumberFilter::new()),
        }
    }

    pub fn build(self) -> FilterChain {
        self.chain
    }
}

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
