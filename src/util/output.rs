use clap::{Args, ValueEnum};
use serde::Serialize;

#[derive(Debug, Args)]
pub struct OutputArgs {
    #[arg(short, long, default_value = "json")]
    output_type: OutputType,
}

#[derive(Debug, Clone, ValueEnum, Eq, PartialEq)]
pub enum OutputType {
    Json,
}

pub fn write<T>(data: T, args: &OutputArgs) -> anyhow::Result<()>
where
    T: Serialize,
{
    // TODO: support other output types
    let str = match args.output_type {
        OutputType::Json => serde_json::to_string::<T>(&data)?,
    };

    println!("{}", str);

    Ok(())
}
