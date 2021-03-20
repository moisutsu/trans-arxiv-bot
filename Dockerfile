FROM ekidd/rust-musl-builder:stable AS builder
RUN cargo install --target x86_64-unknown-linux-musl trans-arxiv-bot

FROM scratch
COPY --from=builder /home/rust/.cargo/bin/trans-arxiv-bot .
CMD ["./trans-arxiv-bot"]
