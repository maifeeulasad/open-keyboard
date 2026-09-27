# ADR-0006: Clean-room scheme; no Avro/OpenBangla code or data

- **Status:** Accepted
- **Date:** 2026-09-27

## Context

Avro Keyboard and OpenBangla Keyboard are the best-known Bengali phonetic input
projects and excellent prior art. However:

- Reusing their **source code or rule/dictionary data** would entangle us with their
  licensing and provenance.
- These projects have recently faced **security questions**, and we do not want to
  inherit code whose trust status is uncertain for a component that sits in the
  keystroke path.

## Decision

Develop open-keyboard **clean-room**:

- We may **study** Avro/OpenBangla and other projects as prior art and cite them.
- We will **not copy** their source code, romanization rule tables, dictionaries, or
  other data files.
- Our romanization scheme is designed **independently** from primary sources — the
  Unicode Bengali block (U+0980–U+09FF) and general, non-proprietary transliteration
  conventions — and is fully documented in
  [design/phonetic-scheme.md](../design/phonetic-scheme.md).

## Consequences

- Our romanization keys will *resemble* common phonetic conventions (that is
  unavoidable and not copyrightable — they are functional mappings), but the tables
  are authored here from scratch.
- We own our provenance and can answer security/licensing questions cleanly.
- We forgo their tuned dictionaries and edge-case rules; we rebuild what we need,
  guided by the Unicode standard and tests.
- Any contribution must certify it is not copied from those projects (see
  [CONTRIBUTING](../../CONTRIBUTING.md)).

## References (prior art — studied, not copied)

- OpenBangla Keyboard — <https://github.com/OpenBangla/OpenBangla-Keyboard>
- riti — <https://github.com/OpenBangla/riti>
- Avro Keyboard — <https://www.omicronlab.com/avro-keyboard.html>
- Unicode Bengali chart — <https://www.unicode.org/charts/PDF/U0980.pdf>
