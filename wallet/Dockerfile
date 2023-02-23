FROM rust:1.65 AS build

COPY Cargo.toml Cargo.lock /app/dlc-protocol-wallet/
WORKDIR /app/dlc-protocol-wallet/

RUN cargo build --release --bin dlc-protocol-wallet

FROM debian:buster-slim AS dlc-link-stack
RUN apt-get update && apt-get install -y openssl libpq-dev && apt-get clean
COPY --from=build /app/dlc-stack/target/release/dlc-protocol-wallet /app/dlc-protocol-wallet
WORKDIR /app
ENTRYPOINT ["/app/dlc-protocol-wallet"]
