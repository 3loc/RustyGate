FROM --platform=$TARGETPLATFORM python:3.11-slim

WORKDIR /tests

# Install build dependencies and packages
RUN apt-get update && apt-get install -y \
    gcc \
    python3-dev \
    && pip install --no-cache-dir --upgrade pip

# Copy requirements first for better caching
COPY tests/python/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Cleanup build dependencies
RUN apt-get remove -y gcc python3-dev \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/*

# Copy test files
COPY tests/python .

CMD ["python", "-m", "pytest", "test_rustygate.py", "-v", "--asyncio-mode=auto"] 