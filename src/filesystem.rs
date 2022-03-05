use std::path::PathBuf;

#[inline]
pub fn home_dir() -> PathBuf {
    env!("HOME").into()
}

#[inline]
pub fn config_dir() -> PathBuf {
    home_dir().join(".config")
}

#[inline]
pub fn local_dir() -> PathBuf {
    home_dir().join(".local")
}
