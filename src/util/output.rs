use clap::Args;
use serde::Serialize;

#[derive(Debug, Args)]
pub struct OutputArgs {
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

pub fn write_json<T>(data: T, args: &OutputArgs) -> anyhow::Result<()>
where
    T: Serialize,
{
    let to_str_fn = if args.pretty {
        serde_json::to_string_pretty::<T>
    } else {
        serde_json::to_string::<T>
    };

    let str = to_str_fn(&data)?;
    println!("{}", str);

    Ok(())
}
