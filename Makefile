.PHONY: help build-bpf build-sbpf-linker build test test-e2e test-all clean fmt lint check deploy deploy-localnet deploy-devnet deploy-mainnet upgrade verify-build

# Program details
PROGRAM_NAME = spl-stake-pool
PROGRAM_ID = SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
PROGRAM_KEYPAIR = .keys/$(PROGRAM_ID).json
PROGRAM_SO = target/deploy/spl_stake_pool.so

# Fogo CLI defaults
CLUSTER ?= localnet
FOGO_URL_localnet = http://localhost:8899
FOGO_URL_testnet = https://testnet.fogo.io
FOGO_URL_mainnet = https://mainnet.fogo.io

help:
	@echo "SPL Stake Pool"
	@echo ""
	@echo "Build Targets:"
	@echo "  make build         	- Build stake-pool program"
	@echo "  make build/cli         - Build CLI binary"
	@echo ""
	@echo "Test Targets:"
	@echo "  make test              - Run unit tests"
	@echo "  make test-e2e          - Run E2E integration tests"
	@echo "  make test-all          - Run all tests (unit + E2E)"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt               - Format code with rustfmt"
	@echo "  make lint              - Run clippy linter"
	@echo "  make check             - Run cargo check"
	@echo ""
	@echo "Deployment:"
	@echo "  make deploy            - Deploy to localnet (default)"
	@echo "  make deploy/localnet   - Deploy to localnet"
	@echo "  make deploy/testnet    - Deploy to testnet"
	@echo "  make deploy/mainnet    - Deploy to mainnet"
	@echo "  make upgrade           - Upgrade existing program"
	@echo "  make verify-build      - Verify program build"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean             - Clean build artifacts"
	@echo ""

build:
	@echo "Building stake pool program..."
	cargo build-sbf -- -p spl-stake-pool

build/cli:
	@echo "Building cli..."
	cargo build --bin spl-stake-pool --release

test:
	@echo "Running unit tests..."
	cargo test --lib

test-e2e:
	@echo "Running E2E integration tests..."
	cargo test -p spl-stake-pool

test-all: test test-e2e
	@echo "All tests completed!"

fmt:
	@echo "Formatting code..."
	cargo +nightly fmt --all

lint:
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

check:
	@echo "Running cargo check..."
	cargo check --all-targets --all-features

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf target/

deploy: deploy/localnet

deploy/localnet: build
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

deploy/testnet: build
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

deploy/mainnet: build
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

upgrade: build
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

verify-build: build
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