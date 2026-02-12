use anyhow::{Context, Ok, anyhow};
use serde_json::{Value, json};

use crate::json_rpc::JsonRpcConnection;

pub struct BicepJsonRpcClient {
    connection: JsonRpcConnection,
}

impl BicepJsonRpcClient {
    pub fn from(connection: JsonRpcConnection) -> Self {
        Self { connection }
    }

    pub async fn version(&mut self) -> anyhow::Result<String> {
        let result = self.req("bicep/version", json!({})).await?;
        let version_string = result["version"].as_str().with_context(|| "No content")?;
        Ok(version_string.into())
    }

    pub async fn format(&mut self, path: &str) -> anyhow::Result<String> {
        let params = json!({
            "path": path
        });

        let result = self.req("bicep/format", params).await?;
        let contents = result["contents"].as_str().with_context(|| "No content")?;

        Ok(contents.into())
    }

    pub async fn compile(&mut self, path: &str) -> anyhow::Result<BicepCompileResult> {
        let params = json!({
            "path": path
        });

        let result = self.req("bicep/compile", params).await?;

        let Some(contents) = result["contents"].as_str() else {
            let diagnostics = result["diagnostics"].as_array().unwrap();

            // TODO: improve mapping and move to func
            let errors: Vec<BicepCompileError> = diagnostics
                .iter()
                .map(|f| BicepCompileError {
                    code: f["code"].as_str().unwrap().into(),
                    level: f["level"].as_str().unwrap().into(),
                    message: f["message"].as_str().unwrap().into(),
                    source: f["source"].as_str().unwrap().into(),
                })
                .collect();

            return Ok(BicepCompileResult::Error(errors));
        };

        Ok(BicepCompileResult::Ok(contents.into()))
    }

    async fn req(&mut self, method: &str, params: Value) -> anyhow::Result<Value> {
        let response = self.connection.send(method, params).await?;

        if let Some(error) = &response.error {
            return Err(anyhow!(
                "JSON RPC Error ({}): {}",
                error["code"],
                error["message"]
            ));
        }

        let Some(result) = response.result else {
            return Err(anyhow!("Empty response"));
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct BicepCompileError {
    pub code: String,
    pub level: String,
    pub message: String,
    pub source: String,
}

#[derive(Debug)]
pub enum BicepCompileResult {
    Ok(String),
    Error(Vec<BicepCompileError>),
}
