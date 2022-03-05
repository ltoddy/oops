use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

use log::error;
use rdev::{Event, EventType, Key};

pub fn listen() {
    let (sender, receiver) = sync_channel::<Key>(1024);

    thread::spawn(move || receive_keyboard_press(receiver));
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

fn receive_keyboard_press(receiver: Receiver<Key>) {
    let mut monitor = HashMap::<String, usize>::with_capacity(36);

    for key in receiver {
        let counter = monitor.entry(format!("{key:?}")).or_insert(0);
        *counter += 1;
        println!("monitor is: {monitor:?}");
    }
}
