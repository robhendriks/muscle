use anyhow::Context;
use tokio::process::Child;

pub async fn json_rpc(port: u16) -> anyhow::Result<Child> {
    let port_str = port.to_string();

    let child = tokio::process::Command::new("bicep")
        .args(["jsonrpc", "--socket", &port_str])
        .kill_on_drop(true)
        .spawn()
        .with_context(|| "Failed to execute Bicep CLI command")?;

    Ok(child)
}
