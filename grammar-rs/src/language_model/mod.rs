//! N-gram language model for probability-based error detection
//!
//! This module provides probability calculations using N-gram statistics
//! for detecting confusion errors (their/there/they're, etc.)
//!
//! Two storage formats are available:
//! - `NgramLanguageModel`: HashMap-based, good for small datasets or when building
//! - `CompactNgramModel`: Memory-mapped, instant loading, zero RAM for large datasets
//!
//! Download options:
//! - `ngram-download`: Basic single-connection download
//! - `ngram-download-fast`: Parallel download with compression and resume support

mod ngram_model;
mod probability;
mod compact_model;
mod builder;
pub mod downloader;

#[cfg(feature = "ngram-download-fast")]
pub mod parallel_downloader;

pub use ngram_model::{NgramLanguageModel, NgramData};
pub use probability::Probability;
pub use compact_model::{CompactNgramModel, NgramHeader, CompactModelStats};
pub use builder::{CompactNgramBuilder, BuildStats, StreamingNgramBuilder};
