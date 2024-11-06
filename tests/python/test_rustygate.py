from openai import OpenAI
import time
import sys
import asyncio
import aiohttp
import pytest
import os

# Get the RustyGate endpoint from environment variable, default to localhost if not set
RUSTYGATE_ENDPOINT = os.getenv('RUSTYGATE_ENDPOINT', 'http://localhost:8080')

def test_translation_through_proxy():
    # Initialize OpenAI client with proxy URL - no API key needed
    client = OpenAI(
        base_url=f"{RUSTYGATE_ENDPOINT}/v1",  # Use environment variable
        api_key="not-needed"  # OpenAI client requires some value, but it won't be used
    )
    
    # Swedish text: "Hej världen, hur mår du idag?"
    # (meaning: "Hello world, how are you today?")
    response = client.chat.completions.create(
        model="gpt-4o",
        messages=[
            {
                "role": "system",
                "content": "You are a professional translator with expertise in Swedish, Chinese, and English. Always provide translations with both Chinese characters and pinyin."
            },
            {
                "role": "user",
                "content": "Translate this Swedish text to Chinese (include both simplified Chinese characters and pinyin): 'Hej världen, hur mår du idag?'"
            }
        ],
        temperature=0.7,
        max_tokens=1000
    )
    
    # Print the response
    print("\nSwedish: Hej världen, hur mår du idag?")
    print(f"Chinese: {response.choices[0].message.content}")
    
    # Basic assertion to verify we got a response
    assert response.choices[0].message.content is not None
    assert len(response.choices[0].message.content) > 0

def test_streaming_through_proxy():
    # Initialize OpenAI client with proxy URL
    client = OpenAI(
        base_url=f"{RUSTYGATE_ENDPOINT}/v1",
        api_key="not-needed"
    )
    
    print("\nTesting streaming response:")
    print("Question: Explain quantum computing in simple terms, step by step")
    print("Response:\n")
    
    # Create a streaming chat completion
    stream = client.chat.completions.create(
        model="gpt-4o",
        messages=[
            {
                "role": "system",
                "content": "You are an expert in quantum physics with a talent for explaining complex concepts in simple terms. Provide clear, step-by-step explanations that a general audience can understand."
            },
            {
                "role": "user",
                "content": "Explain quantum computing in simple terms, step by step"
            }
        ],
        temperature=0.7,
        max_tokens=100,
        stream=True  # Enable streaming
    )
    
    # Process the stream
    full_response = ""
    
    for chunk in stream:
        if chunk.choices[0].delta.content is not None:
            content = chunk.choices[0].delta.content
            print(content, end='', flush=True)
            full_response += content
    
    print("\n")  # Add a newline at the end
    
    # Basic assertions
    assert len(full_response) > 0
    assert "quantum" in full_response.lower()

def test_streaming_json_through_proxy():
    client = OpenAI(
        base_url=f"{RUSTYGATE_ENDPOINT}/v1",
        api_key="not-needed"
    )
    
    print("\nTesting streaming JSON response:")
    print("Question: Write a JSON configuration for a web server with nested settings")
    print("Response:\n")
    
    stream = client.chat.completions.create(
        model="gpt-4o",
        messages=[
            {
                "role": "system",
                "content": "You are a senior DevOps engineer who specializes in writing clear, well-documented configuration files. Always include helpful comments explaining each section."
            },
            {
                "role": "user",
                "content": "Write a JSON configuration for a web server with nested settings. Include comments in your explanation."
            }
        ],
        temperature=0.7,
        max_tokens=1000,
        stream=True
    )
    
    full_response = ""
    
    for chunk in stream:
        if chunk.choices[0].delta.content is not None:
            content = chunk.choices[0].delta.content
            print(content, end='', flush=True)
            full_response += content
    
    print("\n")
    
    # Basic assertions
    assert len(full_response) > 0
    assert "{" in full_response  # Should contain JSON
    assert "}" in full_response

