# Rusty<img src="./gate.png" width="13%" alt="RustyGate Logo">Gate

**RustyGate** is a lightweight, high performance, asynchronous OpenAI API proxy service with rate limiting, written in Rust.

## Features
- **Request Forwarding**: Asynchronously forwards requests to OpenAI's API.
- **Streaming**: Handles Server-Sent Events (SSE) streaming responses from OpenAI.
- **Rate Limiting**: Configurable rate limiting using leaky bucket algorithm.

## Requirements
- **Rust or Docker**: 
- **OpenAI API Key**: You need an OpenAI API key to authenticate requests.

## Docker Compose

The easiest way to run RustyGate is using Docker Compose:

1. **Set your OpenAI API key**:
    ```bash
    export OPENAI_API_KEY="your-api-key"
    ```

2. **Start the services**:
    ```bash
    docker compose up --build
    ```

This will:
- Build and start the RustyGate server
- Run the test suite against the server
- Configure the server using environment variables

You can customize the configuration by setting environment variables before running docker compose:
```bash
export PORT=9000
export RATE_LIMIT=20
export RATE_LIMIT_BURST=40
docker compose up --build
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
