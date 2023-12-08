use clap::Parser;
use clap_adapters::{HttpGet, JsonOf, PathTo, Reloading};
use serde::Deserialize;
use tokio_stream::StreamExt;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    // println!("{}", cli.doc.data.0);
    println!("{}", cli.doc.data);
    // let mut stream = cli.config.stream();
    // while let Some(config) = stream.next().await {
    //     println!("{:?}", config.data.0)
    // }
    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    // #[clap(long)]
    // config: Reloading<PathTo<JsonOf<MyConfig>>>,

    // #[clap(long)]
    // doc: HttpGet<JsonOf<serde_json::Value>>,
    #[clap(long)]
    doc: HttpGet<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct MyConfig {
    apple: String,
    banana: String,
}
