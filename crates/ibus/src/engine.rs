//! The `org.freedesktop.IBus.Engine` D-Bus object.
//!
//! One [`Engine`] exists per input context. It owns an [`okb_ime::Session`], turns
//! `ProcessKeyEvent` into session actions, and emits `CommitText` /
//! `UpdatePreeditText` back to the daemon.

use okb_ime::Session;
use zbus::interface;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::Value;

use crate::ibus_text::{ibus_text, ibus_text_with_attrs, underline};
use crate::keymap::map_key;

/// `IBusPreeditFocusMode::Commit` — commit the preedit if focus is lost.
const PREEDIT_MODE_COMMIT: u32 = 1;

/// A single IBus engine instance (one per input context).
pub struct Engine {
    session: Session,
}

impl Engine {
    /// Create an engine using the Bengali phonetic scheme.
    #[must_use]
    pub fn new() -> Self {
        Self {
            session: Session::bengali(),
        }
    }

    /// Emit the preedit update for the current session state, underlining the
    /// composing text so it is visually distinct from committed text.
    async fn emit_preedit(emitter: &SignalEmitter<'_>, preedit: &str) {
        let len = u32::try_from(preedit.chars().count()).unwrap_or(0);
        // Underline the whole preedit; fall back to plain text if the attribute or
        // text cannot be built (never panic in the keystroke path).
        let text = match underline(0, len) {
            Ok(attr) => ibus_text_with_attrs(preedit, vec![attr]),
            Err(_) => ibus_text(preedit),
        };
        let Ok(text) = text else { return };
        let visible = !preedit.is_empty();
        let _ = Engine::update_preedit_text(emitter, text, len, visible, PREEDIT_MODE_COMMIT).await;
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[interface(name = "org.freedesktop.IBus.Engine")]
impl Engine {
    /// The heart of the engine: decide what a key does.
    async fn process_key_event(
        &mut self,
        keyval: u32,
        keycode: u32,
        state: u32,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> bool {
        let Some(key) = map_key(keyval, keycode, state) else {
            return false; // key release: let the application have it
        };

        let response = self.session.press(key);

        if let Some(committed) = response.commit {
            if let Ok(text) = ibus_text(&committed) {
                let _ = Engine::commit_text(&emitter, text).await;
            }
        }
        Engine::emit_preedit(&emitter, &response.preedit).await;

        response.handled
    }

    /// Focus lost: commit anything in progress so it is not stranded.
    async fn focus_out(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        if let Some(committed) = self.session.flush() {
            if let Ok(text) = ibus_text(&committed) {
                let _ = Engine::commit_text(&emitter, text).await;
            }
        }
        Engine::emit_preedit(&emitter, "").await;
    }

    /// Focus gained: nothing to restore in this version.
    fn focus_in(&self) {}

    /// Discard the current composition without committing.
    async fn reset(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        self.session.reset();
        Engine::emit_preedit(&emitter, "").await;
    }

    /// Engine enabled by the user.
    fn enable(&mut self) {
        self.session.reset();
    }

    /// Engine disabled by the user.
    async fn disable(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        self.session.reset();
        Engine::emit_preedit(&emitter, "").await;
    }

    /// The input context is going away.
    fn destroy(&mut self) {
        self.session.reset();
    }

    // ---- No-op methods IBus may call; we accept and ignore them. ----------------

    fn set_capabilities(&self, _caps: u32) {}
    fn set_cursor_location(&self, _x: i32, _y: i32, _w: i32, _h: i32) {}
    fn set_surrounding_text(&self, _text: Value<'_>, _cursor_index: u32, _anchor_pos: u32) {}
    fn property_activate(&self, _name: String, _state: u32) {}
    fn page_up(&self) {}
    fn page_down(&self) {}
    fn cursor_up(&self) {}
    fn cursor_down(&self) {}
    fn candidate_clicked(&self, _index: u32, _button: u32, _state: u32) {}

    // ---- Signals emitted to the daemon. ----------------------------------------

    /// Insert finished text into the focused application.
    #[zbus(signal)]
    async fn commit_text(emitter: &SignalEmitter<'_>, text: Value<'_>) -> zbus::Result<()>;

    /// Update the underlined composition text.
    #[zbus(signal)]
    async fn update_preedit_text(
        emitter: &SignalEmitter<'_>,
        text: Value<'_>,
        cursor_pos: u32,
        visible: bool,
        mode: u32,
    ) -> zbus::Result<()>;
}
