FROM rust:1.75.0-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/ruke .

RUN apt-get update && apt install -y openssl

CMD ["./ruke"]
