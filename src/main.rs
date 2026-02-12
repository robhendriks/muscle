use clap::Parser;
use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};

use crate::cli::Cli;

mod az;
mod cli;
mod command;
mod core;
mod json_rpc;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger_init();

    let cli = Cli::parse();

    cli.execute().await
}

fn logger_init() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    )])
    .unwrap();
}
