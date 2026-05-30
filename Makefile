WASM_TARGET := wasm32-unknown-unknown
WASM_OUT    := target/$(WASM_TARGET)/release/anchorkit.wasm

.PHONY: build test wasm lint fmt fmt-check check

build:
	cargo build --release

test:
	cargo test

wasm:
	cargo build --release --target $(WASM_TARGET) --no-default-features --features wasm
	@ls -lh $(WASM_OUT)

# Formatting
fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

# Linting
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Combined check (run before committing)
check: fmt-check lint test
	@echo "✓ All checks passed!"
