# === BUILD STAGE ===
FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy source code and build
COPY Cargo.toml Cargo.lock ./
COPY benches ./benches
COPY src ./src
COPY data ./data
# NOTE: data/ngrams/ is excluded via .dockerignore (downloaded at runtime via GRAMMAR_RS_AUTO_DOWNLOAD)

# Build with ngram-download feature for auto-downloading N-gram data
RUN cargo build --release --bin grammar-api --features ngram-download && \
    strip /build/target/release/grammar-api

# === RUNTIME STAGE ===
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1001 grammaruser

WORKDIR /app

COPY --from=builder /build/target/release/grammar-api /app/grammar-api
COPY --from=builder /build/data/dictionaries /app/data/dictionaries
COPY entrypoint.sh /app/entrypoint.sh

# Create N-grams directory (will be mounted from volume on Fly.io)
RUN mkdir -p /app/data/ngrams && \
    chmod +x /app/entrypoint.sh && \
    chown -R grammaruser:grammaruser /app

USER grammaruser

ENV PORT=8081
ENV RUST_LOG=info
ENV GRAMMAR_RS_AUTO_DOWNLOAD=1

EXPOSE 8081

HEALTHCHECK --interval=30s --timeout=5s --start-period=30s \
  CMD curl -f http://localhost:8081/v2/languages || exit 1

CMD ["/app/entrypoint.sh"]
