# mfm_server

## ENVs
- BIND_ADDRESS: the host address to bind default is `127.0.0.1:8000`
- DATABASE_URL: the database url of the server with the format `postgres://postgres:example@database:5432/mfmserver_development`

## Optional
### Install bunyan
_Bunyan is a simple and fast JSON logging library for node.js services, a one-JSON-object-per-line
log format, and a bunyan CLI tool for nicely viewing those logs. this is a Rust implementation of
bunyan cli used to filter and pretty-print Bunyan log file content_

```
cargo install bunyan
```

## To run
```sh
DATABASE_URL="postgres://postgres:example@127.0.0.1:5445/mfmserver_development" RUST_LOG=debug cargo watch -x 'run' | bunyan
```