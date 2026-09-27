# How we run this project (no issues, no feature branches)

By decision, open-keyboard is managed **from the README and the `docs/` tree**, not
through GitHub issues or long-lived feature branches.

## What this means

- **Planning & tracking:** the [roadmap](../roadmap.md) is the task list. Update its
  checkboxes as work progresses.
- **Decisions:** significant choices are [ADRs](../decisions/README.md), not issue
  threads. ADRs are immutable; supersede rather than edit.
- **Work happens on `main`** in small, coherent commits. Keep `main` green: every
  commit should build and pass tests/lints.
- **No issue tracker** for planned work. The one exception is **security**:
  vulnerabilities are reported **privately** via GitHub Security Advisories, never as
  public issues (see [SECURITY.md](../../SECURITY.md)).

## Commit conventions

- Small, focused commits with clear messages; imperative mood
  ("Add anusvara handling", not "added").
- Optional prefixes to match the CI/Dependabot style: `deps:`, `ci:`, `docs:`.
- When a commit adds a dependency, it must also update the register in
  [ADR-0005](../decisions/ADR-0005-dependency-policy.md).
- When a commit changes the phonetic scheme, update
  [design/phonetic-scheme.md](../design/phonetic-scheme.md) and add a test in the
  same commit.

## Definition of done for a change

- [ ] `cargo build --workspace` passes.
- [ ] `cargo test --workspace` passes (new behaviour has tests).
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` is clean.
- [ ] `cargo fmt --all --check` is clean.
- [ ] Relevant docs updated (roadmap checkbox, scheme table, ADR register).
- [ ] No new `unsafe` outside a documented FFI shim; no new dependency without an
      ADR update.

## Releases

Pre-1.0: tag from `main` when a phase completes. Release artifacts are built in CI.
Signing and reproducible builds are tracked in the roadmap.
