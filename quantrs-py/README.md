# quantrs-py

Python bindings for the quantrs technical analysis library.

## Features

- Fast Rust-powered implementations of standard technical analysis indicators
- NumPy integration
- Full type hint support for better IDE experience
- Production-ready performance for financial applications
- Comprehensive collection of technical indicators and overlays

## Installation

Install using uv:
```bash
uv pip install quantrs
```

## Usage

```python
import quantrs

# Calculate SMA
data = [1.0, 2.0, 3.0, 4.0, 5.0]
sma_values = quantrs.sma(data, 3)
print(f"SMA values: {sma_values}")

# Use streaming functionality
sma_stream = quantrs.SMAStream(3)
for value in data:
    sma_value = sma_stream.next(value)
    print(f"New SMA value: {sma_value}")
```

## Development

First, install uv if you haven't already:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Development commands are handled through the Makefile in the root directory:

```bash
# Set up development environment
make .venv

# Build both Rust and Python packages
make build

# Run all tests
make test

# Format code
make fmt

# Run linters
make lint

# Clean build artifacts and virtual environment
make clean

# Publishing

To publish to PyPI, you'll need API tokens for both TestPyPI and PyPI. Set them as environment variables:

```bash
export TWINE_PASSWORD_TESTPYPI=your_testpypi_token
export TWINE_PASSWORD_PYPI=your_pypi_token
```

Then you can publish:

```bash
# Test publish to TestPyPI first
make publish-test

# If everything looks good, publish to PyPI
make publish
```

You can verify the test package by installing from TestPyPI:
```bash
uv pip install --index-url https://test.pypi.org/simple/ quantrs
```

This will:
- Create a virtual environment
- Install all development dependencies
- Install the package in development mode
- Configure the environment for your IDE

For VS Code users, install the Python extension and select the interpreter from .venv/bin/python.

For PyCharm users, set the Python interpreter to .venv/bin/python in the project settings.

## Code Quality

Format and lint code:
```bash
# Format
ruff format .

# Lint and auto-fix
ruff check --fix .

# Type check
mypy .
```

## Testing

Run the tests:

```bash
pytest
```

## License

This project is licensed under the MIT License. See the LICENSE file for details. 