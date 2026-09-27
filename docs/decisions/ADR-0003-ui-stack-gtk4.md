# ADR-0003: Native Rust UI with GTK4 (gtk-rs)

- **Status:** Accepted
- **Date:** 2026-09-27

## Context

The IME needs two pieces of UI:

1. A **preferences/settings window** (choose scheme, layout, behaviour).
2. A **candidate/suggestion popup** (word suggestions near the cursor).

Options considered: native GTK4 via [gtk-rs](https://gtk-rs.org/), a web UI via
Tauri, or native Qt/C++ (as OpenBangla uses).

## Decision

Use **native Rust with GTK4** (gtk-rs).

## Rationale

- Keeps the entire stack in Rust — no C++ UI layer, no browser runtime.
- Native GNOME/GTK look and feel on our first target, Ubuntu.
- Fast startup and low memory, which matters for an always-resident component.
- Fewer moving parts and a smaller dependency/attack surface than shipping a
  webview (aligns with [ADR-0005](ADR-0005-dependency-policy.md) and the
  [security model](../architecture/security.md)).

## Consequences

- gtk-rs is a substantial dependency tree; it is confined to the `okb-ui` crate and
  never pulled into `okb-engine` or the adapters. The core stays dependency-free.
- The candidate popup may need per-framework positioning glue (IBus and Fcitx5 both
  offer their own candidate UI; we will prefer the framework's native candidate
  window where it is good enough, and use our GTK popup only where needed).
- Cross-platform UI later (Windows/macOS) may require revisiting; GTK works there
  but is not idiomatic. That is a future ADR when those OSes are in scope.
