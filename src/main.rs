use clap::Parser;

use crate::cli::Cli;

mod az;
mod cli;
mod command;
mod core;
mod json_rpc;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let cli = Cli::parse();

    cli.execute().await
}
