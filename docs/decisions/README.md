# Architecture Decision Records (ADRs)

An ADR captures a single significant decision: its context, the choice, and the
consequences. ADRs are **immutable once accepted** — to reverse one, write a new
ADR that supersedes it and update the status here.

Format: a lightweight [MADR](https://adr.github.io/madr/)-style template.

| # | Title | Status |
| - | ----- | ------ |
| [0001](ADR-0001-product-type-ime.md) | Build a system-wide Input Method (IME) | Accepted |
| [0002](ADR-0002-ime-frameworks.md) | Target both IBus and Fcitx5 on Linux | Accepted |
| [0003](ADR-0003-ui-stack-gtk4.md) | Native Rust UI with GTK4 (gtk-rs) | Accepted |
| [0004](ADR-0004-language-split.md) | Rust-first; C/C++ only at FFI boundaries | Accepted |
| [0005](ADR-0005-dependency-policy.md) | Minimal-dependency policy; std-only core | Accepted |
| [0006](ADR-0006-clean-room-phonetic-scheme.md) | Clean-room scheme; no Avro/OpenBangla code or data | Accepted |
| [0007](ADR-0007-security-controls.md) | Baseline security controls | Accepted |

All seven were accepted at project inception (2026-09-27) based on the founding
requirements.
