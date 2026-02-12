use clap::Args;

use crate::{az::bicep::BicepRpcClient, cli::Cli, core::domain};

#[derive(Debug, Args)]
pub struct BuildArgs {}

impl BuildArgs {
    pub async fn execute(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut project = domain::Project::from_path(&cli.root);

        project.init().await?;
        project.discover_modules().await?;

        let mut rpc_client = BicepRpcClient::new();
        rpc_client.start().await?;

        for module in &project.modules {
            let build_file = module.main_file();
            // let build_result = BicepClient::build(&build_file).await;

            // match build_result {
            //     Ok(_) => {
            //         println!("OK");
            //     }
            //     Err(_) => {
            //         println!("FAIL");
            //     }
            // }
        }

        Ok(())
    }
}
