# --- BUILD --- #
FROM rust:1.84.0-slim-bookworm as builder

WORKDIR /usr/microservice

RUN rustup target add x86_64-unknown-linux-musl

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

RUN cargo build --target x86_64-unknown-linux-musl --release

# --- RUN --- #
FROM scratch

COPY --from=builder /usr/microservice/target/x86_64-unknown-linux-musl/release/distance-measuring-service /usr/local/bin/microservice

ENV SERVICE_TITLE microservice
ENV SERVICE_VERSION 0.0.1
ENV SERVICE_HOST localhost
ENV SERVICE_PORT 6969
ENV SERVICE_WORKERS 1
ENV LOG_LEVEL info

CMD ["microservice"]
