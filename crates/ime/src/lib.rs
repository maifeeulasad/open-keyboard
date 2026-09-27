//! # okb-ime
//!
//! Framework-agnostic **input-method session logic** for open-keyboard.
//!
//! This crate sits between the pure transliteration engine ([`okb_engine`]) and
//! the OS-specific adapters ([IBus](https://github.com/ibus/ibus),
//! [Fcitx5](https://github.com/fcitx/fcitx5)). It owns the *preedit buffer* — the
//! Latin keystrokes the user has typed but not yet committed — and decides, for
//! each key, whether to:
//!
//! - update the on-screen preedit (composition) text,
//! - commit finished text to the application, and/or
//! - let the key pass through to the application unchanged.
//!
//! Crucially it contains **no D-Bus, no GObject, no toolkit** and does no I/O, so
//! the entire commit/passthrough policy is unit-testable in isolation and is
//! shared verbatim by every framework adapter (satisfying
//! `ADR-0002`: one behaviour, two frameworks).
//!
//! ## Object model
//!
//! - [`Key`] — a normalized key event. Adapters translate framework key codes and
//!   modifiers into this small set.
//! - [`Session`] — the stateful input context: buffer + policy.
//! - [`KeyResponse`] — what the adapter must do after a key: whether it was
//!   handled, any text to commit, and the preedit to display.
//!
//! ## Example
//!
//! ```
//! use okb_ime::{Key, Session};
//!
//! let mut s = Session::bengali();
//! for c in "bangla".chars() {
//!     s.press(Key::Char(c));
//! }
//! assert_eq!(s.preedit(), "বাংলা");
//!
//! // Space commits the word and passes the space through to the app.
//! let r = s.press(Key::Space);
//! assert_eq!(r.commit.as_deref(), Some("বাংলা"));
//! assert!(!r.handled); // the space itself is left for the application
//! assert_eq!(s.preedit(), "");
//! ```

// Product names (IBus, Fcitx5, Avro, OpenBangla) appear as prose in these docs.
#![allow(clippy::doc_markdown)]

mod session;

pub use session::{KeyResponse, Session};

/// A normalized key event, independent of any input-method framework.
///
/// Adapters are responsible for mapping their framework's key codes and modifier
/// state onto these variants. In particular, a key pressed while a control-style
/// modifier (Ctrl/Alt/Super) is held should be delivered as [`Key::Other`], since
/// it is a shortcut rather than text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// A text-producing character, e.g. `'b'` or `'B'` (case is significant to the
    /// phonetic scheme).
    Char(char),
    /// Backspace: delete the last unit of the preedit, or pass through when idle.
    Backspace,
    /// Space: commit the current word, then pass through so the app inserts a space.
    Space,
    /// Enter/Return: commit the current word, then pass through.
    Enter,
    /// Escape: discard the current preedit, or pass through when idle.
    Escape,
    /// Any other key (arrows, Tab, function keys, modified shortcuts): commit the
    /// current word first, then pass through.
    Other,
}
