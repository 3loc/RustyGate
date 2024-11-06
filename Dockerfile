FROM --platform=$BUILDPLATFORM rust:1.76-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    gcc \
    make \
    perl

WORKDIR /usr/src/rustygate
COPY . .

# Build with vendored OpenSSL (no need for openssl-dev as we're using vendored)
RUN cargo build --release && \
    ls -la target/release/

# The binary is named "main" because it's in src/bin/main.rs
RUN mv target/release/main /usr/local/bin/rustygate

FROM --platform=$TARGETPLATFORM alpine:3.19

# Install ca-certificates and curl in the final image
RUN apk add --no-cache \
    ca-certificates \
    curl

COPY --from=builder /usr/local/bin/rustygate /usr/local/bin/

EXPOSE 8080

CMD ["rustygate"] 