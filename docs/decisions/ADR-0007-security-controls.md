# ADR-0007: Baseline security controls

- **Status:** Accepted
- **Date:** 2026-09-27

## Context

As a keystroke-path component, open-keyboard needs security controls in place from
day one, not retrofitted. The founding requirement called for GitHub security
advisories and status badges.

## Decision

Enable and maintain this baseline:

### Enforced in the repository (code)

- **`#![forbid(unsafe_code)]`** in all pure-Rust crates.
- **Committed `Cargo.lock`** and a **minimal dependency tree**
  ([ADR-0005](ADR-0005-dependency-policy.md)).
- **CI quality gates:** `cargo fmt --check`, `cargo clippy` with `-D warnings`,
  `cargo test` ([`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)).
- **CodeQL** static analysis ([`.github/workflows/codeql.yml`](../../.github/workflows/codeql.yml)).
  Code-scanning uploads require a **public repository or GitHub Advanced Security**;
  the workflow is gated on repo visibility so it runs (and turns green) automatically
  once the repo is public or GHAS is enabled, and is skipped meanwhile rather than
  failing CI. Until then, `cargo-deny`, clippy, and tests provide the automated gates.
- **cargo-deny** for advisories/licenses/sources/bans, on push, PR, and daily
  schedule ([`.github/workflows/security-audit.yml`](../../.github/workflows/security-audit.yml)).
- **Dependabot** for cargo and github-actions ([`.github/dependabot.yml`](../../.github/dependabot.yml)).
- **Least-privilege CI:** every workflow sets minimal `permissions:`.

### Enabled in GitHub settings (one-time, by the maintainer)

These are repository *settings* that cannot be committed as files and must be
toggled once in the GitHub UI (**Settings → Code security and analysis**), tracked
in the [roadmap](../roadmap.md) Phase 0 checklist:

- [ ] **Private vulnerability reporting** (enables the Security tab report flow used
      by [SECURITY.md](../../SECURITY.md)).
- [ ] **Dependabot alerts** and **security updates**.
- [ ] **Secret scanning** and **push protection**.
- [ ] **Code scanning** (surfaces CodeQL results).
- [ ] Branch protection on `main` requiring CI to pass.

## Consequences

- Contributors face stricter gates (all checks must pass); this is intentional.
- Badges in the [README](../../README.md) reflect these controls so their status is
  visible at a glance.
- New attack surface (e.g. the C++ Fcitx5 shim) must extend this baseline —
  CodeQL `cpp` and sanitizer builds are added when that code lands.
