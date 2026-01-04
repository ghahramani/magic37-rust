FROM rust:latest as builder

WORKDIR /usr/src/app
COPY code/ .

# Build dependencies - this is the caching Docker layer!
RUN cargo build --release

FROM debian:stable-slim

COPY --from=builder /usr/src/app/target/release/auth-provider /bin/auth-provider

CMD ["auth-provider"]
