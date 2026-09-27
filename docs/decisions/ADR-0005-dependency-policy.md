# ADR-0005: Minimal-dependency policy; std-only core

- **Status:** Accepted
- **Date:** 2026-09-27

## Context

open-keyboard runs in the keystroke path, so every dependency is code that can see
everything the user types. A large transitive dependency tree is both a
supply-chain risk and an audit burden. The founding requirement was explicit:
*"as least third party library as possible."*

## Decision

Adopt a strict minimal-dependency policy:

1. **`okb-engine` (the core) has zero third-party dependencies** — `std` only. It
   performs no I/O (no network, no filesystem, no logging of buffer contents).
2. **Any dependency, anywhere in the workspace, requires justification** recorded
   by updating this ADR's register below. Prefer `std`; prefer one well-maintained
   crate over several; prefer crates with a clean advisory history.
3. **`Cargo.lock` is committed** — the exact tree is pinned and reviewable.
4. **CI enforces** the tree with `cargo-deny` (advisories, licenses, sources, bans)
   and Dependabot keeps it patched.
5. Unavoidable heavy trees (e.g. `gtk4` for the UI) are **quarantined** in the crate
   that needs them and never leak into the engine or adapters.

## Dependency register

| Crate | Dependency | Justification |
| ----- | ---------- | ------------- |
| `okb-engine` | *(none)* | Std-only by policy. |
| `okb-ime` | `okb-engine` (workspace) | Internal; no external deps. |
| `okb-cli` | `okb-engine` (workspace) | Internal; no external deps. |
| `okb-ibus` | `okb-ime` (workspace), `zbus` | `zbus` is the D-Bus transport for the IBus protocol. It is **pure Rust** — chosen over libibus/GObject bindings specifically to avoid C FFI and keep `#![forbid(unsafe_code)]` intact in the keystroke path. It is quarantined to this crate; the engine and session stay dependency-free. |
| `okb-ui` (planned) | `gtk4` (gtk-rs) | Required for the native UI ([ADR-0003](ADR-0003-ui-stack-gtk4.md)); quarantined to this crate. |

Update this table in the same change that adds a dependency.

## Consequences

- Slower feature velocity in exchange for a small, auditable trusted base — an
  acceptable trade for an input method.
- Some wheels get reinvented in the core (e.g. the transliteration tables). That is
  intentional and keeps the trusted base tiny.
