use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::command::{build::BuildArgs, health::HealthArgs, init::InitArgs, module::ModuleArgs};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(long = "root", env = "MUSCLE_ROOT", global = true, default_value = ".")]
    pub root: PathBuf,

    #[arg(
        long = "debug",
        env = "MUSCLE_DEBUG",
        global = true,
        default_value_t = false
    )]
    pub debug: bool,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn execute(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Build(args) => args.execute(self).await,
            Commands::Init(args) => args.execute(self).await,
            Commands::Module(args) => args.execute(self).await,
            Commands::Health(args) => args.execute(self).await,
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

    #[command(alias = "h")]
    Health(HealthArgs),
}
