# ADR-0002: Target both IBus and Fcitx5 on Linux

- **Status:** Accepted
- **Date:** 2026-09-27

## Context

On Linux, an IME does not talk to applications directly; it plugs into an
input-method framework. The two relevant ones are:

- **IBus** — the default on Ubuntu/GNOME, works out of the box, tightly integrated
  but less extensible.
- **Fcitx5** — modern, lower-latency, more extensible (addons, Lua), preferred by
  many power users; needs manual setup on Ubuntu.

We could target one or both.

## Decision

Target **both IBus and Fcitx5**, via separate thin adapter crates over a shared,
framework-agnostic core (`okb-engine`).

Delivery order: **IBus first** (it is Ubuntu's default, so it is the fastest path
to "type Bengali on a stock Ubuntu install"), then **Fcitx5**.

## Consequences

- The engine must expose a clean, framework-neutral API (it does: the `Scheme`
  trait). Adapters own framework-specific concerns (key events, preedit, commit).
- Fcitx5's addon API is C++, so its adapter needs a C-ABI FFI shim over the Rust
  engine (see [ADR-0004](ADR-0004-language-split.md)); IBus can be driven from Rust
  over D-Bus.
- Two integration surfaces to test and package. Mitigated by keeping all logic in
  the shared core so adapters stay thin.
- Wayland vs X11 differences are handled by the frameworks, not by us.

## References

- IBus — <https://github.com/ibus/ibus>
- Fcitx5 — <https://github.com/fcitx/fcitx5>
- Fcitx5 develop-an-input-method docs — <https://fcitx-im.org/wiki/Develop_an_Input_Method>
