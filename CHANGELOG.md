# Changelog

All notable changes to open-keyboard are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/), and the project aims to follow
[Semantic Versioning](https://semver.org/).

## [0.1.0] — 2026-09-27

First tagged release: a working, system-wide Bengali phonetic input method on
Ubuntu/GNOME, plus a cross-platform transliteration CLI.

### Added

- **`okb-engine`** — clean-room, dependency-free English→Bengali phonetic
  transliteration core (schemes, sounds, contextual conjuncts/vowel signs).
- **`okb-ime`** — framework-agnostic input-session state machine (preedit buffer +
  commit/passthrough policy), shared by input-method adapters.
- **`okb-ibus`** — pure-Rust IBus engine (via zbus) driving the shared session;
  confirmed typing Bengali system-wide on Ubuntu/GNOME. Composing text is underlined.
- **`okb-cli`** (`okb`) — command-line transliterator for scripting and demos.
- **Packaging** — Debian `.deb`, install/uninstall scripts, IBus component
  descriptor, and a `Makefile`.
- **Release artifacts** — Linux `.deb` + tarball (engine + CLI), Windows CLI `.zip`,
  macOS universal CLI `.tar.gz`, built and published by CI.
- **Security & CI** — `#![forbid(unsafe_code)]` in all pure-Rust crates, minimal and
  pinned dependencies, CodeQL, `cargo-deny`, Dependabot, and full docs/ADRs.

### Notes

- The system-wide IME is Linux/IBus for now; Windows and macOS ship the `okb` CLI.
  Native IMEs for those platforms (TSF/IMK) and Fcitx5 support are planned.

[0.1.0]: https://github.com/maifeeulasad/open-keyboard/releases/tag/v0.1.0
