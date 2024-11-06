# Builder stage
FROM --platform=$BUILDPLATFORM python:3.11-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    python3-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements
COPY tests/python/requirements.txt .

# Install dependencies
RUN pip install --no-cache-dir -r requirements.txt --target /install

# Final stage
FROM --platform=$TARGETPLATFORM python:3.11-slim

WORKDIR /tests

# Copy only the installed packages and test files
COPY --from=builder /install /usr/local/lib/python3.11/site-packages/
COPY tests/python .

CMD ["python", "-m", "pytest", "test_rustygate.py", "-v", "--asyncio-mode=auto"] 