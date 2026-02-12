use clap::Args;

use crate::{
    az::{
        bicep_cli,
        bicep_rpc::{BicepCompileResult, BicepJsonRpcClient},
    },
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
        simplelog::debug!("Starting JSON RPC socket on port <u>{}</>", server.port());

        simplelog::debug!("Connecting Bicep JSON RPC...");
        let _bicep_process = bicep_cli::json_rpc(server.port()).await?;

        let connection = server.accept().await?;
        let mut client = BicepJsonRpcClient::from(connection);

        let version = client.version().await?;
        simplelog::debug!("Using Bicep CLI version <u>{}</u>", version);

        let c = project.modules.len();

        for (i, module) in project.modules.iter().enumerate() {
            let main_file = module.main_file();
            let main_file_rel = main_file.strip_prefix(&cli.root).unwrap();

            simplelog::info!(
                "<d>[{}/{}]</> <b>Compiling:</> {}",
                i + 1,
                c,
                main_file_rel.display()
            );

            let compile_result = client.compile(main_file.to_str().unwrap()).await?;
            let compile_output_file = module.path.join("main.json");

            match compile_result {
                BicepCompileResult::Ok(contents) => {
                    tokio::fs::write(compile_output_file, contents.as_bytes()).await?;
                }
                BicepCompileResult::Error(errs) => {
                    for err in errs {
                        simplelog::info!(
                            "<u>{}</> <b>{}</> {} <d>{}</>",
                            err.code,
                            err.level,
                            err.message,
                            err.source
                        );
                    }

                    simplelog::error!("Error");
                }
            };
        }

        Ok(())
    }
}
