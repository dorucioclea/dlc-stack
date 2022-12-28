FROM rust:1.62 AS build

COPY Cargo.toml Cargo.lock /app/prototype-wallet-backend/
WORKDIR /app/prototype-wallet-backend/

RUN echo "fn main() {}" >> /app/prototype-wallet-backend/dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY static /app/prototype-wallet-backend/static
COPY index.html /app/prototype-wallet-backend/index.html
COPY ./src /app/prototype-wallet-backend/src
RUN cargo build --release

FROM debian:buster-slim AS app
RUN apt-get update && apt-get install -y openssl && apt-get clean
COPY --from=build /app/prototype-wallet-backend/static /app/static
COPY --from=build /app/prototype-wallet-backend/index.html /app/index.html
COPY --from=build /app/prototype-wallet-backend/target/release/dlc-link-backend /app/dlc-link-backend
WORKDIR /app
ENTRYPOINT ["/app/dlc-link-backend"]