use std::path::{Path, PathBuf};

use crate::{cli::Cli, util};
use anyhow::Context;
use clap::{Args, builder::Str};
use muscle_core::{json::JsonContainer, module, module::ModuleJson, project, project::ProjectJson};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long, default_value = "**/main.bicep")]
    glob: String,

    #[arg(short, long, default_value_t = false)]
    force: bool,

    #[arg(short, long, default_value = "John Doe")]
    author: String,

    #[arg(short, long, default_value = "0.1.0")]
    version: String,
}

impl InitArgs {
    pub async fn execute(&self, cli: &Cli) -> anyhow::Result<()> {
        init_project(&cli.root, &self).await?;
        init_modules(&cli.root, &self).await
    }
}

async fn init_project(root: &Path, args: &InitArgs) -> anyhow::Result<()> {
    let json_path = ProjectJson::get_path(root);
    let json_c = JsonContainer::from(
        &json_path,
        ProjectJson {
            schema: project::SCHEMA_URL.to_string(),
        },
    );

    let result = json_c.write_safe(args.force).await?;

    log::info!("[{:?}] {}", result, json_path.display());
    Ok(())
}

async fn init_modules(root: &Path, args: &InitArgs) -> anyhow::Result<()> {
    let pattern = root.join(&args.glob);
    let pattern_str = pattern.to_str().unwrap();

    let glob = util::Glob::new(&pattern_str);
    let glob_matches = glob.matches()?;

    for glob_match in glob_matches {
        let module_dir = glob_match.parent().with_context(|| "")?;
        let module_main = glob_match.file_name_str().with_context(|| "")?;

        let components = glob_match.components().with_context(|| "")?;
        let (name, tags) = get_name_and_tags(&components);

        let json_path = ModuleJson::get_path(&module_dir);
        let json_c = JsonContainer::from(
            &json_path,
            ModuleJson {
                schema: module::SCHEMA_URL.to_string(),
                name,
                description: String::from(""),
                authors: vec![args.author.to_string()],
                version: args.version.to_string(),
                main: module_main.to_string(),
                tags,
            },
        );

        let result = json_c.write_safe(args.force).await?;

        log::info!("[{:?}] {}", result, json_path.display());
    }

    Ok(())
}

fn get_name_and_tags(components: &Vec<String>) -> (String, Vec<String>) {
    let name: String = components
        .last()
        .map_or_else(|| String::from(""), |s| s.to_owned());

    let categories: Vec<String> = components
        .iter()
        .rev()
        .skip(1)
        .map(|s| s.to_string())
        .collect();

    (name, categories)
}
