# Python Testing Troubleshooting Guide

This guide covers common issues and their solutions when testing Python applications.

## Table of Contents
- [pytest Configuration Issues](#pytest-configuration-issues)
- [Test Discovery Problems](#test-discovery-problems)
- [Async Testing Issues](#async-testing-issues)
- [Mocking and Patching Problems](#mocking-and-patching-problems)
- [Database Testing Issues](#database-testing-issues)
- [Coverage Issues](#coverage-issues)
- [Performance Test Problems](#performance-test-problems)
- [CI/CD Testing Issues](#cicd-testing-issues)
- [Flaky Tests](#flaky-tests)

---

## pytest Configuration Issues

### Test files not being discovered

**Problem**: pytest is not finding your test files or tests.

**Symptoms**:
```
============================= test session starts ==============================
collected 0 items
============================== no tests ran ===============================
```

**Solutions**:

1. **Check file naming convention**:
   ```bash
   # Files should be named either:
   test_*.py
   # or
   *_test.py
   ```

2. **Verify test naming**:
   ```python
   # Functions should start with test_
   def test_something():  # ✓ Correct
   def something_test():  # ✗ Incorrect

   # Classes should start with Test
   class TestSomething:  # ✓ Correct
   class SomethingTest:  # ✗ Incorrect
   ```

3. **Check pytest.ini configuration**:
   ```ini
   [tool:pytest]
   testpaths = tests
   python_files = test_*.py *_test.py
   python_classes = Test*
   python_functions = test_*
   ```

4. **Use explicit file paths**:
   ```bash
   pytest tests/test_specific_file.py
   pytest tests/test_specific_file.py::test_specific_function
   ```

### Import errors in tests

**Problem**: `ImportError` or `ModuleNotFoundError` when running tests.

**Solutions**:

1. **Install package in development mode**:
   ```bash
   pip install -e .
   ```

2. **Add source directory to PYTHONPATH**:
   ```bash
   export PYTHONPATH="${PYTHONPATH}:$(pwd)/src"
   pytest
   ```

3. **Use pytest's src layout**:
   ```
   myproject/
   ├── src/
   │   └── mypackage/
   └── tests/
       └── test_mypackage.py
   ```

4. **Use conftest.py to set up imports**:
   ```python
   # tests/conftest.py
   import sys
   import os
   sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))
   ```

---

## Test Discovery Problems

### Tests in nested directories not found

**Problem**: Tests in subdirectories are not being discovered.

**Solution**: Ensure proper `__init__.py` files and configuration:

```bash
# Directory structure
tests/
├── __init__.py
├── unit/
│   ├── __init__.py
│   └── test_module.py
└── integration/
    ├── __init__.py
    └── test_api.py
```

```ini
# pytest.ini
[tool:pytest]
testpaths = tests
python_files = test_*.py
python_classes = Test*
python_functions = test_*
```

### Parameterized tests not running

**Problem**: Parametrized tests show as collected but don't execute.

**Solutions**:

1. **Check for syntax errors**:
   ```python
   # Correct parametrize usage
   @pytest.mark.parametrize("input,expected", [
       ("test1", "result1"),
       ("test2", "result2"),
   ])
   def test_function(input, expected):
       assert process(input) == expected
   ```

2. **Verify marker registration**:
   ```ini
   # pytest.ini
   [tool:pytest]
   markers =
       parametrize: Parameterize test cases
   ```

---

## Async Testing Issues

### Async tests not executing

**Problem**: `RuntimeError: asyncio.run() cannot be called from a running event loop`

**Solutions**:

1. **Install pytest-asyncio**:
   ```bash
   pip install pytest-asyncio
   ```

2. **Configure asyncio mode**:
   ```ini
   # pytest.ini
   [tool:pytest]
   asyncio_mode = auto
   ```

3. **Use correct async syntax**:
   ```python
   @pytest.mark.asyncio
   async def test_async_function():
       result = await some_async_function()
       assert result is not None
   ```

### Async fixtures not working

**Problem**: Async fixtures don't work with regular tests.

**Solution**: Use async fixtures only with async tests:

```python
@pytest.fixture
async def async_fixture():
    return await setup_async_resource()

@pytest.mark.asyncio
async def test_with_async_fixture(async_fixture):
    result = await async_fixture.do_something()
    assert result is not None
```

### Mixed sync/async test issues

**Problem**: Mixing sync and async tests in the same test class.

**Solution**: Separate sync and async tests:

```python
class TestMixedFunctionality:
    def test_sync_functionality(self):
        # Synchronous test
        assert sync_function() works

    @pytest.mark.asyncio
    async def test_async_functionality(self):
        # Asynchronous test
        result = await async_function()
        assert result is not None
```

---

## Mocking and Patching Problems

### Patch not working

**Problem**: Mock/Patch decorator doesn't replace the target function.

**Solutions**:

1. **Patch the right location**:
   ```python
   # Patch where the function is USED, not where it's defined
   @patch('module_using_function.function_name')  # ✓ Correct
   @patch('module_defining_function.function_name')  # ✗ Incorrect
   ```

2. **Use proper import path**:
   ```python
   # If you import like: from requests import get
   @patch('requests.get')  # ✓ Correct

   # If you import like: import requests
   @patch('requests.get')  # ✓ Correct
   ```

3. **Check patch scope**:
   ```python
   # Local patch
   def test_function():
       with patch('module.function') as mock_func:
           # Test code here

   # Global patch
   @patch('module.function')
   def test_function(mock_func):
       # Test code here
   ```

### Mock assertions failing

**Problem**: `assert_not_called()` or `assert_called_with()` failing unexpectedly.

**Solutions**:

1. **Clear mock state between tests**:
   ```python
   def test_function1(mock_func):
       mock_func.reset_mock()
       # Test code
   ```

2. **Check mock call history**:
   ```python
   print(mock_func.call_args_list)
   print(mock_func.call_count)
   print(mock_func.called)
   ```

3. **Use assert_called_once_with()**:
   ```python
   mock_func.assert_called_once_with(expected_args, expected_kwargs)
   ```

### Async mocking issues

**Problem**: Mocking async functions incorrectly.

**Solution**: Use `AsyncMock` for async functions:

```python
from unittest.mock import AsyncMock

@pytest.mark.asyncio
async def test_async_function(mocker):
    # Mock async function
    mock_async = mocker.AsyncMock(return_value="async result")

    # Use the mock
    result = await some_async_function(mock_async)

    # Assertions
    mock_async.assert_awaited_once()
    assert result == "async result"
```

---

## Database Testing Issues

### Test database isolation

**Problem**: Tests are affecting each other through the database.

**Solutions**:

1. **Use transactions and rollbacks**:
   ```python
   @pytest.fixture
   def db_session():
       engine = create_engine("sqlite:///:memory:")
       Session = sessionmaker(bind=engine)
       session = Session()

       transaction = session.begin_nested()
       yield session

       transaction.rollback()
       session.close()
   ```

2. **Clean up after each test**:
   ```python
   @pytest.fixture(autouse=True)
   def cleanup_database(db_session):
       yield
       db_session.query(MyModel).delete()
       db_session.commit()
   ```

3. **Use separate databases for each test**:
   ```python
   @pytest.fixture
   def db_session(tmp_path):
       db_file = tmp_path / "test.db"
       engine = create_engine(f"sqlite:///{db_file}")
       # Setup database
       yield session
   ```

### Foreign key constraint issues

**Problem**: Database tests failing due to foreign key constraints.

**Solutions**:

1. **Create data in correct order**:
   ```python
   def test_with_foreign_keys(db_session):
       # Create parent first
       parent = Parent(name="Test Parent")
       db_session.add(parent)
       db_session.commit()

       # Then create child
       child = Child(name="Test Child", parent_id=parent.id)
       db_session.add(child)
       db_session.commit()
   ```

2. **Use cascade relationships**:
   ```python
   class Child(Base):
       parent_id = Column(Integer, ForeignKey('parent.id', ondelete='CASCADE'))
       parent = relationship("Parent", back_populates="children")
   ```

### Migration testing problems

**Problem**: Migration tests are slow or unreliable.

**Solutions**:

1. **Use in-memory databases**:
   ```python
   @pytest.fixture
   def memory_engine():
       return create_engine("sqlite:///:memory:")
   ```

2. **Test migrations in isolation**:
   ```python
   def test_single_migration(alembic_config):
       # Test just one migration
       alembic.command.upgrade(alembic_config, "+1")
       # Test database state
       alembic.command.downgrade(alembic_config, "-1")
   ```

---

## Coverage Issues

### Coverage not showing tests

**Problem**: Coverage report shows 0% coverage despite running tests.

**Solutions**:

1. **Install pytest-cov**:
   ```bash
   pip install pytest-cov
   ```

2. **Configure coverage source**:
   ```ini
   # .coveragerc
   [run]
   source = src
   omit =
       tests/*
       __pycache__/*
       .venv/*
   ```

3. **Use correct pytest command**:
   ```bash
   pytest --cov=src tests/
   pytest --cov=src --cov-report=html tests/
   ```

### Excluding code from coverage

**Problem**: Need to exclude certain code from coverage calculation.

**Solutions**:

1. **Use pragma comments**:
   ```python
   def debug_function():
       # pragma: no cover
       print("This is debug code")
   ```

2. **Configure in .coveragerc**:
   ```ini
   [run]
   omit =
       tests/*
       migrations/*
       venv/*
       */migrations/*
   ```

3. **Use coverage configuration**:
   ```ini
   [report]
   exclude_lines =
       pragma: no cover
       def __repr__
       raise AssertionError
       raise NotImplementedError
   ```

---

## Performance Test Problems

### Inconsistent performance results

**Problem**: Performance tests show inconsistent timing.

**Solutions**:

1. **Use benchmarking libraries**:
   ```bash
   pip install pytest-benchmark
   ```

2. **Run multiple iterations**:
   ```python
   @pytest.mark.parametrize("iteration", range(10))
   def test_performance_consistency(iteration):
       result = expensive_operation()
       assert result is not None
   ```

3. **Control test environment**:
   ```python
   @pytest.fixture(autouse=True)
   def setup_environment():
       # Disable GC during performance tests
       import gc
       gc.disable()
       yield
       gc.enable()
   ```

### Resource cleanup in performance tests

**Problem**: Performance tests leaving resources open.

**Solutions**:

1. **Explicit resource cleanup**:
   ```python
   def test_with_cleanup():
       resource = allocate_resource()
       try:
           # Performance test here
           pass
       finally:
           resource.cleanup()
   ```

2. **Use context managers**:
   ```python
   def test_with_context_manager():
       with allocate_resource() as resource:
           # Performance test here
           pass
   ```

---

## CI/CD Testing Issues

### Tests passing locally but failing in CI

**Problem**: Tests work on local machine but fail in CI environment.

**Solutions**:

1. **Check environment differences**:
   ```yaml
   # GitHub Actions example
   - name: Debug environment
     run: |
       python --version
       pip list
       printenv
   ```

2. **Use consistent Python versions**:
   ```yaml
   # .github/workflows/test.yml
   strategy:
     matrix:
       python-version: [3.10, 3.11, 3.12]
   ```

3. **Install all dependencies**:
   ```yaml
   - name: Install dependencies
     run: |
       pip install -e .[dev,test]
       pip install pytest pytest-cov pytest-asyncio
   ```

### Test timing out in CI

**Problem**: Tests timeout in CI due to slow execution.

**Solutions**:

1. **Increase timeout values**:
   ```yaml
   - name: Run tests
     run: pytest
     timeout-minutes: 10
   ```

2. **Use parallel execution**:
   ```bash
   pytest -n auto  # Use pytest-xdist
   ```

3. **Run tests faster**:
   ```bash
   # Skip slow tests in CI
   pytest -m "not slow" --cov=src
   ```

---

## Flaky Tests

### Tests failing intermittently

**Problem**: Tests sometimes pass, sometimes fail.

**Solutions**:

1. **Identify race conditions**:
   ```python
   import time

   def test_with_retry():
       max_attempts = 3
       for attempt in range(max_attempts):
           try:
               result = potentially_flaky_operation()
               assert result is not None
               break
           except AssertionError:
               if attempt == max_attempts - 1:
                   raise
               time.sleep(0.1)
   ```

2. **Fix timing dependencies**:
   ```python
   # Use explicit waits instead of time.sleep()
   import asyncio

   @pytest.mark.asyncio
   async def test_async_operation():
       # Wait for condition instead of fixed delay
       await wait_for_condition(lambda: check_condition(), timeout=5.0)
   ```

3. **Isolate tests better**:
   ```python
   @pytest.fixture(autouse=True)
   def isolate_tests():
       # Setup isolation
       yield
       # Cleanup isolation
   ```

### Database-related flaky tests

**Problem**: Database tests failing due to connection or transaction issues.

**Solutions**:

1. **Use connection pooling**:
   ```python
   @pytest.fixture
   def db_engine():
       engine = create_engine(
           "postgresql://user:pass@localhost/testdb",
           pool_size=1,
           max_overflow=0
       )
       yield engine
   ```

2. **Implement retry logic**:
   ```python
   import tenacity

   @tenacity.retry(
       stop=tenacity.stop_after_attempt(3),
       wait=tenacity.wait_exponential(multiplier=1, min=1, max=10)
   )
   def test_database_operation():
       # Test code that might fail due to transient issues
       pass
   ```

## Debugging Tips

### Enable verbose output
```bash
pytest -v -s tests/
pytest --tb=long tests/
```

### Run specific tests
```bash
pytest tests/test_file.py::test_function
pytest -k "test_name" tests/
```

### Debug failed tests
```bash
pytest --pdb tests/
pytest --pdb -x tests/  # Stop at first failure
```

### Show test collection
```bash
pytest --collect-only tests/
```

### Use profiling
```bash
pytest --profile tests/  # Requires pytest-profiling
```

Remember to always check the specific error messages and stack traces, as they often provide the most direct clues to the solution.