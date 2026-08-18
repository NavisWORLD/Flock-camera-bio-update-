FROM rust:1.82-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build -p flock-signal-gateway --release

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 --create-home flocksignal
COPY --from=builder /app/target/release/flock-signal-gateway /usr/local/bin/flock-signal-gateway
USER flocksignal
ENV FLOCK_SIGNAL_BIND=0.0.0.0:8080
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/flock-signal-gateway"]
