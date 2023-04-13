FROM rust:alpine

WORKDIR /code
COPY target/x86_64-unknown-linux-musl/debug/rusty_auth /code
CMD ["./rusty_auth"]
