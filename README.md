# mfm_server
A server to enhance [MFM features](https://github.com/willyrgf/mfm).

## ENVs
- `BIND_ADDRESS`: the host address to bind default is `127.0.0.1:8000`
- `DATABASE_URL`: the database url of the server with the format `postgres://postgres:example@database:5432/mfmserver_development`

## Optional
### Install bunyan
_Bunyan is a simple and fast JSON logging library for node.js services, a one-JSON-object-per-line
log format, and a bunyan CLI tool for nicely viewing those logs. this is a Rust implementation of
bunyan cli used to filter and pretty-print Bunyan log file content_

```
cargo install bunyan
```

## Prerequirement
### Install sqlx-cli
```sh
cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres,rustls
```

## Run migration
```sh
DATABASE_URL="postgres://postgres:example@127.0.0.1:5445/mfmserver_development" sqlx migrate run
```

## To run
```sh
DATABASE_URL="postgres://postgres:example@127.0.0.1:5445/mfmserver_development" RUST_LOG=debug cargo watch -x 'run' | bunyan
```

## To create a auth_token
```sql
-- connected in your `mfmserver_development` database
insert into auth_tokens (token_label)
			values ("my_toekn")
			returning token;
```