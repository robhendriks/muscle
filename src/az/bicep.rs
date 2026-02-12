use std::vec;

use serde_json::json;
use tokio::io::{AsyncWriteExt, BufReader};

use crate::az::json_rpc;

pub struct BicepRpcClient {}

impl BicepRpcClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let mut rpc_server = json_rpc::Server::new("127.0.0.1:1234").await?;
        let mut rpc_conn = rpc_server.accept().await?;

        let req = json_rpc::Request {
            jsonrpc: "2.0",
            id: 1,
            method: "bicep/compile".into(),
            params: json!({
                "path": "/home/rob_hendriks/Git/devops/infrastructure/azure/aks/bicep/main.bicep"
            }),
        };

        for _ in 0..10 {
            let res = rpc_conn.request(&req).await?;
            println!("{:?}", res);
        }

        Ok(())
    }
}
