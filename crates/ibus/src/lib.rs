//! # okb-ibus
//!
//! The [IBus](https://github.com/ibus/ibus) adapter for open-keyboard.
//!
//! IBus is the default input-method framework on Ubuntu/GNOME. This crate is a thin
//! transport that:
//!
//! 1. receives `ProcessKeyEvent(keyval, keycode, state)` from the IBus daemon,
//! 2. maps it to a framework-neutral [`okb_ime::Key`] ([`keymap`]),
//! 3. feeds it to a shared [`okb_ime::Session`] to decide commit/preedit/passthrough,
//! 4. and emits `CommitText` / `UpdatePreeditText` back to the daemon, encoding text
//!    in the [`ibus_text`] `IBusText` wire format.
//!
//! It is **pure Rust** over D-Bus ([zbus](https://github.com/dbus2/zbus)) — no
//! libibus/glib, so no C FFI and `unsafe` stays forbidden.
//!
//! The `keymap` and `ibus_text` modules are pure and unit-tested. The D-Bus service
//! wiring lives in [`engine`] and [`factory`] and is exercised end-to-end against a
//! live IBus daemon (see `docs/development/ibus-setup.md`).

// Product names (IBus, D-Bus, GObject) appear as prose in these docs.
#![allow(clippy::doc_markdown)]
// The `#[zbus::interface]` macro generates public trampoline/registration methods on
// our D-Bus objects that cannot carry doc comments. Our own public API is documented;
// allow missing docs here rather than lose the value of the macro.
#![allow(missing_docs)]
// D-Bus interface methods must take `&self` even when they hold no state, and the
// `#[interface]` macro reads arguments we name with a leading underscore because our
// body ignores them. Both are inherent to implementing the IBus protocol.
#![allow(clippy::unused_self)]
#![allow(clippy::used_underscore_binding)]

pub mod engine;
pub mod factory;
pub mod ibus_text;
pub mod keymap;

/// Well-known IBus D-Bus names and paths used by both the engine and its packaging.
pub mod names {
    /// The bus name our engine process requests on the IBus bus.
    pub const BUS_NAME: &str = "org.freedesktop.IBus.OpenKeyboard";
    /// The object path of our engine factory.
    pub const FACTORY_PATH: &str = "/org/freedesktop/IBus/Factory";
    /// The IBus engine name advertised in the component XML.
    pub const ENGINE_NAME: &str = "open-keyboard-bn";
}
