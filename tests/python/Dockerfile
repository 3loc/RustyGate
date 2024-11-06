# Builder stage
FROM --platform=$BUILDPLATFORM python:3.11-alpine AS builder

WORKDIR /build

# Install build dependencies
RUN apk add --no-cache gcc musl-dev python3-dev

COPY tests/python/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt --target /install

# Final stage
FROM --platform=$TARGETPLATFORM python:3.11-alpine

WORKDIR /tests

# Copy only the installed packages and test files
COPY --from=builder /install /usr/local/lib/python3.11/site-packages/
COPY tests/python .

CMD ["python", "-m", "pytest", "test_rustygate.py", "-v", "--asyncio-mode=auto"] 