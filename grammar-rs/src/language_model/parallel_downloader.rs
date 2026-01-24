//! Fast parallel downloader with compression and resume support
//!
//! This module provides high-speed downloads using:
//! - Parallel HTTP Range requests (16 connections)
//! - Zstd compression (3-4x smaller files)
//! - Resume capability (survives interruptions)
//!
//! Enable with feature flag `ngram-download-fast`:
//! ```bash
//! GRAMMAR_RS_AUTO_DOWNLOAD=1 cargo run --features ngram-download-fast
//! ```

use std::fs::{self, File};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::collections::HashMap;

use futures::stream::{self, StreamExt};
use reqwest::Client;
use sha2::{Sha256, Digest};
use tokio::sync::Semaphore;

/// Number of parallel connections for downloading
const PARALLEL_CONNECTIONS: usize = 16;

/// Chunk size for parallel downloads (20 MB)
const CHUNK_SIZE: u64 = 20 * 1024 * 1024;

/// Buffer size for file operations (1 MB)
const BUFFER_SIZE: usize = 1024 * 1024;

/// Progress file for tracking download state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DownloadProgress {
    pub version: u32,
    pub url: String,
    pub compressed_size: u64,
    pub decompressed_sha256: Option<String>,
    pub chunk_size: u64,
    pub total_chunks: u64,
    pub chunks: HashMap<u64, ChunkStatus>,
    pub started_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChunkStatus {
    pub status: String, // "pending", "partial", "complete"
    pub sha256: Option<String>,
    pub bytes_downloaded: Option<u64>,
}

impl DownloadProgress {
    fn new(url: &str, compressed_size: u64, decompressed_sha256: Option<String>) -> Self {
        let total_chunks = (compressed_size + CHUNK_SIZE - 1) / CHUNK_SIZE;
        let now = chrono::Utc::now().to_rfc3339();

        let mut chunks = HashMap::new();
        for i in 0..total_chunks {
            chunks.insert(i, ChunkStatus {
                status: "pending".to_string(),
                sha256: None,
                bytes_downloaded: None,
            });
        }

        Self {
            version: 1,
            url: url.to_string(),
            compressed_size,
            decompressed_sha256,
            chunk_size: CHUNK_SIZE,
            total_chunks,
            chunks,
            started_at: now.clone(),
            updated_at: now,
        }
    }

    fn progress_file(target: &Path) -> PathBuf {
        target.with_extension("zst.progress.json")
    }

    fn parts_dir(target: &Path) -> PathBuf {
        target.with_extension("zst.parts")
    }

