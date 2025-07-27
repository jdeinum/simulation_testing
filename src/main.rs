mod broadcast;
mod node;
mod simulation;
use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use clap::Parser;

use crate::{node::Node, simulation::RealWorldFunctionality};
use serde::Deserialize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The config file we are using for this instance
    #[arg(short, long = "config")]
    config: String,
}

#[derive(Deserialize, Debug)]
pub struct NodeSettings {
    pub id: String,
    pub endpoint: String,
    pub peers: HashMap<String, String>,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // parse the cli
    let cli: Cli = Cli::parse();

    // parse the program settings
    let f = tokio::fs::read_to_string(&cli.config)
        .await
        .context("load settings")?;
    let settings: NodeSettings = toml::from_str(&f).context("get settings from string")?;

    // real world setting
    let real_world_sim = RealWorldFunctionality::build(&settings.endpoint, settings.peers.clone())
        .await
        .context("build sim layer")?;

    // create the node
    let node = Node::build(Path::new(&cli.config), real_world_sim)
        .await
        .context("build node")?;

    // run
    node.run().await.context("run node")?;

    Ok(())
}
