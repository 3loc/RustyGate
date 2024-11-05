# RustyGate Python Integration Tests

This directory contains Python-based integration tests for RustyGate, using the OpenAI Python client to verify that the proxy correctly forwards requests to OpenAI's API.

## Setup

1. Create a virtual environment:
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

2. Install dependencies:
```bash
pip install -r requirements.txt
```

## Running Tests

1. Ensure RustyGate is running locally on port 8080 (with its own OPENAI_API_KEY configured)
2. Run the tests:
```bash
python test_rustygate.py
# or use pytest:
pytest test_rustygate.py -v
``` 