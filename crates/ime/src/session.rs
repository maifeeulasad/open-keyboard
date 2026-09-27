//! The input session: preedit buffer plus the per-key commit/passthrough policy.

use okb_engine::{PhoneticScheme, Scheme};

use crate::Key;

/// The action an adapter must take after feeding a [`Key`] to a [`Session`].
///
/// The adapter should, in order: commit `commit` (if `Some`) to the application,
/// set the application's preedit/composition text to `preedit`, and then report the
/// key as consumed iff `handled` is `true` (when `false`, the original key event is
/// forwarded to the application).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyResponse {
    /// Whether the session consumed the key. When `false`, the adapter forwards the
    /// original key event to the focused application.
    pub handled: bool,
    /// Text to commit to the application before updating the preedit, if any.
    pub commit: Option<String>,
    /// The preedit (composition) string to display after this key. Empty means the
    /// preedit should be cleared.
    pub preedit: String,
}

impl KeyResponse {
    /// The key was not consumed; nothing to commit, preedit unchanged/empty.
    fn ignored() -> Self {
        Self {
            handled: false,
            commit: None,
            preedit: String::new(),
        }
    }

    /// The key was consumed and updated the preedit.
    fn preedit(text: String) -> Self {
        Self {
            handled: true,
            commit: None,
            preedit: text,
        }
    }

    /// Commit `text`, clear the preedit, and forward the original key to the app.
    fn commit_and_forward(text: String) -> Self {
        Self {
            handled: false,
            commit: Some(text),
            preedit: String::new(),
        }
    }
}

/// A single input context: the buffer of un-committed Latin keystrokes plus the
/// policy that turns key events into [`KeyResponse`]s.
///
/// A `Session` does no I/O and holds no framework state; an adapter creates one per
/// input context and feeds it normalized [`Key`]s.
#[derive(Clone, Debug)]
pub struct Session {
    scheme: PhoneticScheme,
    /// Raw Latin keystrokes not yet committed (e.g. `"bangla"`).
    buffer: String,
}

impl Session {
    /// Create a session driven by `scheme`.
    #[must_use]
    pub fn new(scheme: PhoneticScheme) -> Self {
        Self {
            scheme,
            buffer: String::new(),
        }
    }

    /// Create a session using the built-in Bengali phonetic scheme.
    #[must_use]
    pub fn bengali() -> Self {
        Self::new(PhoneticScheme::bengali())
    }

    /// Whether there is un-committed input (a non-empty preedit).
    #[must_use]
    pub fn is_composing(&self) -> bool {
        !self.buffer.is_empty()
    }

    /// The raw Latin buffer (what the user typed), for debugging/inspection.
    #[must_use]
    pub fn raw_buffer(&self) -> &str {
        &self.buffer
    }

    /// The current preedit (composition) text in the target script.
    #[must_use]
    pub fn preedit(&self) -> String {
        self.scheme.transliterate(&self.buffer)
    }

    /// Discard any un-committed input without committing it.
    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    /// Commit and clear whatever is currently buffered — e.g. on focus-out.
    ///
    /// Returns the committed text, or `None` if nothing was buffered.
    pub fn flush(&mut self) -> Option<String> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(self.take_commit())
        }
    }

    /// Feed one normalized key event and get the action to perform.
    pub fn press(&mut self, key: Key) -> KeyResponse {
        match key {
            Key::Char(c) if is_input_char(c) => {
                self.buffer.push(c);
                KeyResponse::preedit(self.preedit())
            }
            Key::Backspace => {
                if self.buffer.pop().is_some() {
                    KeyResponse::preedit(self.preedit())
                } else {
                    KeyResponse::ignored()
                }
            }
            Key::Escape => {
                if self.is_composing() {
                    self.buffer.clear();
                    KeyResponse {
                        handled: true,
                        commit: None,
                        preedit: String::new(),
                    }
                } else {
                    KeyResponse::ignored()
                }
            }
            // Everything else finishes the current word and passes through: a
            // non-input character (punctuation like '.', '-', '/'), Space, Enter,
            // and any other/modified key.
            Key::Char(_) | Key::Space | Key::Enter | Key::Other => self.terminate(),
        }
    }

    /// Finish the current word (if any) and forward the triggering key to the app.
    fn terminate(&mut self) -> KeyResponse {
        if self.buffer.is_empty() {
            KeyResponse::ignored()
        } else {
            KeyResponse::commit_and_forward(self.take_commit())
        }
    }

    /// Transliterate and clear the buffer, returning the committed text.
    fn take_commit(&mut self) -> String {
        let text = self.preedit();
        self.buffer.clear();
        text
    }
}

