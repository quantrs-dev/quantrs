#!/bin/bash

# Create virtual environment
uv venv .venv

# Activate virtual environment
source .venv/bin/activate

# Install dev dependencies
uv pip install -r requirements-dev.txt

# Install package in development mode
maturin develop

# Create .env file for IDE
echo "PYTHONPATH=${PWD}/python" >.env
