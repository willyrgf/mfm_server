FROM rust:1-buster

# RUN set -x \
#   && apk upgrade --no-cache \
#   && apk add --no-cache --virtual build-dependencies \
#     build-base libc-dev linux-headers openssl openssl-dev bash \
#     postgresql-dev postgresql-client openssl git less libpq \
#     musl-dev wget git gcc

WORKDIR /app

COPY . /app

RUN cargo install diesel_cli --no-default-features --features postgres --verbose
RUN cargo build --release