use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[inline]
pub fn home_dir() -> PathBuf {
    env!("HOME").into()
}

#[inline]
pub fn oops_dir() -> PathBuf {
    home_dir().join(".oops")
}

pub fn drop_socket_file<P: AsRef<Path>>(filename: P) -> io::Result<()> {
    let filename = filename.as_ref();
    if filename.exists() {
        fs::remove_file(filename)?;
    }
    Ok(())
}
