use std::sync::Arc;
use std::thread;

use anyhow::Result;
use log::error;

use crate::filesystem;
use crate::server::Listener;

pub fn listen() -> Result<()> {
    let oops_dir = filesystem::oops_dir();
    let socket_path = oops_dir.join("oops.sock");

    let mut listener = Arc::new(Listener::bind(socket_path)?);
    {
        let listener = Arc::clone(&listener);
        thread::spawn(move || listener.serve());
    }
    if let Err(err) = rdev::listen(move |event| listener.on_keyboard_press(event)) {
        error!("Failed to monitor keyboard: {err:?}");
    }
    Ok(())
}
