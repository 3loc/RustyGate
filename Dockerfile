FROM --platform=$BUILDPLATFORM rust:1.76-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    gcc \
    make

WORKDIR /usr/src/rustygate
COPY . .

# Build with static OpenSSL
RUN OPENSSL_STATIC=1 cargo build --release && \
    ls -la target/release/

# The binary is named "main" because it's in src/bin/main.rs
RUN mv target/release/main /usr/local/bin/rustygate

FROM --platform=$TARGETPLATFORM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache ca-certificates curl

COPY --from=builder /usr/local/bin/rustygate /usr/local/bin/

EXPOSE 8080

CMD ["rustygate"] 