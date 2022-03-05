pub mod cli;
pub mod commands;
pub mod common;
pub mod config;
pub mod filesystem;
pub mod interactive;

use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger, TerminalMode};

use crate::cli::{Cli, SubCommands};

pub fn main() {
    initialize_logger(LevelFilter::Debug);

    let Cli { subcommand } = Cli::parse();

    match subcommand {
        SubCommands::Init => crate::commands::initialize(),
        SubCommands::Status => crate::commands::status(),
        SubCommands::Listen => crate::commands::listen(),
    };
}

fn initialize_logger(level: LevelFilter) {
    TermLogger::init(level, Default::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
}
