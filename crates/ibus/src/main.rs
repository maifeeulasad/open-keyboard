//! `okb-ibus-engine` — the IBus engine process.
//!
//! The IBus daemon launches this executable (per the installed component XML). It
//! connects to the IBus bus, exports the [`Factory`], requests our well-known name,
//! and then waits for the daemon to drive it.
//!
//! Run manually for debugging with `okb-ibus-engine --ibus`.

// Product names (IBus, D-Bus) appear as prose in these docs.
#![allow(clippy::doc_markdown)]

use std::error::Error;
use std::process::Command;

use okb_ibus::factory::Factory;
use okb_ibus::names;

fn main() -> Result<(), Box<dyn Error>> {
    let address = ibus_address()?;

    // Pure-Rust D-Bus connection to the IBus bus. Signals emitted from the exported
    // objects are dispatched by zbus's internal executor, so `main` just needs to
    // stay alive.
    let _connection = zbus::blocking::connection::Builder::address(address.as_str())?
        .name(names::BUS_NAME)?
        .serve_at(names::FACTORY_PATH, Factory::default())?
        .build()?;

    eprintln!(
        "okb-ibus-engine: connected to IBus, serving {} at {}",
        names::BUS_NAME,
        names::FACTORY_PATH
    );

    // Park forever; the daemon drives us over D-Bus until the process is killed.
    loop {
        std::thread::park();
    }
}

/// Discover the IBus bus address.
///
/// Order: the `IBUS_ADDRESS` environment variable (set when the daemon launches us),
/// then `ibus address` as a fallback for manual runs.
fn ibus_address() -> Result<String, Box<dyn Error>> {
    if let Ok(addr) = std::env::var("IBUS_ADDRESS") {
        if !addr.is_empty() {
            return Ok(addr);
        }
    }

    let output = Command::new("ibus").arg("address").output().map_err(|e| {
        format!("could not run `ibus address` (is IBus installed and running?): {e}")
    })?;
    let addr = String::from_utf8(output.stdout)?.trim().to_string();
    if addr.is_empty() || addr == "(null)" {
        return Err("IBus address is unavailable; is the IBus daemon running?".into());
    }
    Ok(addr)
}
