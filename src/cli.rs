use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::command::{build::BuildArgs, init::InitArgs, module::ModuleArgs};

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
            Commands::Build(args) => args.execute(self).await,
            Commands::Init(args) => args.execute(self).await,
            Commands::Module(args) => args.execute(self).await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(alias = "b")]
    Build(BuildArgs),

    #[command(alias = "i")]
    Init(InitArgs),

    #[command(alias = "mod")]
    Module(ModuleArgs),
}
