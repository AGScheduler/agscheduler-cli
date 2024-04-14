SHELL=/bin/bash

.PHONY: install
install:
	rustup component add rustfmt
	rustup component add clippy
	rustup component add llvm-tools
	cargo install llvm-cov

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
	cargo llvm-cov
	cargo llvm-cov report --lcov --output-path lcov.info
	cargo llvm-cov report --html

.PHONY: check-all
check-all: format-check lint test

.PHONY: build
build:
	cargo build --release
