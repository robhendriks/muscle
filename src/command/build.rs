use clap::Args;

use crate::{
    az::{bicep_cli, bicep_rpc::BicepJsonRpcClient},
    cli::Cli,
    core::domain,
    json_rpc::JsonRpcServer,
};

#[derive(Debug, Args)]
pub struct BuildArgs {}

impl BuildArgs {
    pub async fn execute(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut project = domain::Project::from_path(&cli.root);

        project.init().await?;
        project.discover_modules().await?;

        let server = JsonRpcServer::bind("127.0.0.1:0").await?;
        log::debug!("Starting JSON RPC socket on port {}", server.port());

        log::debug!("Connecting Bicep JSON RPC to socket");
        let _bicep_process = bicep_cli::json_rpc(server.port()).await?;

        let connection = server.accept().await?;
        let mut client = BicepJsonRpcClient::from(connection);

        let version = client.version().await?;
        log::debug!("Using Bicep CLI version {}", version);

        for module in &project.modules {
            let compile_file = module.main_file().to_str().unwrap();
            log::info!("Compiling {}", compile_file);

            let compile_result = client.compile(compile_file).await?;
            let compile_output_file = module.path.join("main.json");

            // Write to output
            tokio::fs::write(compile_output_file, compile_result.as_bytes()).await?;
        }

        Ok(())
    }
}
