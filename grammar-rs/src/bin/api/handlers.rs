//! HTTP request handlers

use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::convert::convert_result;
use crate::state::AppState;
use crate::types::*;

use grammar_rs::prelude::{GrammarChecker, SimpleTokenizer, PassthroughAnalyzer};
use grammar_rs::core::traits::{Tokenizer, Analyzer, Checker};
use grammar_rs::checker::L2ConfusionChecker;

/// Handle POST /v2/check
///
/// Main grammar checking endpoint, compatible with LanguageTool API
pub async fn check_handler(
    State(state): State<Arc<AppState>>,
    Form(req): Form<CheckRequest>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();

    // Detect or use specified language
    let (lang_code, confidence) = if req.language == "auto" {
        let detection = state.language_detector.detect(&req.text);
        let code = detection.code().to_string();
        let conf = if detection == grammar_rs::lang_detect::Language::Unknown {
            0.5
        } else {
            0.95
        };
        (code, conf)
    } else {
        (normalize_language(&req.language), 1.0)
    };

    // Create cache key
    let cache_key = crate::types::CacheKey::from_request(&req, &lang_code);

    // Check cache first
    if let Some(cached_response) = state.cache.get(&cache_key).await {
        let elapsed = start.elapsed();
        tracing::info!(
            lang = %lang_code,
            text_len = req.text.len(),
            elapsed_ms = elapsed.as_millis(),
            cache_hit = true,
            "Check completed (cache hit)"
        );
        return Json((*cached_response).clone());
    }

    // Cache miss - compute result
    let text = req.text.clone();
    let pipeline_clone = Arc::clone(if lang_code.starts_with("fr") {
        &state.fr_pipeline
    } else {
        &state.en_pipeline
    });

    // Check if L2 French confusion checking should be enabled
    let use_l2_fr = req.mother_tongue.as_deref() == Some("fr") && !lang_code.starts_with("fr");

    let result = tokio::task::spawn_blocking(move || {
        let mut result = pipeline_clone.check_text(&text);

        // Add L2 French confusion checking for French native speakers writing in English
        if use_l2_fr {
            let l2_checker = L2ConfusionChecker::new();
            let tokenizer = SimpleTokenizer::new();
            let analyzer = PassthroughAnalyzer::new();
            let tokens = tokenizer.tokenize(&text);
            let analyzed = analyzer.analyze(tokens);
            let l2_result = l2_checker.check(&text, &analyzed);
            result.matches.extend(l2_result.matches);
        }

        result
    })
    .await
    .unwrap();

    // Convert to LanguageTool format
    let response = convert_result(result, &req.text, &lang_code, confidence);

    // Store in cache
    let response_arc = Arc::new(response.clone());
    state.cache.insert(cache_key, response_arc).await;

    let elapsed = start.elapsed();
    tracing::info!(
        lang = %lang_code,
        mother_tongue = ?req.mother_tongue,
        matches = response.matches.len(),
        text_len = req.text.len(),
        elapsed_ms = elapsed.as_millis(),
        cache_hit = false,
        "Check completed (cache miss)"
    );

    Json(response)
}

/// Handle GET /v2/languages
///
/// Returns the list of supported languages
pub async fn languages_handler() -> impl IntoResponse {
    let languages = vec![
        LanguageResponse {
            name: "English (US)".to_string(),
            code: "en".to_string(),
            long_code: "en-US".to_string(),
        },
        LanguageResponse {
            name: "English (GB)".to_string(),
            code: "en".to_string(),
            long_code: "en-GB".to_string(),
        },
        LanguageResponse {
            name: "French".to_string(),
            code: "fr".to_string(),
            long_code: "fr-FR".to_string(),
        },
        LanguageResponse {
            name: "French (Canada)".to_string(),
            code: "fr".to_string(),
            long_code: "fr-CA".to_string(),
        },
    ];

    Json(languages)
}

/// Handle GET / (health check)
pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Normalize language code to a standard format
fn normalize_language(lang: &str) -> String {
    match lang.to_lowercase().as_str() {
        "en" | "en-us" | "english" => "en-US".to_string(),
        "en-gb" | "en-uk" => "en-GB".to_string(),
        "fr" | "fr-fr" | "french" => "fr-FR".to_string(),
        "fr-ca" => "fr-CA".to_string(),
        other => other.to_string(),
    }
}
