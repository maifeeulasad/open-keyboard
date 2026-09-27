# ADR-0001: Build a system-wide Input Method (IME)

- **Status:** Accepted
- **Date:** 2026-09-27

## Context

"A functional keyboard" for Bengali could mean several very different products:

1. A **system-wide input method engine (IME)** that integrates with the OS so the
   user can type Bengali in any application (this is what Avro / OpenBangla are).
2. An **on-screen virtual keyboard** — a standalone window with clickable keys.
3. A **standalone editor** with Bengali input built in.

These imply completely different architectures, so the choice must come first.

## Decision

Build a **system-wide IME**. The headline feature — typing phonetically in English
and getting Bengali — is only broadly useful if it works everywhere the user types
(browsers, editors, chat, terminals).

## Consequences

- We must integrate with Linux input-method frameworks (see
  [ADR-0002](ADR-0002-ime-frameworks.md)).
- More upfront integration work than a standalone app, so we stage delivery: a
  pure engine + CLI harness first, then IME integration (see
  [roadmap](../roadmap.md)).
- The core engine is designed to be framework-agnostic so the same logic backs
  every integration and future OS.
- An on-screen keyboard is **not excluded** — it can later be added as another
  frontend over the same engine, but it is not the primary product.