    fn load(target: &Path) -> Option<Self> {
        let progress_file = Self::progress_file(target);
        if progress_file.exists() {
            let content = fs::read_to_string(&progress_file).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    fn save(&self, target: &Path) -> io::Result<()> {
        let progress_file = Self::progress_file(target);
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(&progress_file, content)?;
        Ok(())
    }

    fn cleanup(target: &Path) -> io::Result<()> {
        let progress_file = Self::progress_file(target);
        let parts_dir = Self::parts_dir(target);

        if progress_file.exists() {
            fs::remove_file(&progress_file)?;
        }
        if parts_dir.exists() {
            fs::remove_dir_all(&parts_dir)?;
        }
        Ok(())
    }

    fn pending_chunks(&self) -> Vec<u64> {
        self.chunks
            .iter()
            .filter(|(_, status)| status.status != "complete")
            .map(|(idx, _)| *idx)
            .collect()
    }

    fn mark_complete(&mut self, chunk_idx: u64, sha256: String) {
        let byte_size = self.chunk_byte_size(chunk_idx);
        if let Some(status) = self.chunks.get_mut(&chunk_idx) {
            status.status = "complete".to_string();
            status.sha256 = Some(sha256);
            status.bytes_downloaded = Some(byte_size);
        }
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    fn chunk_byte_size(&self, chunk_idx: u64) -> u64 {
        let start = chunk_idx * self.chunk_size;
        let end = ((chunk_idx + 1) * self.chunk_size).min(self.compressed_size);
        end - start
    }

    fn all_complete(&self) -> bool {
        self.chunks.values().all(|s| s.status == "complete")
    }
}

/// Download a file using parallel connections with compression and resume support
///
/// # Arguments
/// * `url` - URL to the compressed file (.zst)
/// * `target` - Final destination path (uncompressed)
/// * `decompressed_sha256` - Expected SHA256 of the decompressed file
pub async fn download_parallel_compressed(
    url: &str,
    target: &Path,
    decompressed_sha256: Option<&str>,
) -> io::Result<()> {
    let client = Client::builder()
        .pool_max_idle_per_host(PARALLEL_CONNECTIONS)
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Get file size with HEAD request
    let head_resp = client.head(url).send().await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("HEAD request failed: {}", e)))?;

    if !head_resp.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("HTTP {} from {}", head_resp.status(), url),
        ));
    }

    let compressed_size = head_resp
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing content-length header"))?;

    // Check if server supports range requests
    let accepts_ranges = head_resp
        .headers()
        .get("accept-ranges")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("bytes"))
        .unwrap_or(false);

    if !accepts_ranges {
        tracing::warn!("Server doesn't support range requests, falling back to single connection");
        return download_single_compressed(url, target, decompressed_sha256).await;
    }

    let compressed_mb = compressed_size / (1024 * 1024);
    tracing::info!("  Compressed size: {} MB ({} parallel connections)", compressed_mb, PARALLEL_CONNECTIONS);

    // Load or create progress
    let mut progress = DownloadProgress::load(target)
        .filter(|p| p.url == url && p.compressed_size == compressed_size)
        .unwrap_or_else(|| {
            DownloadProgress::new(url, compressed_size, decompressed_sha256.map(|s| s.to_string()))
        });

    // Create parts directory
    let parts_dir = DownloadProgress::parts_dir(target);
    fs::create_dir_all(&parts_dir)?;

    // Get pending chunks
    let pending = progress.pending_chunks();
    let pending_count = pending.len();

    if pending_count > 0 {
        if pending_count < progress.total_chunks as usize {
            tracing::info!("  Resuming: {} of {} chunks remaining", pending_count, progress.total_chunks);
        }

        // Download chunks in parallel
        let semaphore = Arc::new(Semaphore::new(PARALLEL_CONNECTIONS));
        let client = Arc::new(client);
        let parts_dir = Arc::new(parts_dir.clone());
        let url = Arc::new(url.to_string());
        let chunk_size = progress.chunk_size;
        let total_size = progress.compressed_size;

        let downloaded = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let total_to_download: u64 = pending.iter().map(|&idx| progress.chunk_byte_size(idx)).sum();

        let results: Vec<_> = stream::iter(pending.clone())
            .map(|chunk_idx| {
                let semaphore = semaphore.clone();
                let client = client.clone();
                let parts_dir = parts_dir.clone();
                let url = url.clone();
                let downloaded = downloaded.clone();

                async move {
                    let _permit = semaphore.acquire().await.unwrap();

                    let start = chunk_idx * chunk_size;
                    let end = ((chunk_idx + 1) * chunk_size - 1).min(total_size - 1);

                    let chunk_path = parts_dir.join(format!("chunk_{:04}.part", chunk_idx));

                    let result = download_chunk(&client, &url, start, end, &chunk_path).await;

                    if result.is_ok() {
                        let chunk_bytes = end - start + 1;
                        let prev = downloaded.fetch_add(chunk_bytes, std::sync::atomic::Ordering::Relaxed);
                        let progress_pct = ((prev + chunk_bytes) * 100) / total_to_download;
                        if progress_pct % 5 == 0 || chunk_idx == 0 {
                            tracing::info!(
                                "  Progress: {}% ({} / {} MB)",
                                progress_pct,
                                (prev + chunk_bytes) / (1024 * 1024),
                                total_to_download / (1024 * 1024)
                            );
                        }
                    }

                    result.map(|sha256| (chunk_idx, sha256))
                }
            })
            .buffer_unordered(PARALLEL_CONNECTIONS)
            .collect()
            .await;

        // Update progress with completed chunks
        for result in results {
            match result {
                Ok((chunk_idx, sha256)) => {
                    progress.mark_complete(chunk_idx, sha256);
                }
                Err(e) => {
                    tracing::error!("Chunk download failed: {}", e);
                    progress.save(target)?;
                    return Err(e);
                }
            }
        }

        progress.save(target)?;
    }

    if !progress.all_complete() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Not all chunks completed",
        ));
    }

    tracing::info!("  All chunks downloaded, assembling and decompressing...");

    // Assemble chunks and decompress
    let parts_dir = DownloadProgress::parts_dir(target);
    assemble_and_decompress(&parts_dir, target, progress.total_chunks, decompressed_sha256).await?;

    // Cleanup
    DownloadProgress::cleanup(target)?;

    tracing::info!("  Download and decompression complete!");

    Ok(())
}

