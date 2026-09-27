//! Translate raw IBus key events into the framework-neutral [`okb_ime::Key`].
//!
//! IBus delivers `ProcessKeyEvent(keyval, keycode, state)` where `keyval` is an X11
//! keysym and `state` is a bitmask of [`modifiers`]. This module contains the pure
//! mapping logic — no D-Bus — so it can be unit-tested exhaustively.

use okb_ime::Key;

/// X11 keysym values IBus uses for the non-text keys we care about.
mod keysym {
    pub const BACKSPACE: u32 = 0xff08;
    pub const RETURN: u32 = 0xff0d;
    pub const ESCAPE: u32 = 0xff1b;
    pub const KP_ENTER: u32 = 0xff8d;
    pub const SPACE: u32 = 0x0020;
    /// Inclusive range of directly-printable Latin-1 keysyms (`!` ..= `~`), whose
    /// keysym value equals the character's code point.
    pub const PRINTABLE_LO: u32 = 0x0021;
    pub const PRINTABLE_HI: u32 = 0x007e;
}

/// `IBusModifierType` bits (subset). See IBus `ibustypes.h`.
pub mod modifiers {
    /// Shift is fine for text (produces capitals); it does not make a shortcut.
    pub const SHIFT: u32 = 1 << 0;
    /// Caps Lock; also fine for text.
    pub const LOCK: u32 = 1 << 1;
    /// Ctrl — turns a key into a shortcut.
    pub const CONTROL: u32 = 1 << 2;
    /// Mod1 / Alt — shortcut.
    pub const ALT: u32 = 1 << 3;
    /// Super (logo key) — shortcut.
    pub const SUPER: u32 = 1 << 26;
    /// Hyper — shortcut.
    pub const HYPER: u32 = 1 << 27;
    /// Meta — shortcut.
    pub const META: u32 = 1 << 28;
    /// Set on key-release events; we act only on presses.
    pub const RELEASE: u32 = 1 << 30;

    /// Any modifier that turns a key into an application shortcut rather than text.
    pub const SHORTCUT: u32 = CONTROL | ALT | SUPER | HYPER | META;
}

/// Map an IBus key event to a [`Key`], or `None` when it should be ignored and
/// forwarded to the application untouched (key releases).
///
/// A key pressed with a shortcut modifier (Ctrl/Alt/Super/…) is mapped to
/// [`Key::Other`], so the session finishes the current word and lets the shortcut
/// through.
#[must_use]
pub fn map_key(keyval: u32, _keycode: u32, state: u32) -> Option<Key> {
    // Ignore key-release events entirely.
    if state & modifiers::RELEASE != 0 {
        return None;
    }

    // A shortcut modifier means "not text": end composition and pass through.
    if state & modifiers::SHORTCUT != 0 {
        return Some(Key::Other);
    }

    let key = match keyval {
        keysym::BACKSPACE => Key::Backspace,
        keysym::RETURN | keysym::KP_ENTER => Key::Enter,
        keysym::ESCAPE => Key::Escape,
        keysym::SPACE => Key::Space,
        keysym::PRINTABLE_LO..=keysym::PRINTABLE_HI => {
            // In this range the keysym equals the ASCII/Unicode code point.
            match char::from_u32(keyval) {
                Some(c) => Key::Char(c),
                None => Key::Other,
            }
        }
        // Tab, arrows, function keys, and anything else are non-text: end the word
        // and pass through.
        _ => Key::Other,
    };
    Some(key)
}

#[cfg(test)]
mod tests {
    use super::{map_key, modifiers};
    use okb_ime::Key;

    #[test]
    fn lowercase_letters_map_to_chars() {
        assert_eq!(map_key(0x62, 0, 0), Some(Key::Char('b'))); // 'b'
        assert_eq!(map_key(0x61, 0, 0), Some(Key::Char('a'))); // 'a'
    }

    #[test]
    fn shift_preserves_capital_letters_as_text() {
        // 'T' arrives as keysym 0x54 with Shift held — still text, not a shortcut.
        assert_eq!(map_key(0x54, 0, modifiers::SHIFT), Some(Key::Char('T')));
    }

    #[test]
    fn control_and_alt_become_other() {
        assert_eq!(map_key(0x63, 0, modifiers::CONTROL), Some(Key::Other)); // Ctrl+C
        assert_eq!(map_key(0x61, 0, modifiers::ALT), Some(Key::Other)); // Alt+A
        assert_eq!(map_key(0x61, 0, modifiers::SUPER), Some(Key::Other));
    }

    #[test]
    fn named_keys_map_correctly() {
        assert_eq!(map_key(0xff08, 0, 0), Some(Key::Backspace));
        assert_eq!(map_key(0xff0d, 0, 0), Some(Key::Enter));
        assert_eq!(map_key(0xff8d, 0, 0), Some(Key::Enter)); // keypad enter
        assert_eq!(map_key(0xff1b, 0, 0), Some(Key::Escape));
        assert_eq!(map_key(0x0020, 0, 0), Some(Key::Space));
        assert_eq!(map_key(0xff09, 0, 0), Some(Key::Other)); // Tab
    }

    #[test]
    fn arrows_and_unknown_keys_become_other() {
        assert_eq!(map_key(0xff51, 0, 0), Some(Key::Other)); // Left
        assert_eq!(map_key(0xff52, 0, 0), Some(Key::Other)); // Up
    }

    #[test]
    fn release_events_are_ignored() {
        assert_eq!(map_key(0x62, 0, modifiers::RELEASE), None);
    }

    #[test]
    fn digits_and_punctuation_map_to_chars() {
        assert_eq!(map_key(0x32, 0, 0), Some(Key::Char('2'))); // '2'
        assert_eq!(map_key(0x2e, 0, 0), Some(Key::Char('.'))); // '.'
        assert_eq!(map_key(0x2c, 0, 0), Some(Key::Char(','))); // ','
    }
}
