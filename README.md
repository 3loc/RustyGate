# Rusty<img src="./gate.png" width="13%" alt="RustyGate Logo">Gate

[![Docker Build](https://github.com/3loc/rustygate/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/3loc/rustygate/actions/workflows/docker-publish.yml) [![License](https://img.shields.io/github/license/3loc/rustygate.svg)](https://github.com/3loc/rustygate/blob/main/LICENSE) ![Linux](https://img.shields.io/badge/Linux-amd64%20%7C%20arm64-blue) ![macOS](https://img.shields.io/badge/macOS-amd64%20%7C%20arm64-blue) ![FreeBSD](https://img.shields.io/badge/FreeBSD-amd64-blue) [![Rust Version](https://img.shields.io/badge/rust-1.76%2B-orange.svg)](https://www.rust-lang.org) ![OpenSSL](https://img.shields.io/badge/OpenSSL-static-green) [![Release](https://img.shields.io/github/v/release/3loc/rustygate)](https://github.com/3loc/rustygate/releases/latest) [![GitHub issues](https://img.shields.io/github/issues/3loc/rustygate)](https://github.com/3loc/rustygate/issues) [![Crates.io](https://img.shields.io/badge/async-runtime-yellow)](https://crates.io/crates/tokio)

**RustyGate** is a lightweight, high performance, asynchronous OpenAI API proxy server with rate limiting, written in Rust.

## Features
- **Request Forwarding**: Asynchronously forwards requests to OpenAI's API.
- **Streaming**: Handles Server-Sent Events (SSE) streaming responses from OpenAI.
- **Rate Limiting**: Configurable rate limiting using leaky bucket algorithm.
- **Multi-architecture Support**: Docker images available for AMD64 and ARM64 architectures.

## Requirements
- **Docker and Docker Compose** (recommended) or **Rust**
- **OpenAI API Key**: You need an OpenAI API key to authenticate requests.

## Quick Start with Docker Compose

The easiest way to run RustyGate and verify it's working is using Docker Compose:

1. **Set your OpenAI API key**:
    ```bash
    export OPENAI_API_KEY="your-api-key"
    ```

2. **Start RustyGate**:
    ```bash
    docker compose up rustygate
    ```

3. **Run the test suite** (optional):
    ```bash
    # In a new terminal
    docker compose up tests
    ```

The test suite will verify:
- Basic request forwarding
- Streaming responses
- JSON streaming
- Rate limiting behavior
- Different model support (gpt-4, o1-mini)

### Configuration

You can customize the configuration by setting environment variables before running docker compose:
```bash
# Example configuration
export PORT=9000                  # Default: 8080
export RATE_LIMIT=20              # Default: 1
export RATE_LIMIT_BURST=40        # Default: 3
export SSE_KEEPALIVE_INTERVAL=30  # Default: 15
export RUST_LOG=info             # Default: debug

docker compose up rustygate
```

## From source

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/3loc/rustygate.git
    cd rustygate
    ```

2. **Build the Project**:
    ```bash
    cargo build --release
    ```

## Configuration

The following environment variables are supported:

- `OPENAI_API_KEY` (required): Your OpenAI API key
- `PORT`: Server port (default: 8080)
- `BIND_ADDRESS`: Server bind address (default: 127.0.0.1)
- `RUST_LOG`: Log level (default: debug)
- `SSE_CHANNEL_CAPACITY`: Capacity for streaming message channel (default: 100)
- `SSE_KEEPALIVE_INTERVAL`: Keepalive interval in seconds for SSE (default: 15)
- `SSE_BUFFER_CAPACITY`: Initial capacity for SSE response buffer (default: 1024)
- `RATE_LIMIT`: Maximum requests per second (default: 10)
- `RATE_LIMIT_BURST`: Maximum burst capacity for rate limiting (default: 20)

## Usage

### Running the Server

1. **Set Environment Variables**:
    ```bash
    export OPENAI_API_KEY="your-api-key"
    export RUST_LOG=debug
    ```

2. **Start the Server**:
    ```bash
    cargo run
    ```

### Making Requests

The proxy forwards requests to OpenAI's API while maintaining the same API structure. Simply replace the OpenAI API base URL with your RustyGate server URL:

```bash
# Non-streaming request
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'

# Streaming request
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Hello!"}],
    "stream": true
  }'
```

## Libraries Used

- **axum**: Modern web framework for building HTTP servers
- **tokio**: Asynchronous runtime
- **tower-http**: HTTP-specific middleware
- **reqwest**: HTTP client for making requests
- **tracing**: Application-level logging
- **color-eyre**: Error handling and reporting
- **leaky-bucket**: Rate limiting implementation with fair queuing

## Development

To run in development mode with debug logging:

```bash
make dev
```

## License

This project is licensed under the MIT License.

## Installation

### Using Docker (recommended)
```bash
# Pull from GitHub Container Registry
docker pull ghcr.io/3loc/rustygate:latest

# Or specify a version
docker pull ghcr.io/3loc/rustygate:0.1.0
```

### Using Pre-built Binaries

Download the latest binary for your platform from the [releases page](https://github.com/3loc/rustygate/releases).

```bash
# Linux AMD64
curl -LO https://github.com/3loc/rustygate/releases/latest/download/rustygate-linux-amd64
chmod +x rustygate-linux-amd64
sudo mv rustygate-linux-amd64 /usr/local/bin/rustygate

# Linux ARM64 (e.g., Raspberry Pi)
curl -LO https://github.com/3loc/rustygate/releases/latest/download/rustygate-linux-arm64
chmod +x rustygate-linux-arm64
sudo mv rustygate-linux-arm64 /usr/local/bin/rustygate

# FreeBSD AMD64
curl -LO https://github.com/3loc/rustygate/releases/latest/download/rustygate-freebsd-amd64
chmod +x rustygate-freebsd-amd64
sudo mv rustygate-freebsd-amd64 /usr/local/bin/rustygate

# macOS AMD64 (Intel)
curl -LO https://github.com/3loc/rustygate/releases/latest/download/rustygate-darwin-amd64
chmod +x rustygate-darwin-amd64
sudo mv rustygate-darwin-amd64 /usr/local/bin/rustygate

# macOS ARM64 (Apple Silicon)
curl -LO https://github.com/3loc/rustygate/releases/latest/download/rustygate-darwin-arm64
chmod +x rustygate-darwin-arm64
sudo mv rustygate-darwin-arm64 /usr/local/bin/rustygate
```

To verify the binary (recommended):
```bash
# Download the checksum file
curl -LO https://github.com/3loc/rustygate/releases/latest/download/rustygate-<platform>.sha256

# Verify the checksum (replace <platform> with your platform)
sha256sum -c rustygate-<platform>.sha256
```

Available platforms:
- `linux-amd64`: Linux on Intel/AMD 64-bit
- `linux-arm64`: Linux on ARM64 (e.g., Raspberry Pi)
- `freebsd-amd64`: FreeBSD on Intel/AMD 64-bit
- `darwin-amd64`: macOS on Intel
- `darwin-arm64`: macOS on Apple Silicon
