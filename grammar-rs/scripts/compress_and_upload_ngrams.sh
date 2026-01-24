#!/bin/bash
# Compress N-gram files with zstd and upload to Cloudflare R2
#
# This script:
# 1. Compresses .bin files with zstd level 19 (best compression)
# 2. Generates SHA256 checksums for both compressed and uncompressed files
# 3. Uploads everything to R2
#
# Prerequisites:
#   1. zstd installed: brew install zstd (macOS) or apt install zstd (Linux)
#   2. rclone configured with R2 credentials
#
# Usage:
#   ./scripts/compress_and_upload_ngrams.sh en
#   ./scripts/compress_and_upload_ngrams.sh fr
#   ./scripts/compress_and_upload_ngrams.sh all

set -e

# Configuration
R2_BUCKET="${GRAMMAR_RS_R2_BUCKET:-autocorrect-quivr}"
R2_REMOTE="${GRAMMAR_RS_R2_REMOTE:-r2}"
ZSTD_LEVEL=19  # Maximum compression (slower compress, fast decompress)
ZSTD_THREADS=0  # Use all available CPU cores

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="${SCRIPT_DIR}/../data/ngrams"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

verify_dependencies() {
    if ! command -v zstd &> /dev/null; then
        log_error "zstd not installed"
        echo "Install: brew install zstd (macOS) or apt install zstd (Linux)"
        exit 1
    fi

    if ! command -v rclone &> /dev/null; then
        log_error "rclone not installed"
        echo "Install: https://rclone.org/install/"
        exit 1
    fi

    if ! rclone listremotes | grep -q "^${R2_REMOTE}:"; then
        log_error "rclone remote '${R2_REMOTE}' not configured"
        echo "Run 'rclone config' to set up Cloudflare R2"
        exit 1
    fi
}

format_size() {
    local size=$1
    if [ "$size" -ge 1073741824 ]; then
        echo "$(echo "scale=2; $size / 1073741824" | bc) GB"
    elif [ "$size" -ge 1048576 ]; then
        echo "$(echo "scale=2; $size / 1048576" | bc) MB"
    else
        echo "$size bytes"
    fi
}

compress_and_upload() {
    local lang=$1
    local filename="${lang}_ngrams.bin"
    local source="${DATA_DIR}/${filename}"
    local compressed="${DATA_DIR}/${filename}.zst"

    echo ""
    echo "=========================================="
    echo "Processing ${lang^^} N-grams"
    echo "=========================================="

    # Check source file exists
    if [ ! -f "$source" ]; then
        log_error "${source} not found"
        echo "Build it first with: cargo run --bin sync-lt -- --extract-ngrams --language ${lang}"
        exit 1
    fi

    local source_size=$(stat -f%z "$source" 2>/dev/null || stat -c%s "$source" 2>/dev/null)
    log_info "Source file: $(format_size $source_size)"

    # Step 1: Generate SHA256 for uncompressed file
    log_info "Generating SHA256 for uncompressed file..."
    local uncompressed_sha256=$(sha256sum "$source" | awk '{print $1}')
    echo "$uncompressed_sha256  ${filename}" > "${source}.sha256"
    log_success "SHA256 (uncompressed): ${uncompressed_sha256:0:16}..."

    # Step 2: Compress with zstd
    log_info "Compressing with zstd level ${ZSTD_LEVEL} (using all CPU cores)..."
    local start_time=$(date +%s)

    zstd -${ZSTD_LEVEL} -T${ZSTD_THREADS} --force "$source" -o "$compressed"

    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    local compressed_size=$(stat -f%z "$compressed" 2>/dev/null || stat -c%s "$compressed" 2>/dev/null)
    local ratio=$(echo "scale=2; $source_size / $compressed_size" | bc)

    log_success "Compressed in ${duration}s: $(format_size $source_size) -> $(format_size $compressed_size) (${ratio}x ratio)"

    # Step 3: Generate SHA256 for compressed file
    log_info "Generating SHA256 for compressed file..."
    local compressed_sha256=$(sha256sum "$compressed" | awk '{print $1}')
    echo "$compressed_sha256  ${filename}.zst" > "${compressed}.sha256"
    log_success "SHA256 (compressed): ${compressed_sha256:0:16}..."

    # Step 4: Upload all files to R2
    log_info "Uploading to R2..."

    # Upload compressed file
    log_info "  Uploading ${filename}.zst..."
    rclone copy \
        --progress \
        --s3-chunk-size 64M \
        --transfers 4 \
        "$compressed" \
        "${R2_REMOTE}:${R2_BUCKET}/ngrams/"

    # Upload compressed checksum
    rclone copy \
        "${compressed}.sha256" \
        "${R2_REMOTE}:${R2_BUCKET}/ngrams/"

    # Upload uncompressed checksum (for verification after decompression)
    rclone copy \
        "${source}.sha256" \
        "${R2_REMOTE}:${R2_BUCKET}/ngrams/"

    # Also keep the original uncompressed file for backwards compatibility
    log_info "  Uploading ${filename} (uncompressed, for backwards compatibility)..."
    rclone copy \
        --progress \
        --s3-chunk-size 64M \
        --transfers 4 \
        "$source" \
        "${R2_REMOTE}:${R2_BUCKET}/ngrams/"

    log_success "Upload complete for ${lang^^}!"
    echo ""
    echo "Files uploaded to ${R2_REMOTE}:${R2_BUCKET}/ngrams/:"
    echo "  - ${filename}         (uncompressed, $(format_size $source_size))"
    echo "  - ${filename}.zst     (compressed, $(format_size $compressed_size))"
    echo "  - ${filename}.sha256  (checksum for uncompressed)"
    echo "  - ${filename}.zst.sha256 (checksum for compressed)"
}

