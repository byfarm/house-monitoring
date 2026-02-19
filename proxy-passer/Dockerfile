# ---------- Stage 1: Builder ----------
FROM rust:1.93-slim-bookworm AS builder
WORKDIR /app

# 1️⃣ Copy dependency files only
COPY Cargo.toml Cargo.lock ./

# 2️⃣ Create dummy main to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 3️⃣ Copy actual source code
COPY src ./src

# 4️⃣ Build real binary
RUN cargo build --release

# ---------- Stage 2: Runtime ----------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/proxy-passer /app/proxy-passer

EXPOSE 3000
CMD ["./proxy-passer"]
