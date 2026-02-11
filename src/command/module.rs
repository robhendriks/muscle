use std::path::PathBuf;

use anyhow::anyhow;
use clap::{Args, Subcommand};

use crate::{
    cli::Cli,
    core::{
        domain,
        json::{self, JsonContainer},
        module_config::ModuleCfg,
    },
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
            ModuleCommands::Build(args) => {
                let module = project.find_module(&args.name);

                match module {
                    Some(module) => {
                        let main_file = module.main_file();
                        log::info!("[BUILD] {}", main_file.display());
                        Ok(())
                    }
                    None => Err(anyhow!("Module '{}' not found", args.name)),
                }
            }
            ModuleCommands::Init(args) => {
                let cfg_path = ModuleCfg::get_path(&args.path);
                let cfg_container = JsonContainer::from(
                    &cfg_path,
                    ModuleCfg {
                        schema: json::get_schema_url("module.json"),
                        name: args.name.to_string(),
                        description: args.description.to_string(),
                        authors: vec![args.author.to_string()],
                        version: args.version.to_string(),
                        main: String::from("main.bicep"),
                        tags: vec![],
                    },
                );

                let result = cfg_container.write_safe(args.force).await?;

                log::info!("[{:?}] {}", result, cfg_path.display());

                Ok(())
            }
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
    #[command(alias = "b")]
    Build(BuildArgs),
    #[command(alias = "i")]
    Init(InitArgs),
    #[command(alias = "s")]
    Show(ShowArgs),
    #[command(alias = "ls")]
    List(ListArgs),
}

#[derive(Debug, Args)]
struct BuildArgs {
    name: String,
}

#[derive(Debug, Args)]
struct InitArgs {
    name: String,

    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    #[arg(short, long, default_value_t = false)]
    force: bool,

    #[arg(short, long, default_value = "")]
    description: String,

    #[arg(short, long, default_value = "")]
    author: String,

    #[arg(short, long, default_value = "0.1.0")]
    version: String,
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
