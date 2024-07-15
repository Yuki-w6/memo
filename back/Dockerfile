FROM rust as develop-stage

# WORKDIR /app
WORKDIR /usr/src/back
RUN cargo install cargo-watch

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release
