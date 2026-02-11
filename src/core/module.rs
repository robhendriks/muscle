use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleJson {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub version: String,
    pub main: String,
    pub tags: Vec<String>,
}

impl ModuleJson {
    pub fn get_path(path: impl AsRef<Path>) -> PathBuf {
        static FILE: &str = "module.json";
        path.as_ref().join(FILE)
    }
}
