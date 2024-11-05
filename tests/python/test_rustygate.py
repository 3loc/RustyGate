from openai import OpenAI
import time
import sys
import asyncio
import aiohttp
import pytest

def test_translation_through_proxy():
    # Initialize OpenAI client with proxy URL - no API key needed
    client = OpenAI(
        base_url="http://rustygate:8080/v1",  # Point to RustyGate proxy
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
        base_url="http://rustygate:8080/v1",
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
        base_url="http://rustygate:8080/v1",
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
        base_url="http://rustygate:8080/v1",
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
    
    start_time = time.time()
    
    async def make_request(session, i):
        request_start = time.time()
        try:
            async with session.post(
                "http://rustygate:8080/v1/chat/completions",
                json={
                    "model": "gpt-4",
                    "messages": [{"role": "user", "content": f"Hello {i}"}]
                },
                timeout=aiohttp.ClientTimeout(total=35)  # Match server timeout + 5s
            ) as response:
                body = await response.text()  # Ensure response is fully read
                status = response.status
                request_time = time.time() - request_start
                print(f"Request {i}: Status {status} (took {request_time:.2f}s)")
                return status, request_time
        except asyncio.TimeoutError:
            request_time = time.time() - request_start
            print(f"Request {i} timed out after {request_time:.2f}s")
            return 408, request_time  # 408 Request Timeout
        except Exception as e:
            request_time = time.time() - request_start
            print(f"Request {i} failed after {request_time:.2f}s: {str(e)}")
            if "Too Many Requests" in str(e):
                return 429, request_time
            return 500, request_time  # Internal Server Error

    async with aiohttp.ClientSession() as session:
        total_requests = 10
        print(f"Sending {total_requests} requests simultaneously")
        
        # Add overall timeout for all requests
        try:
            tasks = [make_request(session, i) for i in range(total_requests)]
            results = await asyncio.wait_for(
                asyncio.gather(*tasks),
                timeout=40  # Total test timeout
            )
        except asyncio.TimeoutError:
            print("Test timed out waiting for all requests to complete!")
            return
        
        statuses = [r[0] for r in results]
        times = [r[1] for r in results]
        
        success_count = sum(1 for status in statuses if status == 200)
        rate_limited_count = sum(1 for status in statuses if status == 429)
        timeout_count = sum(1 for status in statuses if status == 408)
        other_count = sum(1 for status in statuses if status not in [200, 429, 408])
        
        total_time = time.time() - start_time
        avg_time = sum(times) / len(times)
        max_time = max(times)
        
        print(f"\nResults Summary:")
        print(f"Successful requests: {success_count}")
        print(f"Rate limited requests: {rate_limited_count}")
        print(f"Timed out requests: {timeout_count}")
        print(f"Other status codes: {other_count}")
        print(f"Total time: {total_time:.2f}s")
        print(f"Average request time: {avg_time:.2f}s")
        print(f"Maximum request time: {max_time:.2f}s")
        
        # All requests should either succeed or timeout
        assert success_count + timeout_count == total_requests, \
            f"Expected all requests to either succeed or timeout. " \
            f"Got {success_count} successes, {timeout_count} timeouts, " \
            f"{rate_limited_count} rate limited, {other_count} other"
        assert max_time > 1.0, "Some requests should take longer due to rate limiting"

if __name__ == "__main__":
    test_translation_through_proxy()
    test_streaming_through_proxy()
    test_streaming_json_through_proxy()
    test_reasoning_with_o1mini()
    asyncio.run(test_rate_limiting())
