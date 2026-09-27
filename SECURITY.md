# Security Policy

open-keyboard is an input method: it sits in the keystroke path of everything a
user types, including passwords and private messages. We therefore treat security
as a first-class requirement, not an afterthought. See
[`docs/architecture/security.md`](docs/architecture/security.md) for the threat
model.

## Reporting a vulnerability

**Please do not open a public issue for security problems.**

Report privately through GitHub's **Private Vulnerability Reporting**:

1. Go to the repository's **Security** tab → **Report a vulnerability**.
2. Describe the issue, affected version/commit, and reproduction steps.

If private reporting is unavailable to you, contact the maintainer directly and
we will open a draft advisory on your behalf.

We aim to acknowledge reports within **72 hours** and to publish a fix and a
[GitHub Security Advisory](https://github.com/maifeeulasad/open-keyboard/security/advisories)
once a patch is available. Reporters are credited unless they prefer to remain
anonymous.

## Repository security controls

These are enabled/maintained for this project (see
[`docs/decisions/ADR-0007-security-controls.md`](docs/decisions/ADR-0007-security-controls.md)):

- **Private vulnerability reporting** (GitHub Security tab).
- **Dependabot** alerts and version updates ([`.github/dependabot.yml`](.github/dependabot.yml)).
- **CodeQL** static analysis ([`.github/workflows/codeql.yml`](.github/workflows/codeql.yml)).
- **cargo-deny** for advisories, licenses, sources, and bans
  ([`.github/workflows/security-audit.yml`](.github/workflows/security-audit.yml)).
- **`#![forbid(unsafe_code)]`** across all pure-Rust crates; every future `unsafe`
  block (FFI only) must be documented and justified.
- **Pinned `Cargo.lock`** and a minimal, audited dependency tree
  ([ADR-0005](docs/decisions/ADR-0005-dependency-policy.md)).

## Supported versions

The project is pre-1.0. Only the latest `main` receives security fixes until a
stable release line is established.

| Version | Supported |
| ------- | --------- |
| `main`  | ✅        |
| < 0.1   | ❌        |
