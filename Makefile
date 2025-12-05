.PHONY: build run test clean dev release help

help: ## Show this help
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the project
	cargo build

release: ## Build optimized release version
	cargo build --release

run: ## Run the application
	cargo run

dev: ## Run in development mode with verbose logging
	cargo run -- --env development --verbose

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with output
	cargo test -- --nocapture

check: ## Check code without building
	cargo check

fmt: ## Format code
	cargo fmt

lint: ## Run clippy linter
	cargo clippy -- -D warnings

clean: ## Clean build artifacts
	cargo clean
	rm -rf logs/*.log

doc: ## Generate documentation
	cargo doc --open

install: ## Install the binary
	cargo install --path .
