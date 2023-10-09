FROM rust:1.73

WORKDIR /usr/src/app

# ローカルのファイルをコンテナにコピー
COPY memo/ .

RUN cargo build --release

CMD ["./target/release/memo"]