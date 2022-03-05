use std::collections::BTreeMap;
use std::io::{BufWriter, Write};
use std::os::unix::net::UnixListener;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;
use std::thread;

use log::{error, info};
use parking_lot::Mutex;
use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};

use crate::common::SOCKET_PATH;

pub fn listen() {
    let (sender, receiver) = sync_channel::<Key>(1024);
    let presses = Arc::new(Mutex::new(Presses::default()));

    {
        let presses = Arc::clone(&presses);
        thread::spawn(move || receive_keyboard_press(receiver, presses));
    }
    {
        let presses = Arc::clone(&presses);
        thread::spawn(move || serve(Arc::clone(&presses)));
    }
    // This will block.
    if let Err(err) = rdev::listen(on_keyboard_press(sender)) {
        error!("Failed to monitor keyboard: {err:?}");
    }
}

fn on_keyboard_press(sender: SyncSender<Key>) -> impl Fn(Event) {
    move |event: Event| {
        let Event { event_type, .. } = event;

        if let EventType::KeyPress(key) = event_type {
            let _ = sender.send(key).map_err(|err| error!("failed to send key: {err}"));
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Presses(BTreeMap<String, usize>);

impl Presses {
    fn press(&mut self, key: Key) {
        let counter = self.0.entry(format!("{key:?}")).or_insert(0);
        *counter += 1;
    }
}

fn receive_keyboard_press(receiver: Receiver<Key>, presses: Arc<Mutex<Presses>>) {
    for key in receiver {
        presses.lock().press(key);
    }
}

// For inter-process communication
fn serve(presses: Arc<Mutex<Presses>>) {
    let listener = UnixListener::bind(SOCKET_PATH).unwrap();
    info!("server start");

    for stream in listener.incoming().flatten() {
        info!("connection received");
        let mut writer = BufWriter::new(stream);

        let presses = presses.lock();
        let _ = writer.write_all(&serde_json::to_vec(&*presses).unwrap());
    }
}
