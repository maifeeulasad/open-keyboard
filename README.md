# open-keyboard

**A fast, safe, multilingual input method for the desktop — type in English, get
your own script.** First language: **Bengali (বাংলা)**. First OS: **Ubuntu**.

Type `amar sonar bangla`, get `আমার সোনার বাংলা` — in *any* application.

<!-- Badges: CI/security status and posture. -->
[![CI](https://github.com/maifeeulasad/open-keyboard/actions/workflows/ci.yml/badge.svg)](https://github.com/maifeeulasad/open-keyboard/actions/workflows/ci.yml)
[![CodeQL](https://github.com/maifeeulasad/open-keyboard/actions/workflows/codeql.yml/badge.svg)](https://github.com/maifeeulasad/open-keyboard/actions/workflows/codeql.yml)
[![Security audit](https://github.com/maifeeulasad/open-keyboard/actions/workflows/security-audit.yml/badge.svg)](https://github.com/maifeeulasad/open-keyboard/actions/workflows/security-audit.yml)
[![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](docs/architecture/security.md)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

> ⚠️ **Early development.** The linguistic core and a CLI harness work today; the
> system-wide IME integration (IBus, then Fcitx5) is next. See the
> [roadmap](docs/roadmap.md).

## Why

Bengali speakers (and, later, speakers of many scripts) should be able to type in
their language everywhere — not just in one app — by typing the way words sound. An
input method that sits in the keystroke path must also be **fast** and
**trustworthy**, so this project is built Rust-first, with a tiny audited dependency
footprint and security wired in from day one.

## Try it now

Needs [Rust](https://rustup.rs) ≥ 1.85. Nothing else.

```bash
cargo run -p okb-cli -- amar sonar bangla
#  => আমার সোনার বাংলা

echo "ami bangla likhi" | cargo run -q -p okb-cli
#  => আমি বাংলা লিখি
```

What to type is documented in the [phonetic scheme](docs/design/phonetic-scheme.md).

## How it's built

A pure, dependency-free **core engine** does the linguistics; thin **adapters** plug
it into each OS input-method framework; a **GTK4** UI handles settings and
suggestions.

```
okb-engine (pure Rust core)  ← okb-cli, okb-ibus, okb-fcitx5, okb-ui
```

- **Rust-first**, C/C++ only where a framework's ABI demands it (Fcitx5).
- **Minimal dependencies** — the core has *zero* third-party crates.
- **System-wide IME**, targeting **IBus** (Ubuntu default) then **Fcitx5**.
- **Clean-room** — no code or data from Avro/OpenBangla; they are studied as prior
  art only.

Full picture: [architecture overview](docs/architecture/overview.md).

## Security

open-keyboard sees everything you type, so security is a first-class requirement:
`#![forbid(unsafe_code)]` across pure-Rust crates, a pinned and audited dependency
tree, CodeQL + `cargo-deny` + Dependabot in CI, and private vulnerability reporting.

- Report a vulnerability: [SECURITY.md](SECURITY.md) (please **do not** open a public
  issue).
- Threat model: [docs/architecture/security.md](docs/architecture/security.md).

## Documentation

Everything is documented under [`docs/`](docs/README.md):

- [Architecture overview](docs/architecture/overview.md)
- [Decisions (ADRs)](docs/decisions/README.md)
- [Engine design](docs/design/engine.md) · [Phonetic scheme](docs/design/phonetic-scheme.md)
- [Roadmap & status](docs/roadmap.md)
- [Build & run](docs/development/build.md)

## Project management

Run **from this README and the docs** — no GitHub issues, no feature branches. The
[roadmap](docs/roadmap.md) is the task list; work lands in small commits on `main`.
See [how we run the project](docs/development/project-management.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions must be **clean-room**
(not copied from Avro/OpenBangla or similar) and pass the CI quality and security
gates.

## License

[GPL-3.0-only](LICENSE) © Maifee Ul Asad.
