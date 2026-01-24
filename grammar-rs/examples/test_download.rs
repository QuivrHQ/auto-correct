//! Test the parallel downloader
//!
//! Run with:
//! ```bash
//! GRAMMAR_RS_AUTO_DOWNLOAD=1 cargo run --features ngram-download-fast --example test_download
//! ```

use std::path::Path;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let data_dir = Path::new("/tmp/test_ngram_download");
    std::fs::create_dir_all(data_dir).expect("Failed to create test dir");

    println!("Testing parallel download with compression...");
    println!("Data directory: {:?}", data_dir);
    println!();

    // Get language from args, default to FR
    let lang = std::env::args().nth(1).unwrap_or_else(|| "fr".to_string());
    println!("Language: {}", lang.to_uppercase());

    // Clean any existing files for this language
    let _ = std::fs::remove_file(data_dir.join(format!("{}_ngrams.bin", lang)));
    let _ = std::fs::remove_dir_all(data_dir.join(format!("{}_ngrams.bin.zst.parts", lang)));
    let _ = std::fs::remove_file(data_dir.join(format!("{}_ngrams.bin.zst.progress.json", lang)));

    match grammar_rs::language_model::downloader::ensure_ngram_data(&lang, data_dir) {
        Ok(true) => {
            println!();
            println!("SUCCESS! FR N-gram data downloaded and verified.");

            // Check file size
            let file_path = data_dir.join("fr_ngrams.bin");
            if let Ok(metadata) = std::fs::metadata(&file_path) {
                let size_gb = metadata.len() as f64 / (1024.0 * 1024.0 * 1024.0);
                println!("File size: {:.2} GB", size_gb);
            }
        }
        Ok(false) => {
            println!("Download skipped (auto-download disabled)");
            println!("Set GRAMMAR_RS_AUTO_DOWNLOAD=1 to enable");
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}
