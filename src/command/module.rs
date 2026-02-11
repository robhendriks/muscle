use anyhow::anyhow;
use clap::{Args, Subcommand};

use crate::{
    cli::Cli,
    core::domain,
    util::{self, output::OutputArgs},
};

#[derive(Debug, Args)]
pub struct ModuleArgs {
    #[command(subcommand)]
    command: ModuleCommands,
}

impl ModuleArgs {
    pub async fn execute(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut project = domain::Project::from_path(&cli.root);

        project.init().await?;
        project.discover_modules().await?;

        match &self.command {
            ModuleCommands::List(args) => {
                util::output::write_json(project.modules_as_json(&project), &args.output)?;
                Ok(())
            }
            ModuleCommands::Show(args) => {
                let module = project.find_module(&args.name);

                match module {
                    Some(module) => {
                        util::output::write_json(module.to_json(&project), &args.output)?;
                        Ok(())
                    }
                    None => Err(anyhow!("Module '{}' not found", args.name)),
                }
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum ModuleCommands {
    #[command(alias = "s")]
    Show(ShowArgs),
    #[command(alias = "ls")]
    List(ListArgs),
}

#[derive(Debug, Args)]
struct ShowArgs {
    name: String,

    #[command(flatten)]
    output: OutputArgs,
}

#[derive(Debug, Args)]
struct ListArgs {
    #[command(flatten)]
    output: OutputArgs,
}
