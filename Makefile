
# Program details
PROGRAM_NAME = spl-stake-pool
PROGRAM_ID = SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
PROGRAM_KEYPAIR = .keys/$(PROGRAM_ID).json
PROGRAM_SO = target/deploy/spl_stake_pool.so

# Solana CLI defaults
CLUSTER ?= localnet
FOGO_URL_localnet = http://localhost:8899
FOGO_URL_testnet = https://testnet.fogo.io
FOGO_URL_mainnet = https://mainnet.fogo.io

.DEFAULT_GOAL: help

RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
NC := \033[0m # No Color

# ----------------------------------------------------------------------------------------------------------------------

help: ## Show this help
	@printf "$(YELLOW)%s:$(NC)\n" 'Available commands'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z\/_-]+:.*?## / {printf "  $(GREEN)%-18s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# ----------------------------------------------------------------------------------------------------------------------

build: ## Build stake pool program
	@echo "Building stake pool program..."
	cargo build-sbf -- -p spl-stake-pool

build/cli: ## Build CLI binary
	@echo "Building cli..."
	cargo build --bin spl-stake-pool --release

test/unit: ## Run unit tests
	@echo "Running unit tests..."
	cargo test --lib

test/integration: ## Run integration tests
	@echo "Running integration tests..."
	cargo test -p spl-stake-pool

test: test/unit test/integration ## Run all tests
	@echo "All tests completed!"

fmt: ## Format code
	@echo "Formatting code..."
	cargo +nightly fmt --all

lint: ## Run clippy linter
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

check: ## Check code compilation
	@echo "Running cargo check..."
	cargo check --all-targets --all-features

clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf target/

deploy/localnet: build ## Deploy to localnet
	@echo "Deploying to localnet..."
	@if [ ! -f "$(PROGRAM_KEYPAIR)" ]; then \
		echo "Error: Program keypair not found at $(PROGRAM_KEYPAIR)"; \
		exit 1; \
	fi
	@echo "Using program keypair: $(PROGRAM_KEYPAIR)"
	@echo "Program ID: $(PROGRAM_ID)"
	solana program deploy \
		--url $(FOGO_URL_localnet) \
		--program-id $(PROGRAM_KEYPAIR) \
		$(PROGRAM_SO)
	@echo "Deployment complete! Program ID: $(PROGRAM_ID)"

deploy/testnet: build ## Deploy to testnet
	@echo "Deploying to testnet..."
	@if [ ! -f "$(PROGRAM_KEYPAIR)" ]; then \
		echo "Error: Program keypair not found at $(PROGRAM_KEYPAIR)"; \
		exit 1; \
	fi
	@echo "WARNING: Deploying to testnet. Make sure you have enough SOL for deployment."
	@echo "Using program keypair: $(PROGRAM_KEYPAIR)"
	@echo "Program ID: $(PROGRAM_ID)"
	solana program deploy \
		--url $(FOGO_URL_testnet) \
		--program-id $(PROGRAM_KEYPAIR) \
		$(PROGRAM_SO)
	@echo "Deployment complete! Program ID: $(PROGRAM_ID)"

deploy/mainnet: build ## Deploy to mainnet
	@echo "WARNING: You are about to deploy to MAINNET!"
	@echo "Program ID: $(PROGRAM_ID)"
	@read -p "Are you sure you want to continue? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		if [ ! -f "$(PROGRAM_KEYPAIR)" ]; then \
			echo "Error: Program keypair not found at $(PROGRAM_KEYPAIR)"; \
			exit 1; \
		fi; \
		echo "Deploying to mainnet..."; \
		solana program deploy \
			--url $(FOGO_URL_mainnet) \
			--program-id $(PROGRAM_KEYPAIR) \
			$(PROGRAM_SO); \
		echo "Deployment complete! Program ID: $(PROGRAM_ID)"; \
	else \
		echo "Deployment cancelled."; \
		exit 1; \
	fi

upgrade: build ## Upgrade program on specified cluster
	@echo "Upgrading program on $(CLUSTER)..."
	@if [ ! -f "$(PROGRAM_KEYPAIR)" ]; then \
		echo "Error: Program keypair not found at $(PROGRAM_KEYPAIR)"; \
		exit 1; \
	fi
	solana program deploy \
		--url $(FOGO_URL_$(CLUSTER)) \
		--program-id $(PROGRAM_ID) \
		--upgrade-authority $(PROGRAM_KEYPAIR) \
		$(PROGRAM_SO)
	@echo "Upgrade complete!"

verify-build: build ## Verify program build
	@echo "Verifying program build..."
	@if [ -f "$(PROGRAM_SO)" ]; then \
		echo "Program binary: $(PROGRAM_SO)"; \
		ls -lh $(PROGRAM_SO); \
		echo ""; \
		solana program show $(PROGRAM_ID) --url $(FOGO_URL_$(CLUSTER)) || echo "Program not deployed yet"; \
	else \
		echo "Error: Program binary not found at $(PROGRAM_SO)"; \
		exit 1; \
	fi
