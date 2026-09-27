# Building, testing, running

## Prerequisites

- **Rust** ≥ 1.85 (edition 2024). Install via <https://rustup.rs>.
- That is all you need for the current crates (`okb-engine`, `okb-cli`) — they are
  std-only.

Later phases add system libraries (documented when they land):

- Phase 2 (IBus): `libibus-1.0-dev`.
- Phase 3 (Fcitx5): Fcitx5 dev packages, a C++ toolchain, CMake.
- Phase 4 (UI): GTK4 dev packages (`libgtk-4-dev`).

## Common commands

```bash
# Build everything
cargo build --workspace

# Run the test suite (unit + doctests)
cargo test --workspace

# Lint (CI treats warnings as errors)
cargo clippy --workspace --all-targets -- -D warnings

# Check formatting
cargo fmt --all --check

# Supply-chain audit (needs: cargo install cargo-deny)
cargo deny check
```

## Try the transliterator

```bash
# From arguments
cargo run -p okb-cli -- amar sonar bangla
#  => আমার সোনার বাংলা

# From stdin (composes in pipelines)
echo "ami bangla likhi" | cargo run -q -p okb-cli
#  => আমি বাংলা লিখি

# Help
cargo run -p okb-cli -- --help
```

Install the `okb` binary locally:

```bash
cargo install --path crates/cli
okb bhalo aachi
```

See [the phonetic scheme](../design/phonetic-scheme.md) for what to type.

## Repository layout

```text
open-keyboard/
├── Cargo.toml            # workspace
├── crates/
│   ├── engine/           # okb-engine  — pure transliteration core (std-only)
│   ├── ime/              # okb-ime     — input-session state machine (std-only)
│   ├── cli/              # okb-cli     — dev/test harness (binary: okb)
│   └── ibus/             # okb-ibus    — IBus D-Bus engine (binary: okb-ibus-engine)
├── packaging/            # IBus component + install/uninstall scripts
├── docs/                 # all documentation (source of truth)
├── .github/              # CI, CodeQL, security-audit, Dependabot
├── Makefile              # convenience wrapper (build/test/lint/install)
├── deny.toml             # cargo-deny config
├── rustfmt.toml
├── SECURITY.md
└── CONTRIBUTING.md
```
