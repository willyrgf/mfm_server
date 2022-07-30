FROM rust:1-buster

WORKDIR /app
COPY . /app

RUN cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres,rustls
RUN cargo build --release