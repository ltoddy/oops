use std::io::{BufReader, Read};
use std::os::unix::net::UnixStream;

use crate::common::SOCKET_PATH;

pub fn status() {
    let socket = UnixStream::connect(SOCKET_PATH).unwrap();
    let mut buffer = Vec::<u8>::with_capacity(1024);

    let mut reader = BufReader::new(socket);
    let _ = reader.read_to_end(&mut buffer);
    println!("{}", String::from_utf8_lossy(&buffer));
}
