#!/bin/bash
set -e

echo "====================================="
echo "grammar-rs startup"
echo "====================================="

# Check if N-gram data needs to be downloaded
NGRAMS_DIR="/app/data/ngrams"
EN_NGRAMS="$NGRAMS_DIR/en_ngrams.bin"
FR_NGRAMS="$NGRAMS_DIR/fr_ngrams.bin"

if [ ! -f "$EN_NGRAMS" ] && [ "$GRAMMAR_RS_AUTO_DOWNLOAD" = "1" ]; then
    echo ""
    echo "====================================="
    echo "First startup detected"
    echo "N-gram data will be downloaded..."
    echo "This is a one-time operation"
    echo "====================================="
    echo ""
    echo "Expected download sizes:"
    echo "  - EN: ~23GB (~10-20 minutes)"
    echo "  - FR: ~2GB (~1-2 minutes)"
    echo ""
    echo "Download will start when the API server initializes."
    echo "You can monitor progress in the logs."
    echo ""
fi

# Start the API server (it will auto-download N-grams if needed)
exec /app/grammar-api
