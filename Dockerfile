# Buillder
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.lock Cargo.toml ./
COPY src ./src

RUN cargo build --release

# Runner
FROM alpine:latest

WORKDIR /app

# todo я чет не так сделал, в рантайме файла нет
COPY --from=builder /app/target/release/golemich ./

ENTRYPOINT ["/app/golemich"]