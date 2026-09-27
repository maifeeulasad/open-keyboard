# ADR-0004: Rust-first; C/C++ only at FFI boundaries

- **Status:** Accepted
- **Date:** 2026-09-27

## Context

The founding requirement named Rust, C, and C++ for performance. All three are
fast; the question is how to divide responsibility to get performance *and* memory
safety *and* maintainability.

## Decision

**Rust-first.** All application logic, the engine, the adapters, and the UI are
written in safe Rust. C or C++ is introduced **only** where an external framework's
ABI requires it — concretely, the **Fcitx5 addon API is C++**, so we will write a
minimal C++ shim that calls into the Rust engine through a C ABI (`okb-ffi`).

## Rationale

- Rust gives us C/C++-class performance without their memory-safety footguns —
  decisive for a component in the keystroke path (see
  [security model](../architecture/security.md)).
- Confining C/C++ to a thin, well-audited shim keeps the `unsafe` surface tiny and
  reviewable.

## Consequences

- `#![forbid(unsafe_code)]` holds for every pure-Rust crate. Only FFI crates relax
  it, and each `unsafe` block must be documented and justified.
- IBus integration can be pure Rust (D-Bus), so it needs no C/C++.
- The C ABI layer (`okb-ffi`) is added only when the Fcitx5 adapter work begins
  (Phase 3), not before — we do not write FFI we do not yet need.
