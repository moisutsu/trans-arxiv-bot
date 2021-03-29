FROM ekidd/rust-musl-builder:stable AS builder
RUN cargo install --target x86_64-unknown-linux-musl --version 0.1.4 trans-arxiv-bot

FROM alpine:3.13
COPY --from=builder /home/rust/.cargo/bin/trans-arxiv-bot .
RUN apk add --no-cache ca-certificates && update-ca-certificates
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD ["./trans-arxiv-bot"]
