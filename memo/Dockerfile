FROM rust:1.73 as develop-stage

WORKDIR /app
RUN cargo install cargo-watch

COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release
