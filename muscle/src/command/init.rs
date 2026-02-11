use std::path::{Path, PathBuf};

use clap::Args;
use muscle_core::{json::JsonContainer, module::ModuleJson, project::ProjectJson};

use crate::cli::Cli;

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

impl InitArgs {
    pub async fn execute(&self, cli: &Cli) -> anyhow::Result<()> {
        init_project(&cli.root, self.force).await?;
        init_modules(&cli.root, self.force).await
    }
}

async fn init_project(root: &Path, overwrite: bool) -> anyhow::Result<()> {
    let json_path = ProjectJson::get_path(root);
    let json_c = JsonContainer::from(&json_path, ProjectJson {});

    let result = json_c.write_safe(overwrite).await?;

    log::info!("[{:?}] {}", result, json_path.display());
    Ok(())
}

async fn init_modules(root: &Path, overwrite: bool) -> anyhow::Result<()> {
    let module_discover_path = root.join("**/bicep/main.bicep");
    let module_discover_str = module_discover_path.to_str().unwrap();

    let module_paths = discover_modules(module_discover_str)?;

    for (module_path, module_main) in module_paths {
        let module_main_file_name = module_main.file_name().unwrap();

        let json_path = ModuleJson::get_path(&module_path);
        let json_c = JsonContainer::from(
            &json_path,
            ModuleJson {
                name: String::from("my-module"),
                description: String::from("My module"),
                authors: vec![String::from("John Doe")],
                version: String::from("0.1.0"),
                main: String::from(module_main_file_name.to_str().unwrap()),
            },
        );

        let result = json_c.write_safe(overwrite).await?;

        log::info!("[{:?}] {}", result, json_path.display());
    }

    Ok(())
}

fn discover_modules(pattern: &str) -> anyhow::Result<Vec<(PathBuf, PathBuf)>> {
    let mut results: Vec<(PathBuf, PathBuf)> = Vec::new();

    for entry in glob::glob(pattern)? {
        if let Ok(module_path) = entry {
            if let Some(module_parent) = module_path.parent() {
                results.push((module_parent.to_path_buf(), module_path));
            }
        }
    }

    Ok(results)
}
