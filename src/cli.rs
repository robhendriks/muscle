use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::command::init::InitArgs;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, env = "MUSCLE_ROOT", default_value = ".")]
    pub root: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn execute(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Init(args) => args.execute(self).await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init(InitArgs),
}
