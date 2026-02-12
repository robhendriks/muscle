use clap::Args;

use crate::{az::bicep_rpc::BicepJsonRpcClient, cli::Cli, core::domain, json_rpc::JsonRpcServer};

#[derive(Debug, Args)]
pub struct BuildArgs {}

impl BuildArgs {
    pub async fn execute(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut project = domain::Project::from_path(&cli.root);

        project.init().await?;
        project.discover_modules().await?;

        let server = JsonRpcServer::bind("127.0.0.1:1337").await?;
        log::debug!("JSON RPC server listening on port {}", server.port());

        let connection = server.accept().await?;
        let mut client = BicepJsonRpcClient::from(connection);

        let version = client.version().await?;
        log::debug!("Using Bicep version {}", version);

        for module in &project.modules {
            let format_file = module.main_file().to_str().unwrap();
            let format_result = client.format(format_file).await?;
            println!("{}", format_result);

            // TODO: replace format with build!
        }

        Ok(())
    }
}
