use clap::Parser;
use clap_adapters::prelude::*;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut stream = cli.config.stream();
    while let Some(config) = stream.next().await {
        println!("{:?}", config.data.0)
    }
    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    config: Reloading<PathTo<JsonOf<serde_json::Value>>>,
}
