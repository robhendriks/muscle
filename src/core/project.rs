use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const SCHEMA_URL: &str =
    "https://git.robhendriks.dev/rob/muscle/raw/branch/main/schemas/project.json";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectJson {
    #[serde(rename = "$schema")]
    pub schema: String,
}

impl ProjectJson {
    pub fn get_path(path: impl AsRef<Path>) -> PathBuf {
        static FILE: &'static str = "muscle.json";
        path.as_ref().join(FILE)
    }
}
