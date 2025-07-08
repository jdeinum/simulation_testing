use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use simulation_testing::run;

#[derive(Deserialize, Debug)]
pub struct Peers {
    pub peers: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short = 'c', long = "--config")]
    config: String,

    #[arg(short = 'e', long = "--endpoint")]
    endpoint: String,

    #[arg(short = 'p', long = "--port")]
    port: u16,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // parse cli
    let cli = Cli::parse();

    // read peers from file
    let s = tokio::fs::read_to_string(cli.config)
        .await
        .context("read file")?;
    let peers = toml::from_str(&s).context("read peers")?;

    // run the node
    run(&cli.endpoint, cli.port, peers)
        .await
        .context("run node")?;

    Ok(())
}
