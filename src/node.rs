use crate::{broadcast::BroadcastLayer, simulation::Simulation};
use anyhow::{Context, Result};
use bytes::Bytes;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Deserialize, Debug)]
pub struct NodeSettings {
    id: String,
    peers: HashMap<String, String>,
}

/// A node in our system
/// In the real world, these could be on different machines
/// In our case for testing, multiple nodes are run on the same machine
pub struct Node<S: Simulation> {
    id: String,
    sequence_number: usize,
    log: Vec<String>,
    broadcast_layer: BroadcastLayer<S>,
}

impl<S: Simulation> Node<S> {
    // When a node is created, it needs explicit knowledge of its peers so it can open connections
    // to them
    pub async fn build(config_path: &Path) -> Result<Self> {
        // get our settings
        let f = tokio::fs::read_to_string(config_path)
            .await
            .context("read settings to string")?;
        let settings: NodeSettings = toml::from_str(&f).context("parse settings from string")?;

        // build our broadcast layer
        let broadcast_layer = BroadcastLayer::build(settings.peers)
            .await
            .context("build broadcast layer")?;

        // build
        Ok(Self {
            id: settings.id,
            sequence_number: 0,
            log: vec![],
            broadcast_layer,
        })
    }

    // Run the node
    pub async fn run(mut self) -> Result<()> {
        // 1 of 2 things can happen:
        // 1. We generate a local event that we want to broadcast to other nodes
        // 2. We receive a message from another node that we need to store in our log
        todo!()
    }

    // handle receiving a message from a node
    pub fn handle_message(&mut self, message: Bytes) -> Result<()> {
        todo!()
    }
}