/// Download a single chunk with Range header
async fn download_chunk(
    client: &Client,
    url: &str,
    start: u64,
    end: u64,
    chunk_path: &Path,
) -> io::Result<String> {
    let response = client
        .get(url)
        .header("Range", format!("bytes={}-{}", start, end))
        .send()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Chunk request failed: {}", e)))?;

    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("HTTP {} for chunk", response.status()),
        ));
    }

    let bytes = response.bytes().await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to read chunk: {}", e)))?;

    // Calculate SHA256
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let sha256 = format!("{:x}", hasher.finalize());

    // Write chunk to file
    let mut file = File::create(chunk_path)?;
    file.write_all(&bytes)?;
    file.flush()?;

    Ok(sha256)
}

/// Assemble chunks and decompress with zstd
async fn assemble_and_decompress(
    parts_dir: &Path,
    target: &Path,
    total_chunks: u64,
    expected_sha256: Option<&str>,
) -> io::Result<()> {
    // Create a reader that concatenates all chunks
    let chunk_readers: Vec<_> = (0..total_chunks)
        .map(|i| {
            let chunk_path = parts_dir.join(format!("chunk_{:04}.part", i));
            File::open(&chunk_path)
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Chain all chunk readers
    let combined_reader = ChainedReader::new(chunk_readers);
    let buffered_reader = BufReader::with_capacity(BUFFER_SIZE, combined_reader);

    // Create zstd decoder
    let decoder = zstd::stream::read::Decoder::new(buffered_reader)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Zstd init failed: {}", e)))?;

    // Create output file with hasher
    let temp_target = target.with_extension("bin.tmp");
    let output_file = File::create(&temp_target)?;
    let mut buffered_writer = BufWriter::with_capacity(BUFFER_SIZE, output_file);

    // Decompress and calculate SHA256 simultaneously
    let mut hasher = Sha256::new();
    let mut decoder = decoder;
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut total_written = 0u64;

    loop {
        let bytes_read = decoder.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
        buffered_writer.write_all(&buffer[..bytes_read])?;
        total_written += bytes_read as u64;

        // Log progress every 500 MB
        if total_written % (500 * 1024 * 1024) == 0 {
            tracing::info!("  Decompressed: {} MB", total_written / (1024 * 1024));
        }
    }

    buffered_writer.flush()?;
    drop(buffered_writer);

    let actual_sha256 = format!("{:x}", hasher.finalize());

    tracing::info!("  Decompressed size: {} MB", total_written / (1024 * 1024));
    tracing::info!("  SHA256: {}", actual_sha256);

    // Verify checksum if provided
    if let Some(expected) = expected_sha256 {
        if actual_sha256.to_lowercase() != expected.to_lowercase() {
            fs::remove_file(&temp_target)?;
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Checksum mismatch!\n  Expected: {}\n  Got:      {}",
                    expected, actual_sha256
                ),
            ));
        }
        tracing::info!("  Checksum verified OK");
    }

    // Move to final location
    fs::rename(&temp_target, target)?;

    Ok(())
}

