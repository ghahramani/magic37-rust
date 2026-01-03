FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# Build dependencies - this is the caching Docker layer!
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /usr/src/app/target/release/auth-provider /usr/local/bin/auth-provider

CMD ["auth-provider"]
