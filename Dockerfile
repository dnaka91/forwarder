# syntax = docker/dockerfile:experimental
FROM clux/muslrust:stable as builder

WORKDIR /volume

COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    cargo install --path .

RUN strip --strip-all /root/.cargo/bin/forwarder

FROM scratch

COPY --from=builder /root/.cargo/bin/forwarder /bin/

EXPOSE 8080
STOPSIGNAL SIGINT

ENTRYPOINT ["/bin/forwarder"]
