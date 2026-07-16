.PHONY: help build test check lint fmt clean

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the project (debug)
	cargo build

release: ## Build the project (release)
	cargo build --release

test: ## Run all tests
	cargo test --all-features

check: ## Check compilation (no tests)
	cargo check --all-targets --all-features

lint: ## Run clippy lints
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format code
	cargo fmt

fmt-check: ## Check formatting
	cargo fmt --all -- --check

clean: ## Clean build artifacts
	cargo clean

ci: fmt-check check lint test ## Full CI pipeline
