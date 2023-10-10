FROM rust:1.73

WORKDIR /usr/src/app

RUN cargo install cargo-watch

# ローカルのファイルをコンテナにコピー
COPY memo/ .

RUN cargo build --release

CMD ["./target/release/memo"]