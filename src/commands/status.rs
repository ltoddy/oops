use std::io::{BufReader, Read};
use std::os::unix::net::UnixStream;

use anyhow::{Context, Result};

use crate::common::Statistics;
use crate::filesystem;

pub fn status() -> Result<()> {
    let oops_dir = filesystem::oops_dir();
    let socket_path = oops_dir.join("oops.sock");

    let socket = UnixStream::connect(socket_path).with_context(|| {
        "Can't connect oops, make sure the `oops listen` command is executed before this."
    })?;
    let mut reader = BufReader::new(socket);
    let mut buffer = Vec::<u8>::with_capacity(1024);

    // TODO: error handle
    let _ = reader.read_to_end(&mut buffer).with_context(|| "Failed to read, please try again.");
    let statistics = Statistics::from_bytes(&buffer).unwrap();
    println!("{}", statistics.pretty_format());
    println!("Total number of keyboard presses is: {}", statistics.total_presses());
    Ok(())
}
