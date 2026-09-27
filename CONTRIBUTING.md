# Contributing to open-keyboard

Thanks for your interest! A few things make this project unusual — please read
before you start.

## Ground rules

1. **Clean-room provenance.** Do **not** copy source code, romanization rule tables,
   dictionaries, or other data from Avro Keyboard, OpenBangla Keyboard, or similar
   projects. You may study them as prior art. By opening a pull request you certify
   your contribution is your own work (or from a compatibly-licensed source you
   disclose). See [ADR-0006](docs/decisions/ADR-0006-clean-room-phonetic-scheme.md).
2. **Minimal dependencies.** Adding a crate requires updating the register in
   [ADR-0005](docs/decisions/ADR-0005-dependency-policy.md) with a justification. The
   core crate (`okb-engine`) stays std-only.
3. **No `unsafe`** outside a documented FFI shim. Pure-Rust crates
   `#![forbid(unsafe_code)]`.
4. **Security first.** Never add logging, network, or filesystem access to the
   engine. Report vulnerabilities privately (see [SECURITY.md](SECURITY.md)), not as
   public issues.

## Workflow

We run without an issue tracker or feature branches — see
[project management](docs/development/project-management.md). Work in small commits
on `main` (or a short-lived PR branch if you prefer review), keeping `main` green.

## Before you commit

Run all gates locally:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check      # cargo install cargo-deny
```

## When you change the phonetic scheme

- Update `PhoneticScheme::bengali` in `crates/engine/src/scheme.rs`.
- Add or update a test in `crates/engine/src/transliterate.rs`.
- Update [docs/design/phonetic-scheme.md](docs/design/phonetic-scheme.md) in the same
  commit.

## Commit messages

Imperative mood, focused scope, e.g. `Add r-phala handling` or `docs: clarify
conjunct rules`. Prefixes `deps:`, `ci:`, `docs:` are welcome.
