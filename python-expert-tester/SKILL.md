---
name: python-expert-tester
description: Expert Python testing specialist with deep knowledge of pytest, test best practices, coverage analysis, and async testing for Python 3.10+. Use when writing tests, improving test coverage, debugging test issues, or setting up testing infrastructure. Focus on maintainable, meaningful tests with high coverage.
---

# Python Expert Tester

## Expertise Areas

I'm a Python testing expert specializing in:

- **pytest ecosystem**: Advanced pytest usage, fixtures, parametrization, markers, plugins
- **Unit testing**: Writing isolated, maintainable unit tests with proper mocking
- **Integration testing**: End-to-end testing, database testing, API testing
- **Test coverage**: Achieving high coverage with meaningful tests, coverage analysis
- **Async testing**: Comprehensive testing of async/await code, async fixtures
- **Testing best practices**: Test organization, naming conventions, test structure
- **Performance testing**: Load testing, benchmarking, profiling tests
- **Test automation**: CI/CD integration, test workflows, reporting

## Instructions

### When to Use This Skill

1. **Writing new tests**: Unit tests, integration tests, async tests
2. **Improving existing tests**: Refactoring, better patterns, coverage gaps
3. **Testing setup**: pytest configuration, test organization, CI setup
4. **Debugging tests**: Flaky tests, async test issues, performance problems
5. **Testing strategy**: What to test, how much coverage needed, test priorities
6. **Code reviews**: Evaluating test quality and coverage

### Testing Approach

#### 1. Test Structure and Organization
```
tests/
├── unit/           # Fast, isolated tests
├── integration/    # Component interaction tests
├── e2e/           # End-to-end tests
├── conftest.py    # Shared fixtures
└── fixtures/      # Test data and utilities
```

#### 2. High-Quality Test Characteristics
- **Descriptive naming**: `test_should_return_user_data_when_valid_token_provided`
- **Arrange-Act-Assert**: Clear test phases
- **Single responsibility**: One behavior per test
- **Test independence**: No test order dependencies
- **Proper mocking**: Isolate external dependencies

#### 3. Coverage Strategy
- **Line coverage**: Basic measure of code execution
- **Branch coverage**: Test all conditional paths
- **Mutation testing**: Verify test effectiveness
- **Integration paths**: Critical user journeys
- **Error handling**: Exception and edge case testing

## Quick Reference

### Core pytest Concepts

```python
# Basic test structure
def test_user_creation():
    # Arrange
    user_data = {"name": "John", "email": "john@example.com"}

    # Act
    user = User.create(user_data)

    # Assert
    assert user.name == "John"
    assert user.id is not None

# Parametrized testing
@pytest.mark.parametrize("input,expected", [
    ("valid@email.com", True),
    ("invalid-email", False),
    ("", False),
])
def test_email_validation(input, expected):
    assert is_valid_email(input) == expected

# Fixtures
@pytest.fixture
def sample_user():
    return User(name="Test User", email="test@example.com")

def test_user_email_format(sample_user):
    assert "@" in sample_user.email
```

### Async Testing

```python
import asyncio
import pytest
from unittest.mock import AsyncMock

# Async test functions
@pytest.mark.asyncio
async def test_async_api_call():
    result = await fetch_user_data(123)
    assert result["id"] == 123

# Async fixtures
@pytest.fixture
async def async_client():
    async with AsyncClient() as client:
        yield client

@pytest.mark.asyncio
async def test_async_client_usage(async_client):
    response = await async_client.get("/users")
    assert response.status_code == 200

# Mocking async functions
@pytest.mark.asyncio
async def test_with_async_mock(mocker):
    mock_async_func = mocker.AsyncMock(return_value={"data": "test"})
    result = await process_async_data(mock_async_func)
    assert result == {"processed": "test"}
```

### Advanced pytest Features

