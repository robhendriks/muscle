use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub struct JsonContainer<T> {
    pub path: PathBuf,
    pub data: Option<T>,
}

impl<T> JsonContainer<T> {
    #[allow(unused)]
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            data: None,
        }
    }

    pub fn from(path: impl AsRef<Path>, data: T) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            data: Some(data),
        }
    }
}

impl<T: DeserializeOwned> JsonContainer<T> {
    #[allow(unused)]
    pub async fn read(&mut self) -> anyhow::Result<()> {
        let path = &self.path;

        let contents = tokio::fs::read(path)
            .await
            .with_context(|| format!("Failed to read JSON file: {}", path.display()))?;

        let result = serde_json::from_slice::<T>(&contents)
            .with_context(|| format!("Failed to deserialize JSON file: {}", &path.display()))?;

        self.data = Some(result);

        Ok(())
    }
}

impl<T: Serialize> JsonContainer<T> {
    pub async fn write(&self) -> anyhow::Result<()> {
        let Some(data) = &self.data else {
            return Err(anyhow!("Container is empty"));
        };

        let path = &self.path;
        let contents = serde_json::to_vec_pretty(&data)
            .with_context(|| format!("Failed to serialize JSON file: {}", path.display()))?;

        tokio::fs::write(&self.path, contents)
            .await
            .with_context(|| format!("Failed to write JSON file: {}", path.display()))?;

        Ok(())
    }

    pub async fn write_safe(&self, overwrite: bool) -> anyhow::Result<WriteResult> {
        let path = &self.path;
        let exists = path.exists();

        if !overwrite && exists {
            return Ok(WriteResult::Ignore);
        }

        self.write().await?;

        Ok(if exists {
            WriteResult::Update
        } else {
            WriteResult::Create
        })
    }
}

#[derive(Debug)]
pub enum WriteResult {
    Create,
    Update,
    Ignore,
}

pub fn get_schema_url(name: &str) -> String {
    let base_url: &'static str = env!("SCHEMA_BASE_URL");
    [base_url, name].join("/")
}
