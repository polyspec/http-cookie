CARGO := cargo
RUST := packages/rust

.PHONY: check test lint-rust fmt-check

check: lint-rust fmt-check test

test:
	$(CARGO) test --manifest-path $(RUST)/Cargo.toml --locked

lint-rust:
	$(CARGO) clippy --manifest-path $(RUST)/Cargo.toml --locked --all-targets -- -D warnings

fmt-check:
	$(CARGO) fmt --manifest-path $(RUST)/Cargo.toml --check
