# Roadmap & status

This is the project's task tracker. We do **not** use GitHub issues or feature
branches (see [project management](development/project-management.md)); work is
planned and tracked here and in commit history.

Legend: ✅ done · 🚧 in progress · ⏳ planned

## Phase 0 — Foundation ✅ (2026-09-27)

- ✅ Architecture decided and recorded ([ADRs](decisions/README.md)).
- ✅ Cargo workspace, `okb-engine` (core), `okb-cli` (harness).
- ✅ Clean-room Bengali phonetic engine v0 with tests.
- ✅ Documentation tree under `docs/`.
- ✅ Security infra in code: CI (fmt/clippy/test), CodeQL, cargo-deny, Dependabot,
  `SECURITY.md`, `#![forbid(unsafe_code)]`, committed `Cargo.lock`.
- ⏳ **Maintainer to enable in GitHub UI** (Settings → Code security and analysis):
  private vulnerability reporting, Dependabot alerts/updates, secret scanning +
  push protection, code scanning, branch protection on `main`. (See
  [ADR-0007](decisions/ADR-0007-security-controls.md).)

## Phase 1 — Engine maturity ⏳

- ⏳ Expand the phonetic scheme (y-phala/r-phala, more edge cases) with tests.
- ⏳ Property tests / fuzzing on the transliterator (idempotence, no panics).
- ⏳ Externalise rule tables into a data file with a strict, dependency-free parser
  (keeps the scheme editable without recompiling).
- ⏳ Benchmarks; confirm per-keystroke latency budget (< 1 ms for word-length input).

## Phase 2 — IBus integration (Ubuntu default) 🚧

- ✅ `okb-ime`: framework-agnostic input-session state machine (preedit buffer +
  commit/passthrough policy), fully unit-tested. Shared by IBus and (later) Fcitx5.
- ✅ `okb-ibus`: pure-Rust (zbus) IBus Engine + Factory driving `okb-ime`.
  - ✅ keyval/modifier → key mapping (unit-tested).
  - ✅ `IBusText` D-Bus wire format, pinned by signature tests.
  - ✅ compiles and passes `clippy -D warnings`.
- ✅ Packaging: IBus component descriptor, `install.sh`/`uninstall.sh`, `Makefile`,
  and [setup guide](development/ibus-setup.md).
- 🚧 **Live end-to-end validation on a running Ubuntu/GNOME session** (needs a desktop
  session; not available in CI/sandbox). First install is a bring-up — see the setup
  guide's status note.
- ⏳ `.deb` packaging for a one-command install.
- ⏳ Test matrix: GNOME on Wayland and X11; browser, editor, terminal.
- ⏳ Preedit underline styling (the `ibus_text` attribute plumbing already exists).

## Phase 3 — Fcitx5 integration ⏳

- ⏳ `okb-ffi`: minimal, documented C ABI over the engine.
- ⏳ `okb-fcitx5`: C++ addon shim calling the Rust engine; add CodeQL `cpp` +
  sanitizer CI.

## Phase 4 — UI (GTK4) ⏳

- ⏳ `okb-ui`: preferences window (scheme/layout/behaviour).
- ⏳ Candidate/suggestion popup (prefer the framework's native candidate window
  where adequate).

## Phase 5 — Smart input ⏳

- ⏳ Local, opt-in user dictionary (privacy-first; see [security](architecture/security.md)).
- ⏳ Word suggestions / auto-correction backed by a Bengali word list built from
  openly-licensed sources (clean-room; no third-party dictionaries copied).

## Phase 6 — Beyond Ubuntu & Bengali ⏳

- ⏳ Additional Linux distros / Wayland compositors.
- ⏳ Additional scripts (each a new `Scheme`): the engine already generalises to
  Brahmic scripts.
- ⏳ Other OSes (Windows/TSF, macOS/IMK) reusing the same core.

## Immediate next steps

1. **Live bring-up of the IBus engine** on Ubuntu/GNOME: `make install`, enable the
   input source, and type — following [ibus-setup](development/ibus-setup.md). Report
   anything off so we can iterate to a confirmed end-to-end working state.
2. Maintainer: flip the GitHub security settings listed in Phase 0.
3. Push and confirm CI is green.
4. Then: preedit underline + `.deb` packaging, or start Phase 1 scheme/fuzzing work.
