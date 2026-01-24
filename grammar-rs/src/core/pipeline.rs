//! Pipeline composable - assemble les étapes
//!
//! C'est ici que la magie opère : tu peux swapper n'importe quelle
//! implémentation sans changer le reste du code.

use super::filter::FilterChain;
use super::traits::{Analyzer, Checker, GrammarChecker, Tokenizer};
use super::CheckResult;
use rayon::prelude::*;
use std::sync::Arc;

/// Initialize the rayon thread pool with a specific number of threads.
/// Call this at application startup to limit CPU usage.
///
/// # Arguments
/// * `num_threads` - Maximum number of threads to use (0 = use all available CPUs)
///
/// # Example
/// ```
/// use grammar_rs::core::pipeline::init_thread_pool;
/// init_thread_pool(4); // Use max 4 CPUs
/// ```
///
/// # Panics
/// Panics if called more than once (rayon only allows one global pool initialization).
pub fn init_thread_pool(num_threads: usize) {
    let threads = if num_threads == 0 {
        num_cpus()
    } else {
        num_threads
    };

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .expect("Failed to initialize rayon thread pool");
}

/// Returns the number of CPUs available on this system.
pub fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1)
}

/// Le pipeline principal - compose les étapes
pub struct Pipeline {
    tokenizer: Arc<dyn Tokenizer>,
    analyzer: Arc<dyn Analyzer>,
    checkers: Vec<Arc<dyn Checker>>,
    filters: Option<FilterChain>,
}

impl Pipeline {
    pub fn new(
        tokenizer: impl Tokenizer + 'static,
        analyzer: impl Analyzer + 'static,
    ) -> Self {
        Self {
            tokenizer: Arc::new(tokenizer),
            analyzer: Arc::new(analyzer),
            checkers: Vec::new(),
            filters: None,
        }
    }

    /// Ajoute un checker au pipeline (builder pattern)
    pub fn with_checker(mut self, checker: impl Checker + 'static) -> Self {
        self.checkers.push(Arc::new(checker));
        self
    }

    /// Ajoute plusieurs checkers
    pub fn with_checkers(mut self, checkers: Vec<Arc<dyn Checker>>) -> Self {
        self.checkers.extend(checkers);
        self
    }

    /// Ajoute une chaîne de filtres pour réduire les faux positifs
    pub fn with_filters(mut self, filters: FilterChain) -> Self {
        self.filters = Some(filters);
        self
    }

    /// Ajoute les filtres par défaut (URLs, code, quotes, dates, numbers)
    pub fn with_default_filters(mut self) -> Self {
        self.filters = Some(crate::filter::default_filters());
        self
    }
}

impl GrammarChecker for Pipeline {
    fn check_text(&self, text: &str) -> CheckResult {
        // Étape 0: Find masked regions (if filters are configured)
        let masks = self.filters.as_ref().map(|f| f.find_all_masks(text));

        // Étape 1: Tokenize
        let tokens = self.tokenizer.tokenize(text);

        // Étape 2: Analyze
        let analyzed = self.analyzer.analyze(tokens);

        // Étape 3: Check (tous les checkers en parallèle avec rayon)
        let results: Vec<CheckResult> = self
            .checkers
            .par_iter()
            .map(|checker| checker.check(text, &analyzed))
            .collect();

        let mut result = CheckResult::new();
        for r in results {
            result.merge(r);
        }

        // Étape 4: Filter out matches in masked regions
        if let Some(ref masks) = masks {
            result = result.filter_masked(masks);
        }

        // Nettoyer et trier
        result.sort_and_dedupe();
        result
    }
}
