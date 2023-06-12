FROM rust:alpine3.17

RUN sudo apt-get install libssl-dev
WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/utils .env.dev .env.production ./

ENTRYPOINT ["/app/utils"]