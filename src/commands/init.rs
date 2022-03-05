use configit::Storage;
use log::{info, warn};

use crate::config::Config;
use crate::filesystem::config_dir;

pub fn initialize() {
    let config = Config::default();
    let filename = config_dir().join("config.toml");

    match config.store(&filename) {
        Ok(_) => info!("Created configuration file: {}", filename.display()),
        Err(err) => warn!("Failed to create configuration file: {err}"),
    };
}
