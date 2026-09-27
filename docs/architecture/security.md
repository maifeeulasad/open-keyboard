# Security architecture & threat model

An input method is one of the most security-sensitive components on a desktop: it
observes **every keystroke**, including passwords, recovery phrases, and private
messages, and it runs continuously in the user's session. This document states
what we defend against and how.

## Assets

| Asset | Why it matters |
| ----- | -------------- |
| Keystroke stream | Contains secrets (passwords, 2FA codes, private text). |
| Preedit buffer | Transient plaintext of what the user is typing. |
| User dictionary / learned words (future) | May contain personal or sensitive terms. |
| Configuration | Controls behaviour; tampering could redirect input. |
| Build & release artifacts | Supply-chain target; a trojaned build sees all input. |

## Threat model

We consider these adversaries and threats:

1. **Malicious or vulnerable dependency (supply chain).** A compromised crate in
   our tree would run inside the keystroke path.
   - *Mitigations:* zero dependencies in the core; minimal, pinned, audited
     dependencies elsewhere ([ADR-0005](../decisions/ADR-0005-dependency-policy.md));
     `cargo-deny` + Dependabot + `Cargo.lock` committed.
2. **Memory-safety exploitation.** A memory bug in an input-path component is a
   high-value target.
   - *Mitigations:* `#![forbid(unsafe_code)]` in all pure-Rust crates; `unsafe`
     permitted only in FFI shims, each block documented and minimised; CodeQL and
     (for C++) sanitizer builds in CI when the Fcitx5 shim lands.
3. **Secret leakage / exfiltration.** The engine could, by accident or malice,
   log or transmit typed text.
   - *Mitigations:* the core performs **no I/O** — no network, no filesystem, no
     logging of buffer contents. This is enforced by design (std-only, no `std::fs`
     / `std::net` use in the engine) and reviewable because the crate is tiny.
     Persistence (user dictionary) will be local-only and opt-in.
4. **Tampered configuration / dictionary files.** Malformed or hostile data files.
   - *Mitigations:* strict parsing with no code execution; treat all on-disk data
     as untrusted input; fail closed to built-in defaults.
5. **Compromised release/build.** A backdoored binary.
   - *Mitigations:* reproducible-friendly release profile, CI-built artifacts,
     signed releases (planned), and a documented, minimal build.

## Non-goals (for now)

- Defending against a fully compromised OS or a root-level keylogger — outside any
  IME's control.
- Encrypting the preedit buffer in memory — not meaningful against a local root
  attacker; revisit if a concrete threat emerges.

## Secure-development practices

- **Private vulnerability reporting** and coordinated disclosure via GitHub
  Security Advisories ([SECURITY.md](../../SECURITY.md)).
- **Least privilege in CI:** workflows declare minimal `permissions:`.
- **No secrets in the repo;** no telemetry.
- **Clean-room provenance** so we inherit neither code nor the security questions
  of other projects ([ADR-0006](../decisions/ADR-0006-clean-room-phonetic-scheme.md)).

## Review checklist for input-path changes

Before merging anything that touches the engine or an adapter:

- [ ] No new dependency without an ADR update.
- [ ] No `unsafe` outside a documented FFI shim.
- [ ] No logging, network, or filesystem access added to the core.
- [ ] Buffer contents never written to logs or error messages.
- [ ] `cargo test`, `cargo clippy -D warnings`, `cargo fmt --check`, `cargo deny`
      all pass.
