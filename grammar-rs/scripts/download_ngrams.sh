#!/bin/bash
# Download N-gram data from LanguageTool for grammar-rs
#
# Usage:
#   ./scripts/download_ngrams.sh           # Download both EN and FR
#   ./scripts/download_ngrams.sh en        # Download English only
#   ./scripts/download_ngrams.sh fr        # Download French only
#
# After downloading, extract and convert to compact format:
#   cargo run --bin sync-lt -- --extract-ngrams --language en
#   cargo run --bin sync-lt -- --extract-ngrams --language fr

set -e

# Configuration
NGRAM_DIR="data/ngrams"
EN_URL="https://languagetool.org/download/ngram-data/ngrams-en-20150817.zip"
FR_URL="https://languagetool.org/download/ngram-data/ngrams-fr-20150913.zip"

# Expected sizes (approximate)
EN_SIZE="8.96 GB"
FR_SIZE="1.82 GB"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create directory
mkdir -p "$NGRAM_DIR"

download_english() {
    local zip_file="$NGRAM_DIR/ngrams-en-20150817.zip"
    local extract_dir="$NGRAM_DIR/ngrams-en-20150817"

    if [ -d "$extract_dir" ]; then
        print_info "English N-grams already extracted at $extract_dir"
        return 0
    fi

    if [ -f "$zip_file" ]; then
        print_info "English N-grams zip already downloaded at $zip_file"
    else
        print_info "Downloading English N-grams ($EN_SIZE)..."
        print_warn "This is a large file and may take a while..."
        curl -L -o "$zip_file" "$EN_URL" --progress-bar
    fi

    print_info "Extracting English N-grams..."
    unzip -q "$zip_file" -d "$NGRAM_DIR"
    print_info "English N-grams extracted to $extract_dir"
}

download_french() {
    local zip_file="$NGRAM_DIR/ngrams-fr-20150913.zip"
    local extract_dir="$NGRAM_DIR/ngrams-fr-20150913"

    if [ -d "$extract_dir" ]; then
        print_info "French N-grams already extracted at $extract_dir"
        return 0
    fi

    if [ -f "$zip_file" ]; then
        print_info "French N-grams zip already downloaded at $zip_file"
    else
        print_info "Downloading French N-grams ($FR_SIZE)..."
        curl -L -o "$zip_file" "$FR_URL" --progress-bar
    fi

    print_info "Extracting French N-grams..."
    unzip -q "$zip_file" -d "$NGRAM_DIR"
    print_info "French N-grams extracted to $extract_dir"
}

# Parse arguments
LANG="${1:-all}"

case "$LANG" in
    en|english)
        download_english
        ;;
    fr|french)
        download_french
        ;;
    all)
        download_english
        download_french
        ;;
    *)
        print_error "Unknown language: $LANG"
        echo "Usage: $0 [en|fr|all]"
        exit 1
        ;;
esac

echo ""
print_info "Download complete!"
echo ""
echo "Next steps:"
echo "  1. Convert to compact format for grammar-rs:"
echo "     cargo run --bin sync-lt -- --extract-ngrams --language en"
echo "     cargo run --bin sync-lt -- --extract-ngrams --language fr"
echo ""
echo "  2. Use with grammar-api:"
echo "     cargo run --bin grammar-api -- --ngram-data $NGRAM_DIR/en_ngrams.bin"
