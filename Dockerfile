# syntax = docker/dockerfile:experimental
FROM clux/muslrust:stable as builder

COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    cargo install --path .

FROM scratch

COPY --from=builder /root/.cargo/bin/forwarder /app/

EXPOSE 8080
STOPSIGNAL SIGINT

ENTRYPOINT ["/app/forwarder"]
