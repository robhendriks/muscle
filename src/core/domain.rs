use std::{
    cell::OnceCell,
    path::{Path, PathBuf},
};

use glob::glob;
use serde::Serialize;

use crate::core::{json::JsonContainer, module_config::ModuleCfg, project_config::ProjectCfg};

#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub cfg: JsonContainer<ProjectCfg>,
    pub modules: Vec<Module>,
}

impl Project {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            cfg: JsonContainer::new(ProjectCfg::get_path(path)),
            modules: Vec::new(),
        }
    }

    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.cfg.read().await
    }

    pub async fn discover_modules(&mut self) -> anyhow::Result<()> {
        let module_glob = self.path.join("**/module.json");
        let module_glob = module_glob.to_str().unwrap();

        for module_entry in glob(module_glob)?.flatten() {
            let module_dir = module_entry.parent().unwrap();
            let mut module = Module::new(module_dir);

            let module_init = module.init().await;
            match module_init {
                Ok(_) => {
                    self.modules.push(module);
                }
                Err(_) => {
                    simplelog::warn!("Failed to init module {}", module_entry.display());
                }
            }
        }

        Ok(())
    }

    pub fn find_module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|m| {
            let module_name = &m.get_cfg().name;
            module_name.to_lowercase() == name.to_lowercase()
        })
    }

    pub fn module_views(&self, parent: &Project) -> Vec<ModuleView<'_>> {
        self.modules.iter().map(|m| m.to_view(parent)).collect()
    }
}

#[derive(Debug)]
pub struct Module {
    pub path: PathBuf,
    pub cfg: JsonContainer<ModuleCfg>,
    main: OnceCell<PathBuf>,
    files: Vec<PathBuf>,
}

impl Module {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            cfg: JsonContainer::new(ModuleCfg::get_path(path)),
            main: OnceCell::new(),
            files: Vec::new(),
        }
    }

    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.cfg.read().await?;
        self.discover_files()
    }

    pub fn to_view(&self, parent: &Project) -> ModuleView<'_> {
        let path = self.path.strip_prefix(&parent.path).unwrap();

        let cfg = self.get_cfg();
        ModuleView {
            name: &cfg.name,
            description: &cfg.description,
            version: &cfg.version,
            authors: &cfg.authors,
            main: &cfg.main,
            tags: &cfg.tags,
            path,
            files: &self.files,
        }
    }

    pub fn main_file(&self) -> &Path {
        self.main.get_or_init(|| {
            let main_file = &self.get_cfg().main;
            self.path.join(main_file)
        })
    }

    fn discover_files(&mut self) -> anyhow::Result<()> {
        let pattern = self.path.join("**/*.bicep");

        for file in glob::glob(pattern.to_str().unwrap())?.flatten() {
            let file_rel = file.strip_prefix(&self.path).unwrap();
            self.files.push(file_rel.to_path_buf());
        }

        Ok(())
    }

    fn get_cfg(&self) -> &ModuleCfg {
        self.cfg.data.as_ref().unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct ModuleView<'m> {
    pub name: &'m str,
    pub description: &'m str,
    pub authors: &'m [String],
    pub version: &'m str,
    pub main: &'m str,
    pub tags: &'m [String],
    pub path: &'m Path,
    pub files: &'m [PathBuf],
}
