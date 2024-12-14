ex:
	cargo run -p quantrs_examples --bin $(name)

clean:
	cargo clean
	$(MAKE) clean.venv

fmt:
	cargo fmt -- --check

lint:
	cargo clippy

check:
	cargo check

test:
ifeq ($(name),)
	@echo "Running all tests"
	cargo test -p quantrs_examples
	cd quantrs-py && source .venv/bin/activate && pytest
else
	@echo "Running tests for $(name)"
	cargo test -p quantrs_examples --test $(name)
endif

spec:
ifeq ($(name),)
	@echo "Running all unit tests"
	cargo test -p quantrs
else
	@echo "Running unit tests for $(name)"
	cargo test -p quantrs --test $(name)
endif

build:
	cargo build	

ci: check fmt lint spec test build

# Rust package publishing
publish-test:
	cargo publish --dry-run -p quantrs

publish:
	cargo publish -p quantrs

# Python package publishing
publish-py-test: .venv
	@if [ -z "$$MATURIN_PASSWORD_TESTPYPI" ]; then \
		echo "Error: MATURIN_PASSWORD_TESTPYPI is not set"; \
		exit 1; \
	fi
	cd quantrs-py && source .venv/bin/activate && \
		maturin publish --repository-url https://test.pypi.org/legacy/ --username __token__

publish-py: .venv
	@if [ -z "$$MATURIN_PASSWORD_PYPI" ]; then \
		echo "Error: MATURIN_PASSWORD_PYPI is not set"; \
		exit 1; \
	fi
	cd quantrs-py && source .venv/bin/activate && maturin publish --username __token__

# Python virtual environment
.venv:
	@if [ ! -d "quantrs-py/.venv" ]; then \
		cargo install maturin; \
		cd quantrs-py && uv venv .venv; \
		cd quantrs-py && source .venv/bin/activate && uv pip install -r requirements-dev.txt; \
		cd quantrs-py && source .venv/bin/activate && maturin develop --uv; \
		cd quantrs-py && echo "PYTHONPATH=$${PWD}/python" >.env; \
	fi

# Python package building
build-py:
	cd quantrs-py && maturin build --release

dev-py:
	cd quantrs-py && maturin develop

install-py: build-py
	cd quantrs-py && uv pip install target/wheels/quantrs-*.whl

# Python tests
test-py: .venv
	rm -rf quantrs-py/target
	cd quantrs-py && source .venv/bin/activate && pytest

# Python cleaning
clean-venv:
	rm -rf quantrs-py/.venv
	rm -f quantrs-py/.env
	rm -rf quantrs-py/target