/// Characters that participate in romanization and should extend the preedit.
///
/// ASCII alphanumerics plus the scheme's punctuation keys (`,` for the explicit
/// virama `,,`, `` ` `` for khanda-ta `` t`` ``, `:` visarga, `^` chandrabindu).
fn is_input_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, ',' | ':' | '^' | '`')
}

#[cfg(test)]
mod tests {
    use crate::{Key, Session};

    fn type_str(s: &mut Session, text: &str) {
        for c in text.chars() {
            s.press(Key::Char(c));
        }
    }

    #[test]
    fn typing_builds_preedit_without_committing() {
        let mut s = Session::bengali();
        type_str(&mut s, "bangla");
        assert_eq!(s.preedit(), "বাংলা");
        assert!(s.is_composing());
    }

    #[test]
    fn space_commits_word_and_passes_through() {
        let mut s = Session::bengali();
        type_str(&mut s, "ami");
        let r = s.press(Key::Space);
        assert_eq!(r.commit.as_deref(), Some("আমি"));
        assert!(!r.handled, "space itself must reach the application");
        assert_eq!(r.preedit, "");
        assert!(!s.is_composing());
    }

    #[test]
    fn enter_commits_word_and_passes_through() {
        let mut s = Session::bengali();
        type_str(&mut s, "sonar");
        let r = s.press(Key::Enter);
        assert_eq!(r.commit.as_deref(), Some("সোনার"));
        assert!(!r.handled);
    }

    #[test]
    fn space_while_idle_is_ignored() {
        let mut s = Session::bengali();
        let r = s.press(Key::Space);
        assert!(!r.handled);
        assert_eq!(r.commit, None);
        assert_eq!(r.preedit, "");
    }

    #[test]
    fn backspace_removes_last_unit() {
        let mut s = Session::bengali();
        type_str(&mut s, "bangla");
        let r = s.press(Key::Backspace);
        assert!(r.handled);
        assert_eq!(r.preedit, "বাংল");
        assert_eq!(s.raw_buffer(), "bangl");
    }

    #[test]
    fn backspace_while_idle_passes_through() {
        let mut s = Session::bengali();
        let r = s.press(Key::Backspace);
        assert!(!r.handled, "app must handle backspace when we are idle");
        assert_eq!(r.commit, None);
    }

    #[test]
    fn escape_discards_preedit_without_commit() {
        let mut s = Session::bengali();
        type_str(&mut s, "ami");
        let r = s.press(Key::Escape);
        assert!(r.handled);
        assert_eq!(r.commit, None);
        assert_eq!(r.preedit, "");
        assert!(!s.is_composing());
    }

    #[test]
    fn punctuation_terminates_and_commits() {
        let mut s = Session::bengali();
        type_str(&mut s, "ki");
        let r = s.press(Key::Char('.'));
        assert_eq!(r.commit.as_deref(), Some("কি"));
        assert!(!r.handled);
    }

    #[test]
    fn other_key_terminates_when_composing() {
        let mut s = Session::bengali();
        type_str(&mut s, "ami");
        let r = s.press(Key::Other);
        assert_eq!(r.commit.as_deref(), Some("আমি"));
        assert!(!r.handled);
    }

    #[test]
    fn flush_commits_remaining_buffer() {
        let mut s = Session::bengali();
        type_str(&mut s, "bhalo");
        assert_eq!(s.flush().as_deref(), Some("ভালো"));
        assert_eq!(s.flush(), None);
        assert!(!s.is_composing());
    }

    #[test]
    fn full_sentence_flow() {
        let mut s = Session::bengali();
        let mut committed = String::new();
        for word in ["amar", "sonar", "bangla"] {
            type_str(&mut s, word);
            let r = s.press(Key::Space);
            if let Some(text) = r.commit {
                committed.push_str(&text);
            }
            committed.push(' '); // the passed-through space
        }
        assert_eq!(committed, "আমার সোনার বাংলা ");
    }
}
