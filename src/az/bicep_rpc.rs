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

    pub async fn compile(&mut self, path: &str) -> anyhow::Result<String> {
        let params = json!({
            "path": path
        });

        let result = self.req("bicep/compile", params).await?;
        let contents = result["contents"].as_str().with_context(|| "No content")?;

        Ok(contents.into())
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
