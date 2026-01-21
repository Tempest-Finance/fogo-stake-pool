# ══════════════════════════════════════════════════════════════════════════════
# Fogo Stake Pool
# ══════════════════════════════════════════════════════════════════════════════

PROGRAM_ID := SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
KEYPAIR    := .keys/$(PROGRAM_ID).json
PROGRAM_SO := target/deploy/spl_stake_pool.so

RUST_TOOLCHAIN_NIGHTLY := nightly-2025-02-16
SOLANA_CLI_VERSION := 2.3.4

nightly := +$(RUST_TOOLCHAIN_NIGHTLY)

CLUSTER ?= mainnet
RPC_localnet := http://localhost:8899
RPC_testnet  := https://testnet.fogo.io
RPC_mainnet  := https://mainnet.fogo.io

# Path helpers (converts clients-js -> clients/js)
pattern-dir = $(firstword $(subst -, ,$1))
find-pattern-dir = $(findstring $(call pattern-dir,$1)-,$1)
make-path = $(subst $(call find-pattern-dir,$1),$(subst -,/,$(call find-pattern-dir,$1)),$1)

# Publish helpers
tag-name = $(lastword $(subst /, ,$(call make-path,$1)))
crate-version = $(subst ",,$(shell toml get $(call make-path,$1)/Cargo.toml package.version))

.DEFAULT_GOAL := help

# ══════════════════════════════════════════════════════════════════════════════
# CI Targets
# ══════════════════════════════════════════════════════════════════════════════

rust-toolchain-nightly:
	@echo $(RUST_TOOLCHAIN_NIGHTLY)

solana-cli-version:
	@echo $(SOLANA_CLI_VERSION)

# SBF build targets (called with directory paths like program)
build-sbf-%:
	RUSTFLAGS="--allow=unexpected_cfgs" cargo build-sbf --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

# JS CI targets (called with path patterns like clients-js)
build-js-%:
	cd $(call make-path,$*) && pnpm install && pnpm build $(ARGS)

format-js-%:
	cd $(call make-path,$*) && pnpm install && pnpm format:fix $(ARGS)

format-check-js-%:
	cd $(call make-path,$*) && pnpm install && pnpm format $(ARGS)

lint-js-%:
	cd $(call make-path,$*) && pnpm install && pnpm lint $(ARGS)

test-js-%:
	cd $(call make-path,$*) && pnpm install && pnpm build && pnpm test $(ARGS)

# Python CI targets (called with path patterns like clients-py)
setup-py-venv-%:
	cd $(call make-path,$*) && python3 -m venv venv \
		&& ./venv/bin/pip3 install -r requirements.txt \
		&& ./venv/bin/pip3 install -r optional-requirements.txt

format-py-%:
	$(MAKE) setup-py-venv-$*
	cd $(call make-path,$*) && ./venv/bin/black .

format-check-py-%:
	$(MAKE) setup-py-venv-$*
	cd $(call make-path,$*) && ./venv/bin/flake8 --exclude venv

lint-py-%:
	$(MAKE) setup-py-venv-$*
	cd $(call make-path,$*) && ./venv/bin/mypy --exclude venv .

test-py-%:
	$(MAKE) setup-py-venv-$*
	cd $(call make-path,$*) && ./venv/bin/python3 -m pytest

# Rust CI targets (called with path patterns like interface, clients-rust)
format-check-%:
	cargo $(nightly) fmt --check --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

clippy-%:
	RUSTFLAGS="--allow=unexpected_cfgs" cargo $(nightly) clippy --manifest-path $(call make-path,$*)/Cargo.toml --all-targets -- -D warnings $(ARGS)

test-%:
	RUSTFLAGS="--allow=unexpected_cfgs" SBF_OUT_DIR=$(CURDIR)/target/deploy cargo $(nightly) test --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

build-doc-%:
	cargo doc --manifest-path $(call make-path,$*)/Cargo.toml --no-deps $(ARGS)

generate-clients:
	pnpm generate:clients $(ARGS)

generate-idl-%:
	cargo run --manifest-path $(call make-path,$*)/Cargo.toml --features codama --bin generate-idl $(ARGS)

powerset-%:
	@echo "Powerset testing not configured for $*"

# Rust publish targets (called with path patterns like interface, clients-rust)
git-tag-rust-%:
	@echo "$(call tag-name,$*)@v$(call crate-version,$*)"

publish-rust-%:
	cd "$(call make-path,$*)" && cargo release $(LEVEL) --tag-name "$(call tag-name,$*)@v{{version}}" --execute --no-confirm --dependent-version fix

publish-rust-dry-run-%:
	cd "$(call make-path,$*)" && cargo release $(LEVEL) --tag-name "$(call tag-name,$*)@v{{version}}"

# ══════════════════════════════════════════════════════════════════════════════
# Build
# ══════════════════════════════════════════════════════════════════════════════

build: ## Build on-chain program
	@RUSTFLAGS="--allow=unexpected_cfgs" cargo build-sbf -- -p spl-stake-pool

build/cli: ## Build CLI binary
	cargo build --release -p fogo-stake-pool-cli

build/js: ## Build JS client
	pnpm -C clients/js build

# ══════════════════════════════════════════════════════════════════════════════
# Test
# ══════════════════════════════════════════════════════════════════════════════

test: build ## Run all tests
	SBF_OUT_DIR=$(CURDIR)/target/deploy cargo $(nightly) test -p spl-stake-pool

test/unit: ## Run unit tests only
	cargo test --lib -p spl-stake-pool

test/int: build ## Run integration tests only
	SBF_OUT_DIR=$(CURDIR)/target/deploy cargo $(nightly) test -p spl-stake-pool --test '*'

# ══════════════════════════════════════════════════════════════════════════════
# Code Quality
# ══════════════════════════════════════════════════════════════════════════════

fmt: ## Format Rust code
	cargo $(nightly) fmt --all

fmt/check: ## Check Rust formatting
	cargo $(nightly) fmt --all --check

lint: ## Run clippy
	cargo $(nightly) clippy --all-targets -- -D warnings

audit: ## Run security audit
	cargo audit --ignore RUSTSEC-2022-0093 --ignore RUSTSEC-2024-0344 --ignore RUSTSEC-2025-0055

spellcheck: ## Run spellcheck
	@echo "Spellcheck skipped - not configured"

# ══════════════════════════════════════════════════════════════════════════════
# Deploy
# ══════════════════════════════════════════════════════════════════════════════

deploy: build _check-keypair _check-cluster ## Deploy program (CLUSTER=localnet|testnet|mainnet)
ifeq ($(CLUSTER),mainnet)
	@printf "\033[31mDeploy to MAINNET? [y/N] \033[0m" && read ans && [ $${ans:-N} = y ]
endif
	solana program deploy --url $(RPC_$(CLUSTER)) --program-id $(KEYPAIR) $(PROGRAM_SO)

deploy/js: build/js ## Publish JS client to npm
	pnpm publish -F {./clients/js} --access public --no-git-checks

# ══════════════════════════════════════════════════════════════════════════════
# Utilities
# ══════════════════════════════════════════════════════════════════════════════

help: ## Show this help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z\/_-]+:.*?## / {printf "  \033[32m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

clean: ## Remove build artifacts
	cargo clean
	rm -rf clients/js/dist

size: build ## Show program size
	@ls -lh $(PROGRAM_SO) | awk '{printf "Program size: %s\n", $$5}'

show: ## Show program info on cluster
	@solana program show $(PROGRAM_ID) --url $(RPC_$(CLUSTER)) 2>/dev/null || echo "Program not deployed on $(CLUSTER)"

_check-keypair:
	@test -f $(KEYPAIR) || (echo "Error: $(KEYPAIR) not found" && exit 1)

_check-cluster:
	@test -n "$(RPC_$(CLUSTER))" || (echo "Error: Invalid CLUSTER '$(CLUSTER)'" && exit 1)
