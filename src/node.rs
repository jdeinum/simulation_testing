use crate::{broadcast::BroadcastLayer, simulation::Simulation};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Duration;
use tokio::select;

#[derive(Deserialize, Debug)]
pub struct NodeSettings {
    pub id: String,
    pub endpoint: String,
    pub peers: HashMap<String, String>,
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
    pub async fn build(config_path: &Path, simulation: S) -> Result<Self> {
        // get our settings
        let f = tokio::fs::read_to_string(config_path)
            .await
            .context("read settings to string")?;
        let settings: NodeSettings = toml::from_str(&f).context("parse settings from string")?;

        // build our broadcast layer
        let peer_ids: HashSet<String> = settings.peers.keys().cloned().collect();
        let broadcast_layer = BroadcastLayer::build(peer_ids, simulation)
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
        println!("[Node {}] Starting up...", self.id);
        
        // Wait for all nodes to be ready
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("[Node {}] Ready to broadcast", self.id);
        
        loop {
            // 1 of 2 things can happen:
            // 1. We generate a local event that we want to broadcast to other nodes
            // 2. We receive a message from another node that we need to store in our log
            select! {

                o = self.broadcast_layer.receive() => {
                    if let Ok(Some(message)) = o {
                        // Process received message
                        if let Ok(msg_str) = String::from_utf8(message.to_vec()) {
                            let trimmed = msg_str.trim();
                            if !trimmed.is_empty() {
                                println!("[Node {}] Received broadcast: {}", self.id, trimmed);
                                self.log.push(trimmed.to_string());
                            }
                        }
                    }
                }

                _ = tokio::time::sleep(Duration::from_secs(3)) => {
                    // send message every 3 seconds for easier observation
                    let message = format!("Message {} from node {}", self.sequence_number, self.id);
                    self.sequence_number += 1;
                    
                    println!("[Node {}] Initiating broadcast: {}", self.id, message);
                    if let Err(e) = self.broadcast_layer.broadcast(&message).await {
                        eprintln!("[Node {}] Failed to broadcast: {}", self.id, e);
                    } else {
                        println!("[Node {}] Broadcast complete", self.id);
                        self.log.push(message);
                    }
                }


            }
        }
    }
}
