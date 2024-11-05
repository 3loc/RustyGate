.PHONY: build run test check clean dev deps outdated update update-all fmt lint audit watch

# Default target
all: deps build

# Install dependencies
deps:
	@echo "Installing dependencies..."
	cargo update

# Build the project with memory optimization
build:
	@echo "Building RustyGate..."
	RUSTFLAGS="-C opt-level=2" cargo build

# Run the project
run:
	@echo "Running RustyGate..."
	RUST_LOG=debug PORT=8080 BIND_ADDRESS=127.0.0.1 cargo run

# Run in development mode
dev:
	@echo "Running in development mode..."
	RUST_LOG=debug PORT=8080 BIND_ADDRESS=127.0.0.1 cargo run

# Run tests
test:
	@echo "Running tests..."
	cargo test
	@echo "Running integration test..."
	./test.sh

# Check code without building
check:
	@echo "Checking code..."
	cargo check
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo audit

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf target/

# Install development dependencies
setup:
	@echo "Installing development dependencies..."
	rustup update
	cargo install cargo-audit
	cargo install cargo-watch
	@echo "Done! Run 'make dev' to start development server"

# Show help
help:
	@echo "Available commands:"
	@echo "  make deps     - Update dependencies"
	@echo "  make build    - Build the project"
	@echo "  make run      - Run the project"
	@echo "  make dev      - Run in development mode"
	@echo "  make test     - Run tests"
	@echo "  make check    - Check code"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make setup    - Install development dependencies"

# Check for outdated dependencies using cargo tree
outdated:
	@echo "Checking dependency tree..."
	cargo tree
	@echo "\nTo see latest versions, visit https://crates.io for each package"

# Update dependencies to latest compatible versions
update:
	@echo "Updating dependencies..."
	cargo update
	@echo "Updated to latest compatible versions"

# Update dependencies aggressively (may break semver)
update-all:
	@echo "Updating all dependencies to latest versions..."
	RUSTFLAGS="-C opt-level=1" cargo update --aggressive
	@echo "Updated all dependencies to latest versions"

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Lint code
lint:
	@echo "Linting code..."
	cargo clippy -- -D warnings

# Add a new target for security audit
audit:
	@echo "Auditing dependencies..."
	cargo audit

# Add watch mode for development
watch:
	@echo "Starting watch mode..."
	cargo watch -x run
