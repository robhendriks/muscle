use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectJson {}

impl ProjectJson {
    pub fn get_path(path: impl AsRef<Path>) -> PathBuf {
        static FILE: &'static str = "muscle.json";
        path.as_ref().join(FILE)
    }
}
