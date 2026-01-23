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
