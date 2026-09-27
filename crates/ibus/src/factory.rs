//! The `org.freedesktop.IBus.Factory` D-Bus object.
//!
//! The daemon calls `CreateEngine` when the user switches to our input method; we
//! instantiate an [`Engine`], export it on a fresh object path, and return that path.

use std::sync::atomic::{AtomicU64, Ordering};

use zbus::interface;
use zbus::object_server::ObjectServer;
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

use crate::engine::Engine;

/// The engine factory. Holds a counter to hand out unique engine object paths.
#[derive(Default)]
pub struct Factory {
    next_id: AtomicU64,
}

#[interface(name = "org.freedesktop.IBus.Factory")]
impl Factory {
    /// Create a new engine instance and return its object path.
    async fn create_engine(
        &self,
        _engine_name: String,
        #[zbus(object_server)] server: &ObjectServer,
    ) -> zbus::fdo::Result<OwnedObjectPath> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let path_str = format!("/org/freedesktop/IBus/Engine/OpenKeyboard/{id}");
        let path = ObjectPath::try_from(path_str)
            .map_err(|e| zbus::fdo::Error::Failed(format!("invalid engine path: {e}")))?;

        server
            .at(&path, Engine::new())
            .await
            .map_err(|e| zbus::fdo::Error::Failed(format!("failed to export engine: {e}")))?;

        Ok(path.into())
    }
}
