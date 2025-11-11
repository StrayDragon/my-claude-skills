# Async Testing in Python - Comprehensive Guide

This guide focuses specifically on testing asynchronous Python code with pytest and related tools.

## Table of Contents
- [Setup and Configuration](#setup-and-configuration)
- [Basic Async Testing](#basic-async-testing)
- [Async Fixtures](#async-fixtures)
- [Testing Async Context Managers](#testing-async-context-managers)
- [Mocking Async Code](#mocking-async-code)
- [Error Handling in Async Tests](#error-handling-in-async-tests)
- [Performance Testing Async Code](#performance-testing-async-code)
- [Integration Testing Async APIs](#integration-testing-async-apis)
- [Common Async Testing Patterns](#common-async-testing-patterns)
- [Advanced Async Testing Techniques](#advanced-async-testing-techniques)

---

## Setup and Configuration

### Required Packages

```bash
pip install pytest pytest-asyncio pytest-mock aiohttp httpx
```

### pytest-asyncio Configuration

```ini
# pytest.ini
[tool:pytest]
asyncio_mode = auto
testpaths = tests
markers =
    asyncio: marks tests as async
    slow: marks tests as slow (deselect with '-m "not slow"')
```

### Event Loop Policies (for Python 3.10+)

```python
# tests/conftest.py
import asyncio
import pytest

@pytest.fixture(scope="session")
def event_loop_policy():
    """Set up event loop policy for async tests."""
    if hasattr(asyncio, 'WindowsProactorEventLoopPolicy'):
        # Windows specific policy
        asyncio.set_event_loop_policy(asyncio.WindowsProactorEventLoopPolicy())
    else:
        # Unix systems
        asyncio.set_event_loop_policy(asyncio.DefaultEventLoopPolicy())
```

---

## Basic Async Testing

### Simple Async Function Tests

```python
# async_service.py
import asyncio
from typing import List

async def fetch_user_data(user_id: int) -> dict:
    """Simulate async user data fetch."""
    await asyncio.sleep(0.1)  # Simulate network delay
    return {"id": user_id, "name": f"User {user_id}"}

async def fetch_multiple_users(user_ids: List[int]) -> List[dict]:
    """Fetch multiple users concurrently."""
    tasks = [fetch_user_data(user_id) for user_id in user_ids]
    return await asyncio.gather(*tasks)

# test_async_service.py
import pytest
from unittest.mock import AsyncMock, patch

@pytest.mark.asyncio
async def test_fetch_user_data():
    """Test basic async function."""
    user_id = 123
    result = await fetch_user_data(user_id)

    assert result["id"] == user_id
    assert result["name"] == f"User {user_id}"

@pytest.mark.asyncio
async def test_fetch_multiple_users():
    """Test concurrent async operations."""
    user_ids = [1, 2, 3]
    results = await fetch_multiple_users(user_ids)

    assert len(results) == 3
    for i, result in enumerate(results):
        assert result["id"] == user_ids[i]
        assert result["name"] == f"User {user_ids[i]}"

@pytest.mark.asyncio
async def test_async_function_with_exception():
    """Test async function that raises exception."""

    async def failing_function():
        await asyncio.sleep(0.1)
        raise ValueError("Async error occurred")

    with pytest.raises(ValueError, match="Async error occurred"):
        await failing_function()
```

### Timeout Testing

```python
import pytest
from asyncio import TimeoutError

@pytest.mark.asyncio
async def test_async_operation_timeout():
    """Test timeout handling in async operations."""

    async def slow_operation():
        await asyncio.sleep(2.0)
        return "completed"

    with pytest.raises(asyncio.TimeoutError):
        await asyncio.wait_for(slow_operation(), timeout=1.0)

@pytest.mark.asyncio
async def test_async_operation_within_timeout():
    """Test operation completing within timeout."""

    async def fast_operation():
        await asyncio.sleep(0.1)
        return "completed"

    result = await asyncio.wait_for(fast_operation(), timeout=1.0)
    assert result == "completed"
```

---

## Async Fixtures

### Basic Async Fixtures

```python
# tests/conftest.py
import pytest
import asyncio
from typing import AsyncGenerator

@pytest.fixture
async def async_resource() -> AsyncGenerator[str, None]:
    """Async fixture that provides and cleans up a resource."""
    resource = "async_resource"

    # Setup
    print(f"Setting up {resource}")

    yield resource

    # Cleanup
    print(f"Cleaning up {resource}")

@pytest.fixture(scope="session")
async def shared_async_resource() -> AsyncGenerator[dict, None]:
    """Session-scoped async fixture."""
    shared_data = {"initialized": True}

    # Expensive setup that should run once per session
    await asyncio.sleep(0.1)  # Simulate setup time
    shared_data["setup_time"] = asyncio.get_event_loop().time()

    yield shared_data

    # Cleanup
    print("Cleaning up shared async resource")

# Using async fixtures in tests
@pytest.mark.asyncio
async def test_with_async_fixture(async_resource):
    """Test using async fixture."""
    assert async_resource == "async_resource"
    await asyncio.sleep(0.01)  # Simulate test work

@pytest.mark.asyncio
async def test_with_shared_fixture(shared_async_resource):
    """Test using session-scoped async fixture."""
    assert shared_async_resource["initialized"] is True
    assert "setup_time" in shared_async_resource
```

### Database Async Fixtures

```python
# tests/conftest.py
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession, async_sessionmaker
from sqlalchemy.orm import declarative_base

Base = declarative_base()

@pytest.fixture
async def async_db_session() -> AsyncGenerator[AsyncSession, None]:
    """Provide async database session for tests."""
    engine = create_async_engine("sqlite+aiosqlite:///:memory:")

    # Create tables
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)

    # Create session
    async_session = async_sessionmaker(
        engine, class_=AsyncSession, expire_on_commit=False
    )

    async with async_session() as session:
        yield session

    await engine.dispose()

# Using async database fixture
@pytest.mark.asyncio
async def test_async_database_operations(async_db_session):
    """Test async database operations."""
    from models import User  # Your async model

    user = User(name="Test User", email="test@example.com")
    async_db_session.add(user)
    await async_db_session.commit()

    # Retrieve user
    result = await async_db_session.get(User, user.id)
    assert result.name == "Test User"
```

### HTTP Client Async Fixtures

```python
# tests/conftest.py
import httpx
from fastapi.testclient import TestClient

@pytest.fixture
async def async_http_client() -> AsyncGenerator[httpx.AsyncClient, None]:
    """Provide async HTTP client for testing APIs."""
    async with httpx.AsyncClient(app=app, base_url="http://test") as client:
        yield client

# Usage in tests
@pytest.mark.asyncio
async def test_async_api_endpoint(async_http_client):
    """Test async API endpoint."""
    response = await async_http_client.post("/users", json={"name": "John"})
    assert response.status_code == 201

    data = response.json()
    assert data["name"] == "John"
```

---

## Testing Async Context Managers

### Basic Async Context Manager Testing

```python
# async_context.py
import asyncio
from typing import AsyncGenerator

class AsyncResource:
    def __init__(self, name: str):
        self.name = name
        self.initialized = False

    async def __aenter__(self) -> "AsyncResource":
        await asyncio.sleep(0.1)  # Simulate async initialization
        self.initialized = True
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await asyncio.sleep(0.1)  # Simulate async cleanup
        self.initialized = False
        return False  # Don't suppress exceptions

    async def process(self, data: str) -> str:
        if not self.initialized:
            raise RuntimeError("Resource not initialized")
        return f"processed: {data}"

# test_async_context.py
@pytest.mark.asyncio
async def test_async_context_manager():
    """Test basic async context manager."""
    resource = AsyncResource("test")

    async with resource as r:
        assert r.initialized is True
        assert r.name == "test"

        result = await r.process("test data")
        assert result == "processed: test data"

    # Resource should be cleaned up after context
    assert resource.initialized is False

@pytest.mark.asyncio
async def test_async_context_manager_with_exception():
    """Test async context manager with exception."""
    resource = AsyncResource("test")

    with pytest.raises(ValueError):
        async with resource as r:
            assert r.initialized is True
            raise ValueError("Test exception")

    # Cleanup should still happen
    assert resource.initialized is False
```

### Nested Async Context Managers

```python
@pytest.mark.asyncio
async def test_nested_async_context_managers():
    """Test nested async context managers."""

    async with AsyncResource("outer") as outer:
        assert outer.initialized is True

        async with AsyncResource("inner") as inner:
            assert inner.initialized is True
            assert outer.initialized is True  # Still active

            # Both resources can be used
            outer_result = await outer.process("outer data")
            inner_result = await inner.process("inner data")

            assert outer_result == "processed: outer data"
            assert inner_result == "processed: inner data"

        # Inner resource cleaned up, outer still active
        assert inner.initialized is False
        assert outer.initialized is True

    # Both cleaned up
    assert outer.initialized is False
```

---

## Mocking Async Code

### Mocking Async Functions

```python
import pytest
from unittest.mock import AsyncMock, patch

@pytest.mark.asyncio
async def test_mock_async_function():
    """Test mocking of async functions."""

    # Create async mock
    mock_async_func = AsyncMock(return_value="mocked result")

    # Use the mock
    result = await mock_async_func("arg1", kwarg1="value1")

    assert result == "mocked result"
    mock_async_func.assert_called_once_with("arg1", kwarg1="value1")

@pytest.mark.asyncio
async def test_mock_async_generator():
    """Test mocking of async generators."""

    # Mock async generator
    mock_gen = AsyncMock()
    mock_gen.__aiter__.return_value = iter(["item1", "item2", "item3"])

    # Use the mock generator
    items = []
    async for item in mock_gen:
        items.append(item)

    assert items == ["item1", "item2", "item3"]

@pytest.mark.asyncio
async def test_patch_async_function():
    """Test patching async functions."""

    async def real_async_func(x):
        await asyncio.sleep(0.1)
        return x * 2

    # Patch the function
    with patch(__name__ + '.real_async_func', return_value=999) as mock_func:
        result = await real_async_func(5)
        assert result == 999
        mock_func.assert_called_once_with(5)

    # Original function should work after patch
    result = await real_async_func(5)
    assert result == 10
```

### Mocking Async Context Managers

```python
@pytest.mark.asyncio
async def test_mock_async_context_manager():
    """Test mocking of async context managers."""

    # Create mock async context manager
    mock_context = AsyncMock()
    mock_context.__aenter__.return_value = "mocked resource"
    mock_context.__aexit__.return_value = None

    # Use the mock
    async with mock_context as resource:
        assert resource == "mocked resource"

    # Verify the context manager methods were called
    mock_context.__aenter__.assert_called_once()
    mock_context.__aexit__.assert_called_once()
```

### Complex Mocking Scenarios

```python
@pytest.mark.asyncio
async def test_mock_with_side_effects():
    """Test async mock with side effects."""

    async def async_side_effect(*args, **kwargs):
        await asyncio.sleep(0.01)  # Simulate async work
        if args[0] == "error":
            raise ValueError("Async error")
        return f"processed: {args[0]}"

    mock_func = AsyncMock(side_effect=async_side_effect)

    # Test normal case
    result = await mock_func("test")
    assert result == "processed: test"

    # Test error case
    with pytest.raises(ValueError, match="Async error"):
        await mock_func("error")

@pytest.mark.asyncio
async def test_mock_streaming_response():
    """Test mocking streaming async responses."""

    # Mock streaming response
    mock_stream = AsyncMock()
    mock_stream.__aiter__.return_value = iter([
        b'{"data": "chunk1"}',
        b'{"data": "chunk2"}',
        b'{"data": "chunk3"}'
    ])

    # Process stream
    chunks = []
    async for chunk in mock_stream:
        chunks.append(chunk)

    assert len(chunks) == 3
    assert chunks[0] == b'{"data": "chunk1"}'
```

---

## Error Handling in Async Tests

### Testing Async Exceptions

```python
@pytest.mark.asyncio
async def test_async_exception_handling():
    """Test exception handling in async code."""

    async def failing_async_operation():
        await asyncio.sleep(0.1)
        raise ValueError("Async operation failed")

    # Test that exception is raised
    with pytest.raises(ValueError, match="Async operation failed"):
        await failing_async_operation()

@pytest.mark.asyncio
async def test_async_exception_suppression():
    """Test exception suppression in context manager."""

    class SuppressingAsyncContext:
        async def __aenter__(self):
            return self

        async def __aexit__(self, exc_type, exc_val, exc_tb):
            if isinstance(exc_val, ValueError):
                return True  # Suppress ValueError
            return False  # Don't suppress other exceptions

    # Exception should be suppressed
    async with SuppressingAsyncContext():
        raise ValueError("This should be suppressed")

    # Other exceptions should not be suppressed
    with pytest.raises(RuntimeError):
        async with SuppressingAsyncContext():
            raise RuntimeError("This should not be suppressed")

@pytest.mark.asyncio
async def test_async_timeout_exception():
    """Test timeout exception handling."""

    async def slow_operation():
        await asyncio.sleep(2.0)
        return "completed"

    with pytest.raises(asyncio.TimeoutError):
        await asyncio.wait_for(slow_operation(), timeout=1.0)
```

### Async Cleanup on Exception

```python
@pytest.mark.asyncio
async def test_cleanup_on_exception():
    """Test that cleanup happens even when exception occurs."""
    cleanup_called = False

    class AsyncResourceWithCleanup:
        async def __aenter__(self):
            return self

        async def __aexit__(self, exc_type, exc_val, exc_tb):
            nonlocal cleanup_called
            cleanup_called = True
            await asyncio.sleep(0.01)  # Simulate cleanup work
            return False  # Don't suppress exception

        async def do_work(self):
            raise ValueError("Work failed")

    resource = AsyncResourceWithCleanup()

    with pytest.raises(ValueError):
        async with resource:
            await resource.do_work()

    # Verify cleanup was called
    assert cleanup_called is True
```

---

## Performance Testing Async Code

### Concurrency Testing

```python
@pytest.mark.asyncio
async def test_concurrent_async_operations():
    """Test performance of concurrent async operations."""

    async def simulated_io_task(task_id: int, delay: float = 0.1):
        await asyncio.sleep(delay)
        return f"task_{task_id}_completed"

    # Test sequential execution
    start_time = asyncio.get_event_loop().time()
    sequential_results = []
    for i in range(5):
        result = await simulated_io_task(i)
        sequential_results.append(result)
    sequential_time = asyncio.get_event_loop().time() - start_time

    # Test concurrent execution
    start_time = asyncio.get_event_loop().time()
    concurrent_tasks = [simulated_io_task(i) for i in range(5)]
    concurrent_results = await asyncio.gather(*concurrent_tasks)
    concurrent_time = asyncio.get_event_loop().time() - start_time

    # Verify results are the same
    assert sorted(sequential_results) == sorted(concurrent_results)

    # Concurrent should be significantly faster
    assert concurrent_time < sequential_time * 0.8  # At least 20% faster

@pytest.mark.asyncio
async def test_async_semaphore_limiter():
    """Test rate limiting with semaphore."""

    semaphore = asyncio.Semaphore(2)  # Max 2 concurrent operations
    concurrent_count = 0
    max_concurrent = 0

    async def limited_operation():
        nonlocal concurrent_count, max_concurrent
        async with semaphore:
            concurrent_count += 1
            max_concurrent = max(max_concurrent, concurrent_count)
            await asyncio.sleep(0.1)
            concurrent_count -= 1

    # Start 5 operations
    tasks = [limited_operation() for _ in range(5)]
    await asyncio.gather(*tasks)

    # Verify concurrency was limited
    assert max_concurrent <= 2
```

### Async Memory Testing

```python
@pytest.mark.asyncio
async def test_async_memory_usage():
    """Test memory usage of async operations."""
    import gc
    import sys

    # Memory usage before
    gc.collect()
    initial_objects = len(gc.get_objects())

    # Create many async tasks
    async def memory_intensive_task():
        data = list(range(1000))  # Allocate memory
        await asyncio.sleep(0.01)
        return len(data)

    tasks = [memory_intensive_task() for _ in range(100)]
    results = await asyncio.gather(*tasks)

    # Verify results
    assert len(results) == 100
    assert all(r == 1000 for r in results)

    # Force garbage collection
    del tasks, results
    gc.collect()

    # Memory usage after cleanup
    final_objects = len(gc.get_objects())

    # Should not have significant memory leak (allow some tolerance)
    assert final_objects - initial_objects < 1000
```

---

## Integration Testing Async APIs

### FastAPI Async Testing

```python
# test_async_fastapi.py
from fastapi import FastAPI
from fastapi.testclient import TestClient
import httpx

app = FastAPI()

@app.post("/users")
async def create_user(user_data: dict):
    await asyncio.sleep(0.1)  # Simulate async work
    return {"id": 1, "name": user_data["name"], "status": "created"}

@pytest.mark.asyncio
async def test_fastapi_async_endpoint():
    """Test FastAPI async endpoint."""

    async with httpx.AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post("/users", json={"name": "John"})

        assert response.status_code == 200
        data = response.json()
        assert data["name"] == "John"
        assert data["status"] == "created"
```

### WebSocket Testing

```python
# test_websocket.py
import pytest
import asyncio
from fastapi import FastAPI, WebSocket
from fastapi.testclient import TestClient

app = FastAPI()

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    while True:
        data = await websocket.receive_text()
        await websocket.send_text(f"Echo: {data}")

@pytest.mark.asyncio
async def test_websocket_communication():
    """Test WebSocket communication."""

    async with httpx.AsyncClient(app=app, base_url="http://test") as client:
        # Note: WebSocket testing typically requires specialized WebSocket client
        # This is a simplified example
        with pytest.raises(NotImplementedError):
            # In practice, you'd use a WebSocket client library
            pass
```

---

## Common Async Testing Patterns

### Retry Logic Testing

```python
@pytest.mark.asyncio
async def test_async_retry_logic():
    """Test retry logic with async operations."""

    attempt_count = 0

    async def flaky_operation():
        nonlocal attempt_count
        attempt_count += 1
        await asyncio.sleep(0.01)

        if attempt_count < 3:
            raise ConnectionError("Temporary failure")

        return "success"

    # Implement retry logic
    async def retry_operation(max_attempts=3):
        for attempt in range(max_attempts):
            try:
                return await flaky_operation()
            except ConnectionError as e:
                if attempt == max_attempts - 1:
                    raise
                await asyncio.sleep(0.01)

    result = await retry_operation()
    assert result == "success"
    assert attempt_count == 3  # Should have attempted 3 times
```

### Async Iterator Testing

```python
@pytest.mark.asyncio
async def test_async_iterator():
    """Test async iterator functionality."""

    async def async_data_producer():
        for i in range(5):
            await asyncio.sleep(0.01)
            yield i

    # Collect all items from async iterator
    items = []
    async for item in async_data_producer():
        items.append(item)

    assert items == [0, 1, 2, 3, 4]

@pytest.mark.asyncio
async def test_async_iterator_with_exception():
    """Test async iterator with exception."""

    async def failing_async_iterator():
        for i in range(3):
            await asyncio.sleep(0.01)
            if i == 2:
                raise ValueError("Iterator error")
            yield i

    items = []
    with pytest.raises(ValueError, match="Iterator error"):
        async for item in failing_async_iterator():
            items.append(item)

    # Should have collected items before the error
    assert items == [0, 1]
```

### Async Queue Testing

```python
@pytest.mark.asyncio
async def test_async_queue_operations():
    """Test async queue operations."""

    queue = asyncio.Queue()

    async def producer():
        for i in range(5):
            await queue.put(i)
            await asyncio.sleep(0.01)

    async def consumer():
        items = []
        while True:
            try:
                item = await asyncio.wait_for(queue.get(), timeout=0.1)
                items.append(item)
                queue.task_done()
            except asyncio.TimeoutError:
                break
        return items

    # Run producer and consumer concurrently
    producer_task = asyncio.create_task(producer())
    consumer_task = asyncio.create_task(consumer())

    await producer_task
    items = await consumer_task

    assert items == [0, 1, 2, 3, 4]
    assert queue.empty()
```

---

## Advanced Async Testing Techniques

### Async Event Testing

```python
@pytest.mark.asyncio
async def test_async_event_coordination():
    """Test coordination using async events."""

    event = asyncio.Event()
    results = []

    async def waiter(name: str):
        await event.wait()
        results.append(f"{name} proceeded")

    async def setter():
        await asyncio.sleep(0.1)
        event.set()
        results.append("Event set")

    # Start waiters
    waiter_tasks = [
        asyncio.create_task(waiter(f"Waiter{i}"))
        for i in range(3)
    ]

    # Start setter
    setter_task = asyncio.create_task(setter())

    # Wait for all tasks
    await asyncio.gather(*waiter_tasks, setter_task)

    assert "Event set" in results
    assert len(results) == 4  # 3 waiters + 1 setter

@pytest.mark.asyncio
async def test_async_condition_variable():
    """Test async condition variable for producer-consumer."""

    condition = asyncio.Condition()
    buffer = []
    buffer_size = 3

    async def producer():
        async with condition:
            for i in range(5):
                while len(buffer) >= buffer_size:
                    await condition.wait()
                buffer.append(i)
                condition.notify()
                await asyncio.sleep(0.01)

    async def consumer():
        consumed = []
        while len(consumed) < 5:
            async with condition:
                while not buffer:
                    await condition.wait()
                item = buffer.pop(0)
                consumed.append(item)
                condition.notify()
            await asyncio.sleep(0.02)
        return consumed

    producer_task = asyncio.create_task(producer())
    consumer_task = asyncio.create_task(consumer())

    await producer_task
    consumed = await consumer_task

    assert consumed == [0, 1, 2, 3, 4]
```

### Async Pool Testing

```python
@pytest.mark.asyncio
async def test_async_connection_pool():
    """Test async connection pool behavior."""

    class AsyncConnectionPool:
        def __init__(self, max_connections: int):
            self.semaphore = asyncio.Semaphore(max_connections)
            self.active_connections = 0

        async def get_connection(self):
            await self.semaphore.acquire()
            self.active_connections += 1
            return Connection(self)

        def release_connection(self):
            self.active_connections -= 1
            self.semaphore.release()

    class Connection:
        def __init__(self, pool: AsyncConnectionPool):
            self.pool = pool

        async def execute(self, query: str):
            await asyncio.sleep(0.01)
            return f"Result of: {query}"

        async def __aenter__(self):
            return self

        async def __aexit__(self, exc_type, exc_val, exc_tb):
            self.pool.release_connection()

    pool = AsyncConnectionPool(max_connections=2)

    async def use_connection(query_id: int):
        async with await pool.get_connection() as conn:
            result = await conn.execute(f"query_{query_id}")
            return result

    # Use connections concurrently
    tasks = [use_connection(i) for i in range(5)]
    results = await asyncio.gather(*tasks)

    assert len(results) == 5
    assert all("Result of: query_" in result for result in results)
    assert pool.active_connections == 0  # All connections released
```

This comprehensive guide covers most async testing scenarios you'll encounter. Remember to always configure `pytest-asyncio` properly and use appropriate mocking strategies for async code.