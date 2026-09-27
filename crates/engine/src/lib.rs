//! # okb-engine
//!
//! The framework-agnostic core of **open-keyboard**.
//!
//! This crate turns a stream of Latin ("English") keystrokes into Unicode text
//! in a target script. Version 0.x targets **Bengali** (বাংলা) through a
//! *phonetic* scheme, e.g. typing `amar sonar bangla` yields `আমার সোনার বাংলা`.
//!
//! ## Design
//!
//! The engine is deliberately isolated from any operating system, input-method
//! framework (IBus / Fcitx5), or UI toolkit. Those concerns live in separate
//! crates that depend on this one. Keeping the linguistic core pure means it is:
//!
//! - **portable** — the same logic will back every OS we support later;
//! - **testable** — behaviour is verified with plain unit tests, no display server;
//! - **auditable** — it has *no* third-party dependencies and forbids `unsafe`.
//!
//! ## Clean-room note
//!
//! The romanization tables here are an *independent* design derived from the
//! Bengali Unicode block (U+0980–U+09FF) and common transliteration conventions.
//! No rule tables, dictionaries, or source code were taken from Avro Keyboard,
//! OpenBangla Keyboard, or any other project. See
//! `docs/decisions/ADR-0006-clean-room-phonetic-scheme.md`.
//!
//! ## Object model
//!
//! - [`Scheme`] — a trait: anything that can transliterate a string.
//! - [`PhoneticScheme`] — the concrete Bengali phonetic scheme.
//! - [`Sound`] — the typed linguistic unit a romanization rule maps to.
//! - [`Transliterator`] — applies a [`Scheme`] with contextual state.
//!
//! ## Example
//!
//! ```
//! use okb_engine::{PhoneticScheme, Scheme};
//!
//! let scheme = PhoneticScheme::bengali();
//! assert_eq!(scheme.transliterate("ami"), "আমি");
//! assert_eq!(scheme.transliterate("bangla"), "বাংলা");
//! ```

// Product names (Avro, OpenBangla, IBus, Fcitx5) legitimately appear as prose in
// these docs; do not force them into code spans.
#![allow(clippy::doc_markdown)]

mod scheme;
mod transliterate;

pub use scheme::{PhoneticScheme, Sound};
pub use transliterate::Transliterator;

/// A transliteration scheme: converts a Latin input string into target-script text.
///
/// This is the primary abstraction consumed by the OS/IME adapter crates. New
/// scripts or layouts are added by implementing this trait, without touching the
/// adapters.
pub trait Scheme {
    /// A short, stable identifier for the scheme, e.g. `"bengali-phonetic"`.
    fn id(&self) -> &str;

    /// A human-readable name, e.g. `"Bengali (Phonetic)"`.
    fn display_name(&self) -> &str;

    /// Transliterate a complete input buffer into target-script text.
    fn transliterate(&self, input: &str) -> String;
}
