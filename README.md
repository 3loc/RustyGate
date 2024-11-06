# <img src="./gate.png" width="13%" alt="RustyGate Logo">RustyGate

[![Docker Build](https://github.com/3loc/rustygate/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/3loc/rustygate/actions/workflows/docker-publish.yml) [![License](https://img.shields.io/github/license/3loc/rustygate.svg)](https://github.com/3loc/rustygate/blob/main/LICENSE) ![Linux](https://img.shields.io/badge/Linux-amd64%20%7C%20arm64-blue) ![macOS](https://img.shields.io/badge/macOS-amd64%20%7C%20arm64-blue) ![FreeBSD](https://img.shields.io/badge/FreeBSD-amd64-blue) [![Rust Version](https://img.shields.io/badge/rust-1.76%2B-orange.svg)](https://www.rust-lang.org) ![OpenSSL](https://img.shields.io/badge/OpenSSL-static-green) [![Release](https://img.shields.io/github/v/release/3loc/rustygate)](https://github.com/3loc/rustygate/releases/latest)

High-performance OpenAI API proxy with rate limiting and streaming support.

## Features
- Asynchronous request forwarding
- SSE streaming support
- Configurable rate limiting
- Multi-platform support (Linux, macOS, FreeBSD)
- Docker images for easy deployment

## Quick Start

### Using Docker
```bash
# Set your OpenAI API key
export OPENAI_API_KEY="your-api-key"

# Run RustyGate
docker run -p 8080:8080 -e OPENAI_API_KEY=$OPENAI_API_KEY ghcr.io/3loc/rustygate
```

### Using Pre-built Binaries
Download from [releases page](https://github.com/3loc/rustygate/releases) and install:
```bash
# Example for Linux AMD64
curl -LO https://github.com/3loc/rustygate/releases/latest/download/rustygate-linux-amd64
chmod +x rustygate-linux-amd64
sudo mv rustygate-linux-amd64 /usr/local/bin/rustygate
```

## Usage

```python
from openai import OpenAI

# Initialize client with RustyGate URL
client = OpenAI(
    base_url="http://localhost:8080/v1",  # RustyGate proxy URL
    api_key="not-needed"  # API key is handled by RustyGate
)

# Non-streaming request
response = client.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Hello!"}]
)
print(response.choices[0].message.content)

# Streaming request
stream = client.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Hello!"}],
    stream=True
)
for chunk in stream:
    if chunk.choices[0].delta.content is not None:
        print(chunk.choices[0].delta.content, end="")
```

## Configuration

Environment variables:
- `OPENAI_API_KEY` (required): Your OpenAI API key
- `PORT`: Server port (default: 8080)
- `RATE_LIMIT`: Requests per second (default: 10)
- `RATE_LIMIT_BURST`: Burst capacity (default: 20)
- `RUST_LOG`: Log level (default: debug)

## Development

```bash
# Clone and build
git clone https://github.com/3loc/rustygate.git
cd rustygate
cargo build --release

# Run tests
docker compose up tests
```

## License

MIT License