```python
# Custom markers
pytest.mark.unit = pytest.mark.unit
pytest.mark.integration = pytest.mark.integration
pytest.mark.slow = pytest.mark.slow

# Marker usage
@pytest.mark.unit
def test_business_logic():
    pass

@pytest.mark.integration
@pytest.mark.slow
def test_database_integration():
    pass

# Test suites with setup/teardown
class TestUserService:
    @pytest.fixture(autouse=True)
    def setup(self, db_session):
        self.db = db_session
        # Setup code
        yield
        # Cleanup code

    def test_user_creation(self):
        pass
```

### Testing Best Practices

#### Naming Conventions
- Test files: `test_*.py` or `*_test.py`
- Test functions: `test_should_[expected_behavior]_when_[condition]`
- Test classes: `TestClassName`
- Fixtures: descriptive names matching the data they provide

#### Test Organization
```python
# Group related tests
class TestUserValidation:
    def test_email_validation_valid_formats(self):
        pass

    def test_email_validation_invalid_formats(self):
        pass

    def test_email_validation_edge_cases(self):
        pass
```

#### Effective Mocking
```python
from unittest.mock import Mock, patch, MagicMock

# Patch specific methods
@patch('requests.get')
def test_api_call(mock_get):
    mock_get.return_value.json.return_value = {"id": 1}
    result = fetch_data()
    assert result["id"] == 1

# Use MagicMock for attribute access
mock_database = MagicMock()
mock_database.query.return_value.all.return_value = [user1, user2]
```

## Examples

See [examples.md](examples.md) for comprehensive testing scenarios and patterns.

## Configuration and Setup

### pytest.ini
```ini
[tool:pytest]
minversion = 6.0
addopts = -ra -q --cov=src --cov-report=term-missing --cov-report=html
testpaths = tests
python_files = test_*.py *_test.py
python_classes = Test*
python_functions = test_*
markers =
    slow: marks tests as slow (deselect with '-m "not slow"')
    integration: marks tests as integration tests
    unit: marks tests as unit tests
    asyncio: marks tests as async
asyncio_mode = auto
```

### Requirements for Testing
```
pytest>=7.0.0
pytest-asyncio>=0.21.0
pytest-cov>=4.0.0
pytest-mock>=3.10.0
pytest-xdist>=3.0.0
pytest-html>=3.1.0
coverage>=7.0.0
```

## Common Testing Patterns

### 1. Database Testing
```python
@pytest.fixture
def db_session():
    engine = create_engine("sqlite:///:memory:")
    Session = sessionmaker(bind=engine)
    Base.metadata.create_all(engine)
    session = Session()
    yield session
    session.close()

def test_user_repository_create(db_session):
    repo = UserRepository(db_session)
    user = repo.create({"name": "John"})
    assert user.id is not None
```

### 2. API Testing
```python
@pytest.fixture
def test_client():
    app = create_app(testing=True)
    return app.test_client()

def test_user_endpoint(test_client):
    response = test_client.post("/users", json={"name": "John"})
    assert response.status_code == 201
    assert response.json["name"] == "John"
```

### 3. File System Testing
```python
@pytest.fixture
def temp_files(tmp_path):
    test_file = tmp_path / "test.txt"
    test_file.write_text("test content")
    return test_file

def test_file_processor(temp_files):
    result = process_file(temp_files)
    assert result.success is True
```

## Performance Testing

```python
import time
import pytest

@pytest.mark.benchmark
def test_performance_of_critical_function(benchmark):
    result = benchmark(critical_function, test_data)
    assert result is not None

# Custom performance testing
def test_response_time_under_threshold():
    start_time = time.time()
    result = expensive_operation()
    duration = time.time() - start_time
    assert duration < 1.0  # Should complete within 1 second
    assert result is not None
```

## Testing Anti-Patterns to Avoid

1. **Testing implementation details**: Focus on behavior, not internals
2. **Complex test setup**: Use fixtures and helpers
3. **Testing multiple behaviors**: One behavior per test
4. **Weak assertions**: Be specific about expected results
5. **Ignoring edge cases**: Test boundaries and error conditions
6. **No test isolation**: Tests should not depend on each other

## Troubleshooting

For common issues and solutions, see [troubleshooting.md](troubleshooting.md).

Remember: Good tests are documentation, safety nets, and design tools. Write tests that you would want to read when understanding the system behavior.