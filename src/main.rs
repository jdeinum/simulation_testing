mod broadcast;
mod node;
mod simulation;
use std::path::Path;

use anyhow::Result;
use clap::Parser;

use crate::{node::Node, simulation::RealWorldFunctionality};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The config file we are using for this instance
    #[arg(short, long = "config")]
    config: String,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // parse the cli
    let cli: Cli = Cli::parse();

    // real world setting
    let real_world_sim = RealWorldFunctionality {
        peers: todo!(),
        message_buffer: todo!(),
    };

    // create the node
    let node: Node = Node::build(&Path::from(&cli.config))
        .await
        .context("build node")?;

    // run
    node.run().await.context("run node")?;

    Ok(())
}
