use anyhow::{Context, Result};

use crate::{broadcast::BroadcastLayer, node::Node, simulation::RealWorldFunctionality};

pub async fn run(address: &str, port: u16, peers: Vec<String>, id: String) -> Result<()> {
    // create simulation
    let sim_layer = RealWorldFunctionality {
        peers: todo!(),
        message_buffer: todo!(),
    };

    // create broadcast layer
    let broadcast_layer = BroadcastLayer {
        peers: todo!(),
        buf: todo!(),
        s: sim_layer,
    };

    // create node
    let node = Node::new(id, broadcast_layer);

    // run the node
    node.run().await.context("run node to completion")?;

    Ok(())
}
