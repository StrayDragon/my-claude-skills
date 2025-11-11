# Python Testing Examples

This file contains comprehensive examples for common testing scenarios in Python.

## Table of Contents
- [Basic Unit Testing](#basic-unit-testing)
- [Mocking and Patching](#mocking-and-patching)
- [Async Testing](#async-testing)
- [Database Testing](#database-testing)
- [API Testing](#api-testing)
- [File System Testing](#file-system-testing)
- [Performance Testing](#performance-testing)
- [Error Handling Testing](#error-handling-testing)
- [Property-Based Testing](#property-based-testing)

---

## Basic Unit Testing

### Simple Business Logic
```python
# calculator.py
class Calculator:
    def add(self, a, b):
        return a + b

    def divide(self, a, b):
        if b == 0:
            raise ValueError("Cannot divide by zero")
        return a / b

# test_calculator.py
class TestCalculator:
    def test_add_positive_numbers(self):
        calc = Calculator()
        assert calc.add(2, 3) == 5

    def test_add_negative_numbers(self):
        calc = Calculator()
        assert calc.add(-1, -4) == -5

    def test_divide_normal_case(self):
        calc = Calculator()
        assert calc.divide(10, 2) == 5

    def test_divide_by_zero_raises_error(self):
        calc = Calculator()
        with pytest.raises(ValueError, match="Cannot divide by zero"):
            calc.divide(10, 0)
```

### Parametrized Testing
```python
# Test multiple scenarios with one test
@pytest.mark.parametrize("a,b,expected", [
    (2, 3, 5),
    (-1, 1, 0),
    (0, 0, 0),
    (100, -50, 50),
])
def test_calculator_add(a, b, expected):
    calc = Calculator()
    assert calc.add(a, b) == expected

# Test edge cases
@pytest.mark.parametrize("input_value,should_raise", [
    ("valid@email.com", False),
    ("invalid-email", True),
    ("", True),
    ("a@b.c", False),
    (None, True),
])
def test_email_validation(input_value, should_raise):
    if should_raise:
        with pytest.raises(ValidationError):
            validate_email(input_value)
    else:
        assert validate_email(input_value) is True
```

---

## Mocking and Patching

### Patching External Dependencies
```python
# service.py
import requests

def get_user_data(user_id):
    response = requests.get(f"https://api.example.com/users/{user_id}")
    return response.json()

# test_service.py
import pytest
from unittest.mock import patch

class TestUserService:
    @patch('requests.get')
    def test_get_user_data_success(self, mock_get):
        # Arrange
        mock_response = Mock()
        mock_response.json.return_value = {"id": 123, "name": "John"}
        mock_get.return_value = mock_response

        # Act
        result = get_user_data(123)

        # Assert
        assert result["id"] == 123
        assert result["name"] == "John"
        mock_get.assert_called_once_with("https://api.example.com/users/123")

    @patch('requests.get')
    def test_get_user_data_api_error(self, mock_get):
        # Arrange
        mock_get.side_effect = requests.exceptions.RequestException("API Error")

        # Act & Assert
        with pytest.raises(requests.exceptions.RequestException):
            get_user_data(123)
```

### Mocking Database Operations
```python
# database.py
class UserRepository:
    def __init__(self, session):
        self.session = session

    def get_user_by_id(self, user_id):
        return self.session.query(User).filter(User.id == user_id).first()

# test_database.py
def test_get_user_by_id_found():
    # Arrange
    mock_session = Mock()
    mock_user = User(id=1, name="John")
    mock_session.query.return_value.filter.return_value.first.return_value = mock_user

    repo = UserRepository(mock_session)

    # Act
    result = repo.get_user_by_id(1)

    # Assert
    assert result.id == 1
    assert result.name == "John"

def test_get_user_by_id_not_found():
    # Arrange
    mock_session = Mock()
    mock_session.query.return_value.filter.return_value.first.return_value = None

    repo = UserRepository(mock_session)

    # Act
    result = repo.get_user_by_id(999)

    # Assert
    assert result is None
```

### Using pytest-mock
```python
def test_with_pytest_mocker(mocker):
    # mocker is automatically provided by pytest-mock
    mock_func = mocker.patch('module.function_name')
    mock_func.return_value = "mocked result"

    result = module.function_name()
    assert result == "mocked result"

    # Verify the function was called
    mock_func.assert_called_once()
```

---

## Async Testing

### Basic Async Functions
```python
# async_service.py
import asyncio
import aiohttp

async def fetch_user_data(user_id):
    async with aiohttp.ClientSession() as session:
        async with session.get(f"https://api.example.com/users/{user_id}") as response:
            return await response.json()

async def process_multiple_users(user_ids):
    tasks = [fetch_user_data(user_id) for user_id in user_ids]
    return await asyncio.gather(*tasks)

# test_async_service.py
import pytest
from unittest.mock import AsyncMock

@pytest.mark.asyncio
async def test_fetch_user_data():
    mock_response = AsyncMock()
    mock_response.json.return_value = {"id": 123, "name": "John"}

    mock_session = AsyncMock()
    mock_session.get.return_value.__aenter__.return_value = mock_response

    with patch('aiohttp.ClientSession', return_value=mock_session):
        result = await fetch_user_data(123)
        assert result["id"] == 123
        assert result["name"] == "John"

@pytest.mark.asyncio
async def test_process_multiple_users():
    # Mock the fetch_user_data function
    with patch('async_service.fetch_user_data') as mock_fetch:
        mock_fetch.side_effect = [
            {"id": 1, "name": "John"},
            {"id": 2, "name": "Jane"},
        ]

        results = await process_multiple_users([1, 2])

        assert len(results) == 2
        assert results[0]["name"] == "John"
        assert results[1]["name"] == "Jane"

        # Verify both users were fetched
        assert mock_fetch.call_count == 2
```

### Async Fixtures
```python
@pytest.fixture
async def async_test_client():
    app = create_async_app(testing=True)
    async with AsyncTestClient(app) as client:
        yield client

@pytest.mark.asyncio
async def test_async_endpoint(async_test_client):
    response = await async_test_client.post("/users", json={"name": "John"})
    assert response.status_code == 201
    data = await response.json()
    assert data["name"] == "John"

# Database async fixture
@pytest.fixture
async def async_db_session():
    engine = create_async_engine("sqlite+aiosqlite:///:memory:")
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)

    async_session = async_sessionmaker(engine, class_=AsyncSession, expire_on_commit=False)

    async with async_session() as session:
        yield session
```

### Testing Async Context Managers
```python
@pytest.mark.asyncio
async def test_async_context_manager():
    class AsyncResource:
        async def __aenter__(self):
            return self

        async def __aexit__(self, exc_type, exc_val, exc_tb):
            pass

        async def process(self, data):
            return f"processed: {data}"

    async with AsyncResource() as resource:
        result = await resource.process("test data")

    assert result == "processed: test data"
```

---

## Database Testing

### SQLAlchemy Testing
```python
# models.py
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

Base = declarative_base()

class User(Base):
    __tablename__ = 'users'
    id = Column(Integer, primary_key=True)
    name = Column(String)
    email = Column(String)

# test_models.py
import pytest
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

@pytest.fixture
def db_session():
    # Create in-memory SQLite database
    engine = create_engine("sqlite:///:memory:")
    Base.metadata.create_all(engine)

    Session = sessionmaker(bind=engine)
    session = Session()

    yield session

    session.close()

def test_create_user(db_session):
    user = User(name="John Doe", email="john@example.com")
    db_session.add(user)
    db_session.commit()

    retrieved_user = db_session.query(User).filter(User.name == "John Doe").first()
    assert retrieved_user is not None
    assert retrieved_user.email == "john@example.com"
    assert retrieved_user.id is not None
```

### Alembic Migration Testing
```python
@pytest.fixture
def alembic_config():
    return Config("alembic.ini")

def test_migration_upgrade_downgrade(alembic_config):
    # Get the current head revision
    head = alembic.script.get_current_head()

    # Downgrade to base
    alembic.command.downgrade(alembic_config, "base")

    # Upgrade back to head
    alembic.command.upgrade(alembic_config, head)

    # Verify we can create a session and work with the database
    engine = create_engine("sqlite:///:memory:")
    Session = sessionmaker(bind=engine)
    session = Session()
    session.close()
```

---

## API Testing

### Flask API Testing
```python
# app.py
from flask import Flask, jsonify, request

app = Flask(__name__)

@app.route('/users', methods=['POST'])
def create_user():
    data = request.get_json()
    if not data.get('name'):
        return jsonify({"error": "Name is required"}), 400

    user_id = len(data) + 1  # Simplified user creation
    return jsonify({"id": user_id, "name": data["name"]}), 201

# test_flask_api.py
import pytest
from app import app

@pytest.fixture
def client():
    app.config['TESTING'] = True
    with app.test_client() as client:
        yield client

def test_create_user_success(client):
    response = client.post('/users', json={'name': 'John Doe'})
    assert response.status_code == 201
    data = response.get_json()
    assert data['name'] == 'John Doe'
    assert 'id' in data

def test_create_user_missing_name(client):
    response = client.post('/users', json={'email': 'john@example.com'})
    assert response.status_code == 400
    data = response.get_json()
    assert data['error'] == 'Name is required'

def test_create_user_invalid_json(client):
    response = client.post('/users', data='invalid json',
                          content_type='application/json')
    assert response.status_code == 400
```

### FastAPI Testing
```python
# main.py
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class User(BaseModel):
    name: str
    email: str

users = {}

@app.post("/users/", response_model=User)
async def create_user(user: User):
    user_id = len(users) + 1
    users[user_id] = user
    return User(id=user_id, **user.dict())

# test_fastapi.py
import pytest
from fastapi.testclient import TestClient
from main import app

client = TestClient(app)

def test_create_user():
    response = client.post("/users/", json={"name": "John", "email": "john@example.com"})
    assert response.status_code == 200
    data = response.json()
    assert data["name"] == "John"
    assert data["email"] == "john@example.com"
    assert "id" in data

def test_create_user_invalid_data():
    response = client.post("/users/", json={"email": "john@example.com"})
    assert response.status_code == 422  # Validation error
```

---

## File System Testing

### File Operations
```python
# file_processor.py
import os
from pathlib import Path

def process_file_content(file_path):
    if not os.path.exists(file_path):
        raise FileNotFoundError(f"File {file_path} not found")

    with open(file_path, 'r') as file:
        content = file.read()

    return content.upper()

# test_file_processor.py
def test_process_file_content(tmp_path):
    # Create a test file
    test_file = tmp_path / "test.txt"
    test_file.write_text("hello world")

    result = process_file_content(str(test_file))
    assert result == "HELLO WORLD"

def test_process_file_content_not_found():
    with pytest.raises(FileNotFoundError, match="File .* not found"):
        process_file_content("non_existent_file.txt")

def test_process_empty_file(tmp_path):
    empty_file = tmp_path / "empty.txt"
    empty_file.write_text("")

    result = process_file_content(str(empty_file))
    assert result == ""
```

### CSV Processing
```python
import pytest
import csv

def test_csv_processing(tmp_path):
    # Create test CSV file
    csv_file = tmp_path / "test.csv"
    csv_content = [
        ["Name", "Age", "City"],
        ["John", "30", "New York"],
        ["Jane", "25", "Los Angeles"]
    ]

    with open(csv_file, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(csv_content)

    # Test CSV reading
    with open(csv_file, 'r') as file:
        reader = csv.DictReader(file)
        rows = list(reader)

    assert len(rows) == 2
    assert rows[0]["Name"] == "John"
    assert rows[1]["City"] == "Los Angeles"
```

---

## Performance Testing

### Benchmark Testing
```python
import time
import pytest

def test_sort_performance():
    # Test with large dataset
    large_list = list(range(10000))[::-1]  # Reverse sorted list

    start_time = time.time()
    sorted_list = sorted(large_list)
    duration = time.time() - start_time

    assert duration < 1.0  # Should complete within 1 second
    assert sorted_list == list(range(10000))

# Using pytest-benchmark (requires pytest-benchmark package)
def test_string_concatenation_performance(benchmark):
    def concat_strings():
        result = ""
        for i in range(1000):
            result += str(i)
        return result

    result = benchmark(concat_strings)
    assert len(result) > 0
```

### Memory Usage Testing
```python
import pytest
import gc
import sys

def test_memory_usage():
    # Test memory usage of a function
    def create_large_list():
        return list(range(100000))

    # Get baseline memory
    gc.collect()
    baseline = sys.getrefcount(None)  # Rough indicator

    large_list = create_large_list()

    # Verify the list exists
    assert len(large_list) == 100000

    # Clean up
    del large_list
    gc.collect()
```

---

## Error Handling Testing

### Exception Handling
```python
# error_handler.py
class CustomError(Exception):
    pass

def risky_operation(should_fail=False):
    if should_fail:
        raise CustomError("This operation failed")
    return "success"

# test_error_handling.py
def test_successful_operation():
    result = risky_operation(should_fail=False)
    assert result == "success"

def test_failed_operation():
    with pytest.raises(CustomError, match="This operation failed"):
        risky_operation(should_fail=True)

def test_exception_attributes():
    try:
        risky_operation(should_fail=True)
        assert False, "Should have raised CustomError"
    except CustomError as e:
        assert str(e) == "This operation failed"
```

### Context Manager Error Handling
```python
class FileHandler:
    def __init__(self, file_path):
        self.file_path = file_path
        self.file = None

    def __enter__(self):
        try:
            self.file = open(self.file_path, 'r')
            return self.file
        except FileNotFoundError:
            raise FileNotFoundError(f"Could not find file: {self.file_path}")

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.file:
            self.file.close()
        return False  # Don't suppress exceptions

def test_context_manager_file_not_found():
    with pytest.raises(FileNotFoundError, match="Could not find file"):
        with FileHandler("non_existent.txt") as f:
            pass

def test_context_manager_proper_cleanup(tmp_path):
    test_file = tmp_path / "test.txt"
    test_file.write_text("test content")

    with FileHandler(str(test_file)) as f:
        content = f.read()
        assert content == "test content"

    # File should be closed after context
    assert f.closed
```

---

## Property-Based Testing

### Using Hypothesis
```python
from hypothesis import given, strategies as st

@given(st.integers(min_value=0, max_value=100))
def test_always_positive(x):
    assert x >= 0

@given(st.lists(st.integers()))
def test_sort_is_idempotent(lst):
    assert sorted(sorted(lst)) == sorted(lst)

@given(st.text())
def test_reverse_twice_returns_original(text):
    assert text == text[::-1][::-1]

@given(st.dictionaries(keys=st.text(), values=st.integers()))
def test_dict_len_matches_items(dict_data):
    assert len(dict_data) == len(dict_data.items())
```

### Advanced Property-Based Testing
```python
@given(st.lists(st.integers()))
def test_sorted_list_properties(lst):
    sorted_lst = sorted(lst)

    # Length is preserved
    assert len(sorted_lst) == len(lst)

    # Elements are sorted
    for i in range(len(sorted_lst) - 1):
        assert sorted_lst[i] <= sorted_lst[i + 1]

    # Contains same elements (multiset equality)
    from collections import Counter
    assert Counter(sorted_lst) == Counter(lst)

@given(st.tuples(st.text(min_size=1), st.text(min_size=1)))
def test_string_concat_properties(str1, str2):
    result = str1 + str2

    # Length property
    assert len(result) == len(str1) + len(str2)

    # Prefix/suffix properties
    assert result.startswith(str1)
    assert result.endswith(str2)
```

These examples cover most common testing scenarios in Python. Adapt them to your specific codebase and testing needs.