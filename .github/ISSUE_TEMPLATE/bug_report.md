---
name: Bug report
about: Report incorrect output or a problem with the input method
title: "[bug] "
labels: bug
---

<!--
  Security issues: do NOT file here. Use private reporting (see SECURITY.md).
-->

## What happened

A clear description of the problem.

## Steps to reproduce

1. Input method / app: …
2. Latin text I typed: `…`
3. Bengali I expected: `…`
4. Bengali I actually got: `…`

## Does it reproduce in the CLI?

Run and paste the output:

```bash
cargo run -p okb-cli -- <the text you typed>
```

- [ ] Reproduces in the CLI (→ engine/scheme bug)
- [ ] Only happens while typing in apps (→ transport/IBus integration)

## Environment

- OS / distro + version:
- Desktop environment (GNOME / KDE / …):
- Session type: Wayland / X11
- IBus version (`ibus version`):
- open-keyboard version / commit:

## Additional context

Logs (from running `okb-ibus-engine` by hand — see docs/development/ibus-setup.md),
screenshots, anything else.
