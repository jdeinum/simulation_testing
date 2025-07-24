use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use rand::SeedableRng;

use crate::{node::Node, simulation::RealWorldFunctionality};

mod node;
mod simulation;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long = "--config")]
    config: String,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // parse the cli
    let cli: Cli = Cli::parse();

    // real world setting
    let real_world_sim = RealWorldFunctionality {};

    // create the node
    let node: Node = Node::build(Path::from(&cli.config))
        .await
        .context("build node")?;

    // run
    node.run().await.context("run node")?;

    Ok(())
}
