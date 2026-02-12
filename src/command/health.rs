use crate::cli::Cli;
use clap::Args;

#[derive(Debug, Args)]
pub struct HealthArgs {}

impl HealthArgs {
    pub async fn execute(&self, _cli: &Cli) -> anyhow::Result<()> {
        print_binary_health("-");
        print_binary_health("bicep");

        Ok(())
    }
}

fn binary_ok(name: &str) -> bool {
    which::which(name).is_ok()
}

fn print_binary_health(name: &str) {
    let binary_ok = binary_ok(name);
    let icon = if binary_ok { "✓" } else { "✘" };
    let color = if binary_ok { "green" } else { "red" };
    simplelog::info!("<{}>{} {}</>", color, icon, name)
}
