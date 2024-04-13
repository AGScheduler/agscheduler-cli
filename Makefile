SHELL=/bin/bash

.PHONY: install
install:
	rustup component add clippy
	cargo install cargo-llvm-cov

.PHONY: format
format:
	cargo fmt

.PHONY: format-check
format-check:
	cargo fmt --check

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: test
test:
	cargo llvm-cov --all-features
	cargo llvm-cov report --html

.PHONY: check-all
check-all: format-check lint test
