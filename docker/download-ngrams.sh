#!/bin/bash
set -e

NGRAMS_DIR="/ngrams"
FR_NGRAMS_URL="https://languagetool.org/download/ngram-data/ngrams-fr-20150913.zip"
EN_NGRAMS_URL="https://languagetool.org/download/ngram-data/ngrams-en-20150817.zip"

# French n-grams
if [ ! -d "$NGRAMS_DIR/fr" ]; then
  echo "Downloading French n-grams (~8GB)..."
  mkdir -p "$NGRAMS_DIR"
  cd "$NGRAMS_DIR"
  curl -L -o ngrams-fr.zip "$FR_NGRAMS_URL"
  echo "Extracting French n-grams..."
  unzip -q ngrams-fr.zip && rm ngrams-fr.zip
  echo "French n-grams installed!"
else
  echo "French n-grams already installed."
fi

# English n-grams
if [ ! -d "$NGRAMS_DIR/en" ]; then
  echo "Downloading English n-grams (~8GB)..."
  cd "$NGRAMS_DIR"
  curl -L -o ngrams-en.zip "$EN_NGRAMS_URL"
  echo "Extracting English n-grams..."
  unzip -q ngrams-en.zip && rm ngrams-en.zip
  echo "English n-grams installed!"
else
  echo "English n-grams already installed."
fi

# Start LanguageTool (call the original start script)
cd /LanguageTool
exec bash start.sh "$@"