/// Fallback: single connection download for servers without Range support
async fn download_single_compressed(
    url: &str,
    target: &Path,
    expected_sha256: Option<&str>,
) -> io::Result<()> {
    let client = Client::new();

    let response = client.get(url).send().await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Request failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("HTTP {} from {}", response.status(), url),
        ));
    }

    let total_size = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    tracing::info!("  Downloading {} MB (single connection)...", total_size / (1024 * 1024));

    // Download to temp file
    let temp_compressed = target.with_extension("bin.zst.tmp");
    let mut file = File::create(&temp_compressed)?;

    let bytes = response.bytes().await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Download failed: {}", e)))?;

    file.write_all(&bytes)?;
    file.flush()?;
    drop(file);

    // Decompress
    tracing::info!("  Decompressing...");

    let compressed_file = File::open(&temp_compressed)?;
    let decoder = zstd::stream::read::Decoder::new(BufReader::new(compressed_file))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Zstd init failed: {}", e)))?;

    let temp_target = target.with_extension("bin.tmp");
    let output_file = File::create(&temp_target)?;
    let mut buffered_writer = BufWriter::with_capacity(BUFFER_SIZE, output_file);

    let mut hasher = Sha256::new();
    let mut decoder = decoder;
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let bytes_read = decoder.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
        buffered_writer.write_all(&buffer[..bytes_read])?;
    }

    buffered_writer.flush()?;
    drop(buffered_writer);

    let actual_sha256 = format!("{:x}", hasher.finalize());

    // Verify checksum
    if let Some(expected) = expected_sha256 {
        if actual_sha256.to_lowercase() != expected.to_lowercase() {
            fs::remove_file(&temp_target)?;
            fs::remove_file(&temp_compressed)?;
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Checksum mismatch: expected {}, got {}", expected, actual_sha256),
            ));
        }
    }

    // Cleanup and finalize
    fs::remove_file(&temp_compressed)?;
    fs::rename(&temp_target, target)?;

    Ok(())
}

/// Reader that chains multiple files together
struct ChainedReader {
    readers: Vec<File>,
    current_idx: usize,
}

impl ChainedReader {
    fn new(readers: Vec<File>) -> Self {
        Self {
            readers,
            current_idx: 0,
        }
    }
}

impl Read for ChainedReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if self.current_idx >= self.readers.len() {
                return Ok(0);
            }

            let bytes_read = self.readers[self.current_idx].read(buf)?;
            if bytes_read > 0 {
                return Ok(bytes_read);
            }

            self.current_idx += 1;
        }
    }
}

/// Check if compressed version exists on server
pub async fn check_compressed_available(base_url: &str, filename: &str) -> bool {
    let compressed_url = format!("{}/{}.zst", base_url, filename);

    let client = match Client::builder().build() {
        Ok(c) => c,
        Err(_) => return false,
    };

    match client.head(&compressed_url).send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

/// Fetch the decompressed file's SHA256 from server
pub async fn fetch_decompressed_sha256(base_url: &str, filename: &str) -> Option<String> {
    let sha256_url = format!("{}/{}.sha256", base_url, filename);

    let client = Client::new();
    let response = client.get(&sha256_url).send().await.ok()?;

    if !response.status().is_success() {
        return None;
    }

    let text = response.text().await.ok()?;
    text.split_whitespace().next().map(|s| s.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_new() {
        let progress = DownloadProgress::new(
            "https://example.com/file.zst",
            100 * 1024 * 1024, // 100 MB
            Some("abc123".to_string()),
        );

        assert_eq!(progress.version, 1);
        assert_eq!(progress.compressed_size, 100 * 1024 * 1024);
        assert_eq!(progress.total_chunks, 5); // 100 MB / 20 MB = 5 chunks
        assert_eq!(progress.pending_chunks().len(), 5);
    }

    #[test]
    fn test_progress_mark_complete() {
        let mut progress = DownloadProgress::new(
            "https://example.com/file.zst",
            50 * 1024 * 1024, // 50 MB
            None,
        );

        assert_eq!(progress.pending_chunks().len(), 3);

        progress.mark_complete(0, "sha256_0".to_string());
        progress.mark_complete(1, "sha256_1".to_string());

        assert_eq!(progress.pending_chunks().len(), 1);
        assert!(!progress.all_complete());

        progress.mark_complete(2, "sha256_2".to_string());

        assert!(progress.all_complete());
    }
}
