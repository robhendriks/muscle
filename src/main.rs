use clap::Parser;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
};

use crate::cli::Cli;

mod az;
mod cli;
mod command;
mod core;
mod json_rpc;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    logger_init(cli.debug);

    cli.execute().await
}

fn logger_init(debug: bool) {
    let log_level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let logger = TermLogger::new(
        log_level,
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    );

    CombinedLogger::init(vec![logger]).unwrap();
}
