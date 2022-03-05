use rdev::{listen, Event, EventType};

fn callback(event: Event) {
    let Event { event_type, .. } = event;

    match event_type {
        EventType::KeyPress(key) => println!("press key: {key:?}"),
        _ => (),
    }
}

pub fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
