# Convenience wrapper around cargo + packaging scripts.
# Everything here also works by invoking cargo directly; see docs/development/build.md.

.PHONY: all build release test lint fmt fmt-check clippy audit demo install uninstall clean

all: build

build:
	cargo build --workspace

release:
	cargo build --workspace --release

test:
	cargo test --workspace

lint: fmt-check clippy

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

# Requires: cargo install cargo-deny
audit:
	cargo deny check

# Quick demo of the transliteration engine.
demo:
	cargo run -q -p okb-cli -- amar sonar bangla

# Build and install the IBus engine (uses sudo for system paths).
install:
	packaging/install.sh

uninstall:
	packaging/uninstall.sh

clean:
	cargo clean
