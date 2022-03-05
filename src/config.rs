use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::filesystem;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pid: PathBuf,
    log: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let config_dir = filesystem::config_dir();
        let local_dir = filesystem::local_dir();

        let pid = config_dir.join("oops.pid");
        let log = local_dir.join("oops.log");

        Config { pid, log }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_default_config() {
        let config = Config::default();
        println!("{config:?}");
    }
}
