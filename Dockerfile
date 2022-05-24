FROM rust:1.61-alpine as builder

WORKDIR /volume

RUN apk add --no-cache musl-dev=~1.2

COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release

FROM alpine:3.16 as newuser

RUN echo "forwarder:x:1000:" > /tmp/group && \
    echo "forwarder:x:1000:1000::/dev/null:/sbin/nologin" > /tmp/passwd

FROM scratch

COPY --from=builder /volume/target/release/forwarder /bin/
COPY --from=newuser /tmp/group /tmp/passwd /etc/

EXPOSE 8080
STOPSIGNAL SIGINT
USER forwarder

ENTRYPOINT ["/bin/forwarder"]
