use std::io::{BufWriter, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use log::{error, info};
use parking_lot::Mutex;
use rdev::{Event, EventType};

use crate::common::Statistics;
use crate::filesystem;

// For inter-process communication
pub struct Listener {
    statistics: Mutex<Statistics>,
    inner: UnixListener,
}

impl Listener {
    pub fn bind(socket_path: PathBuf) -> Result<Self> {
        filesystem::drop_socket_file(&socket_path)?;
        let statistics = Mutex::new(Statistics::default());
        let listener = UnixListener::bind(&socket_path)
            .with_context(|| "Failed to bind unix socket listener.")?;

        Ok(Listener { statistics, inner: listener })
    }

    pub fn on_keyboard_press(self: &mut Arc<Self>, event: Event) {
        let Event { event_type, .. } = event;
        if let EventType::KeyPress(key) = event_type {
            self.statistics.lock().press(key);
        }
    }

    pub fn serve(self: Arc<Self>) {
        info!("Server start");

        for stream in self.inner.incoming().flatten() {
            info!("connection received");
            let mut writer = BufWriter::new(stream);

            let statistics = self.statistics.lock().clone();
            if let Err(err) = writer.write_all(&statistics.to_vec()) {
                error!("Failed to send statistics: {err}");
            }
        }
    }
}
