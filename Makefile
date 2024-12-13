ex:
	cargo run -p quantrs_examples --bin $(name)

test:
ifeq ($(name),)
	@echo "Running all tests"
	cargo test -p quantrs_examples
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

clean:
	cargo clean
