//! grammar-rs API Server
//!
//! A high-performance LanguageTool-compatible grammar checking API.
//!
//! ## Endpoints
//!
//! - `POST /v2/check` - Check text for grammar/spelling errors
//! - `GET /v2/languages` - List supported languages
//! - `GET /` - Health check
//!
//! ## Usage
//!
//! ```bash
//! # Start the server
//! cargo run --release --bin grammar-api
//!
//! # Test with curl
//! curl -X POST http://localhost:8081/v2/check \
//!   -d "text=I have a apple&language=en"
//! ```

mod convert;
mod handlers;
mod state;
mod types;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use state::AppState;
use handlers::{check_handler, languages_handler, health_handler};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    tracing::info!("Starting grammar-rs API server...");

    // Configure rayon thread pool for CPU parallelism
    let max_cpus = std::env::var("MAX_CPUS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0); // 0 = use all CPUs

    let actual_cpus = grammar_rs::core::pipeline::num_cpus();
    let threads = if max_cpus == 0 { actual_cpus } else { max_cpus.min(actual_cpus) };

    grammar_rs::core::pipeline::init_thread_pool(threads);
    tracing::info!("Rayon thread pool: {} threads (MAX_CPUS={})", threads,
        if max_cpus == 0 { "auto".to_string() } else { max_cpus.to_string() });

    // Pre-warm lazy statics for faster first request
    tracing::info!("Pre-warming lazy statics...");
    grammar_rs::warm_up();

    // Build application state (pre-initialize pipelines)
    let state = Arc::new(AppState::new());

    // Configure CORS (permissive for browser extension compatibility)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/", get(health_handler))
        .route("/v2/check", post(check_handler))
        .route("/v2/languages", get(languages_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Bind to port
    let port = std::env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on http://{}", addr);

    // Run server
    axum::serve(listener, app).await.unwrap();
}
