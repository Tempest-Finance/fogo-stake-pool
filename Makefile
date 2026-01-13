# ══════════════════════════════════════════════════════════════════════════════
# Fogo Stake Pool
# ══════════════════════════════════════════════════════════════════════════════

PROGRAM_ID := SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
KEYPAIR    := .keys/$(PROGRAM_ID).json
PROGRAM_SO := target/deploy/spl_stake_pool.so

# From workspace.metadata.toolchains
NIGHTLY := +nightly-2025-02-16

# Cluster configuration
CLUSTER ?= testnet
RPC_localnet := http://localhost:8899
RPC_testnet  := https://testnet.fogo.io
RPC_mainnet  := https://mainnet.fogo.io

# Colors
C_CYAN   := \033[36m
C_GREEN  := \033[32m
C_YELLOW := \033[33m
C_RED    := \033[31m
C_RESET  := \033[0m

# ══════════════════════════════════════════════════════════════════════════════

.DEFAULT_GOAL := help

help: ## Show this help
	@printf "$(YELLOW)%s:$(NC)\n" 'Available commands'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z\/_-]+:.*?## / {printf "  $(C_GREEN)%-18s$(C_RESET) %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# ══════════════════════════════════════════════════════════════════════════════
# Build
# ══════════════════════════════════════════════════════════════════════════════

# unexpected_cfgs lint: Rust 1.80+ introduced stricter checking of #[cfg(...)] attributes.
# The solana_program::entrypoint! macro internally uses cfg(feature = "custom-heap") and cfg(feature = "custom-panic")

build: ## Build on-chain program
	@RUSTFLAGS="--allow=unexpected_cfgs" cargo build-sbf -- -p spl-stake-pool

build/cli: ## Build CLI binary
	cargo build --release -p fogo-stake-pool-cli

build/js: ## Build TypeScript client
	pnpm -C clients/js build

# ══════════════════════════════════════════════════════════════════════════════
# Test
# ══════════════════════════════════════════════════════════════════════════════

test: build ## Run all program tests
	SBF_OUT_DIR=$(CURDIR)/target/deploy cargo $(NIGHTLY) test -p spl-stake-pool

test/unit: ## Run unit tests only (fast, no BPF)
	cargo test --lib -p spl-stake-pool

test/int: build ## Run integration tests only
	SBF_OUT_DIR=$(CURDIR)/target/deploy cargo $(NIGHTLY) test -p spl-stake-pool --test '*'

# ══════════════════════════════════════════════════════════════════════════════
# Code Quality
# ══════════════════════════════════════════════════════════════════════════════

fmt: ## Format code (uses nightly)
	cargo $(NIGHTLY) fmt --all

fmt/check: ## Check formatting without changes
	cargo $(NIGHTLY) fmt --all --check

lint: ## Run clippy linter
	cargo $(NIGHTLY) clippy --all-targets -- -D warnings

check: ## Fast type-check
	cargo check --all-targets

audit: ## Run security audit
	cargo audit --ignore RUSTSEC-2022-0093 --ignore RUSTSEC-2024-0344

ci: fmt/check lint check test ## Full CI pipeline
	@printf "$(C_GREEN)✓ All checks passed$(C_RESET)\n"

# ══════════════════════════════════════════════════════════════════════════════
# Deploy
# ══════════════════════════════════════════════════════════════════════════════

deploy: build _check-keypair _check-cluster ## Deploy to cluster (CLUSTER=localnet|testnet|mainnet)
ifeq ($(CLUSTER),mainnet)
	@printf "\033[31mDeploy to MAINNET? [y/N] \033[0m" && read ans && [ $${ans:-N} = y ]
endif
	solana program deploy \
		--url $(RPC_$(CLUSTER)) \
		--program-id $(KEYPAIR) \
		$(PROGRAM_SO)
	@printf "$(C_GREEN)✓ Deployed to $(CLUSTER)$(C_RESET)\n"

deploy/js: build/js ## Publish JS client to npm
	pnpm publish -F {./clients/js} --access public --no-git-checks

show: ## Show program info on cluster
	@solana program show $(PROGRAM_ID) --url $(RPC_$(CLUSTER)) 2>/dev/null \
		|| printf "$(C_YELLOW)Program not deployed on $(CLUSTER)$(C_RESET)\n"

# ══════════════════════════════════════════════════════════════════════════════
# Utilities
# ══════════════════════════════════════════════════════════════════════════════

clean: ## Remove build artifacts
	cargo clean
	rm -rf clients/js/dist

size: build ## Show program size
	@printf "Program: "
	@ls -lh $(PROGRAM_SO) | awk '{print $$5}'

_check-keypair:
	@test -f $(KEYPAIR) || (echo "Error: $(KEYPAIR) not found" && exit 1)

_check-cluster:
	@test -n "$(RPC_$(CLUSTER))" || (printf "$(C_RED)Error: Invalid CLUSTER '$(CLUSTER)'. Use: localnet, testnet, or mainnet$(C_RESET)\n" && exit 1)
