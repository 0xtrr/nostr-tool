FROM rust:1.67 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/nostr-tool /usr/local/bin/nostr-tool
CMD ["nostr-tool"]
