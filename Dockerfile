FROM rust

WORKDIR /code
COPY target/x86_64-unknown-linux-musl/release/rusty_auth /code
CMD ["./rusty_auth"]
