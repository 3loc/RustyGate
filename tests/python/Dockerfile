FROM python:3.11-alpine

WORKDIR /tests

# Install build dependencies for pytest and aiohttp
RUN apk add --no-cache gcc musl-dev python3-dev

COPY tests/python/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY tests/python .

# Use pytest-asyncio to run async tests
CMD ["python", "-m", "pytest", "test_rustygate.py", "-v", "--asyncio-mode=auto"] 