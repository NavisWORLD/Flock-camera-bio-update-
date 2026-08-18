FROM rust:1.80-bookworm AS build
WORKDIR /src
COPY Cargo.toml ./Cargo.toml
COPY rust ./rust
COPY services ./services
RUN cargo build --release -p flock-signal-gateway

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 --create-home flocksignal
COPY --from=build /src/target/release/flock-signal-gateway /usr/local/bin/flock-signal-gateway
USER flocksignal
EXPOSE 8080
ENV FLOCK_SIGNAL_BIND=0.0.0.0:8080
ENTRYPOINT ["/usr/local/bin/flock-signal-gateway"]
