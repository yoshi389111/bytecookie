.DEFAULT_GOAL = help
help: ## Show help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
.PHONY: help

build: ## Build
	cargo build --release
.PHONY: build

clean: ## Clean build artifacts
	rm -rf ./target
.PHONY: clean

test: ## Run tests
	cargo test
.PHONY: test

licenses: ## create a license-third-party.html file
	cargo about generate about.hbs > licenses-third-party.html
.PHONY: licenses
