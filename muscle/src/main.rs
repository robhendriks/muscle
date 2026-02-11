use clap::Parser;

use crate::cli::Cli;

mod cli;
mod command;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let cli = Cli::parse();

    cli.execute().await
}
