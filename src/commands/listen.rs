use rdev::{Event, EventType};

pub fn listen() {
    if let Err(error) = rdev::listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    let Event { event_type, .. } = event;

    match event_type {
        EventType::KeyPress(key) => println!("press key: {key:?}"),
        _ => (),
    }
}