show_usage() {
    echo "Grammar-RS N-gram Compressor & Uploader"
    echo ""
    echo "This script compresses N-gram files with zstd for faster downloads,"
    echo "then uploads both compressed and uncompressed versions to R2."
    echo ""
    echo "Usage: $0 <language>"
    echo ""
    echo "Languages:"
    echo "  en   - Process English N-grams"
    echo "  fr   - Process French N-grams"
    echo "  all  - Process both"
    echo ""
    echo "Prerequisites:"
    echo "  1. zstd installed (brew install zstd)"
    echo "  2. rclone configured with R2 credentials"
    echo "  3. N-gram .bin files in data/ngrams/"
    echo ""
    echo "Environment variables:"
    echo "  GRAMMAR_RS_R2_BUCKET - R2 bucket name (default: autocorrect-quivr)"
    echo "  GRAMMAR_RS_R2_REMOTE - rclone remote name (default: r2)"
    echo ""
    echo "Expected speedup:"
    echo "  - EN: 23 GB -> ~6 GB (4x smaller, 4x faster download)"
    echo "  - FR: 6 GB -> ~1.5 GB (4x smaller, 4x faster download)"
    echo "  - With parallel download: additional 10-16x speedup"
}

# Main
case "${1:-}" in
    en|EN)
        verify_dependencies
        compress_and_upload "en"
        ;;
    fr|FR)
        verify_dependencies
        compress_and_upload "fr"
        ;;
    all|ALL)
        verify_dependencies
        compress_and_upload "en"
        compress_and_upload "fr"
        ;;
    *)
        show_usage
        exit 1
        ;;
esac

echo ""
echo "=========================================="
echo "All done!"
echo "=========================================="
echo ""
echo "Public URLs (after enabling public access):"
echo "  https://pub-8068a615549c43e1893eb3f9a35a0e17.r2.dev/ngrams/en_ngrams.bin.zst"
echo "  https://pub-8068a615549c43e1893eb3f9a35a0e17.r2.dev/ngrams/fr_ngrams.bin.zst"
echo ""
echo "To use fast parallel download in your app:"
echo "  GRAMMAR_RS_AUTO_DOWNLOAD=1 cargo run --features ngram-download-fast"
