#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "Testing RustyGate proxy server..."

# Test health endpoint
echo -e "\n${GREEN}Testing health endpoint:${NC}"
curl -s http://localhost:8080/health

# Test OpenAI proxy endpoint
echo -e "\n${GREEN}Testing OpenAI proxy endpoint:${NC}"
curl -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_openai_api_key" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [{"role": "user", "content": "Say hello!"}]
  }' \
  http://localhost:8080/v1/chat/completions 