def test_reasoning_with_o1mini():
    # Initialize OpenAI client with proxy URL
    client = OpenAI(
        base_url=f"{RUSTYGATE_ENDPOINT}/v1",
        api_key="not-needed"
    )
    
    print("\nTesting o1-mini reasoning capabilities:")
    print("Question: Solve this logical puzzle: If all cats are animals, and Whiskers is a cat, what can we conclude about Whiskers?")
    
    # For o1-mini, we'll combine system and user prompts since it doesn't support system messages
    combined_prompt = """Context: You are a logic tutor who helps students understand logical reasoning through clear explanations.

Question: Solve this logical puzzle: If all cats are animals, and Whiskers is a cat, what can we conclude about Whiskers?"""
    
    # Create a completion with o1-mini
    response = client.chat.completions.create(
        model="o1-mini",
        messages=[
            {
                "role": "user",
                "content": combined_prompt
            }
        ],
        max_completion_tokens=1000,
    )
    
    print(f"Response: {response.choices[0].message.content}\n")
    
    # Basic assertions to verify the response
    assert response.choices[0].message.content is not None
    assert len(response.choices[0].message.content) > 0
    # The response should mention either "animal" or "Whiskers" as it's a logical conclusion
    assert any(word in response.choices[0].message.content.lower() 
              for word in ["animal", "whiskers"])

async def test_rate_limiting():
    print("\nTesting rate limiting with queuing:")
    
    rate_limit = int(os.getenv('RATE_LIMIT', '1'))
    rate_limit_burst = int(os.getenv('RATE_LIMIT_BURST', '3'))  # Match docker-compose.yml
    total_requests = 10
    remaining_requests = total_requests - rate_limit_burst
    
    async def make_request(session, i):
        try:
            start_time = time.time()
            async with session.post(
                f"{RUSTYGATE_ENDPOINT}/v1/chat/completions",
                json={
                    "model": "gpt-4",
                    "messages": [{"role": "user", "content": f"Hello {i}"}]
                },
                timeout=aiohttp.ClientTimeout(total=35)
            ) as response:
                elapsed = time.time() - start_time
                print(f"Request {i}: Completed after {elapsed:.2f}s")
                return elapsed
        except Exception as e:
            print(f"Request {i} failed: {str(e)}")
            return None

    print(f"Sending {total_requests} requests simultaneously")
    start_time = time.time()
    
    async with aiohttp.ClientSession() as session:
        completion_times = await asyncio.gather(*[
            make_request(session, i) for i in range(total_requests)
        ])
        
        total_time = time.time() - start_time
        
        # Calculate minimum time needed:
        # - First rate_limit_burst (3) requests complete immediately
        # - For remaining 7 requests, we need 4 seconds total
        # because:
        #   - Request 4 starts immediately after burst
        #   - Request 5 starts at 1s
        #   - Request 6 starts at 2s
        #   - Request 7 starts at 3s
        #   - Request 8 starts at 4s
        #   - Request 9 starts at 5s
        #   - Request 10 starts at 6s
        # So we need to wait until Request 8 completes at 4s
        min_expected_time = 4.0  # Fixed time based on our analysis
        
        print(f"\nRate Limiting Analysis:")
        print(f"Burst limit: {rate_limit_burst} requests")
        print(f"Rate limit: {rate_limit} req/s")
        print(f"Total requests: {total_requests}")
        print(f"Requests after burst: {remaining_requests}")
        print(f"Minimum expected time: {min_expected_time:.1f}s")
        print(f"Actual total time: {total_time:.2f}s")
        
        # Verify all requests completed
        assert all(t is not None for t in completion_times), \
            "All requests should complete successfully"
        
        # Verify minimum time requirement
        assert total_time >= min_expected_time, \
            f"Total time ({total_time:.2f}s) should be at least {min_expected_time:.1f}s to process {remaining_requests} rate-limited requests"
        
        # Analyze timing distribution
        completion_times = [t for t in completion_times if t]
        completion_times.sort()
        
        print(f"\nTiming Distribution:")
        print(f"Fastest request: {completion_times[0]:.2f}s")
        print(f"Slowest request: {completion_times[-1]:.2f}s")
        print(f"Requests completed in first second: {sum(1 for t in completion_times if t <= 1.0)}")
        print(f"Average time: {sum(completion_times) / len(completion_times):.2f}s")

if __name__ == "__main__":
    test_translation_through_proxy()
    test_streaming_through_proxy()
    test_streaming_json_through_proxy()
    test_reasoning_with_o1mini()
    asyncio.run(test_rate_limiting())
