pub mod cli;
pub mod commands;
pub mod common;
pub mod filesystem;
pub mod interactive;
pub mod server;

use std::fs::OpenOptions;

use anyhow::Result;
use clap::Parser;
use log::{LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

use crate::cli::{Cli, SubCommands};
use crate::filesystem::oops_dir;

pub fn main() -> Result<()> {
    initialize_logger();

    let Cli { subcommand } = Cli::parse();

    match subcommand {
        SubCommands::Status => crate::commands::status(),
        SubCommands::Listen => crate::commands::listen(),
    }
}

fn initialize_logger() {
    let log_path = oops_dir().join("oops.log");
    let log_file = OpenOptions::new().create(true).write(true).read(true).open(log_path).unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
    ])
    .unwrap();
}
