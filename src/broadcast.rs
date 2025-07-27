use crate::simulation::Simulation;
use anyhow::{Context, Result};
use bytes::Bytes;
use std::collections::HashSet;

pub struct BroadcastLayer<S: Simulation> {
    pub peers: HashSet<String>,
    pub messages: Vec<String>,
    pub s: S,
    pub current_seq_number: usize,
}

impl<S: Simulation> BroadcastLayer<S> {
    pub async fn build(peers: HashSet<String>, s: S) -> Result<Self> {
        Ok({
            Self {
                peers,
                s,
                messages: vec![],
                current_seq_number: 0,
            }
        })
    }
}

impl<S: Simulation> BroadcastLayer<S> {
    pub async fn broadcast(&mut self, message: &str) -> Result<()> {
        println!("[BroadcastLayer] Broadcasting message to {} peers: {}", self.peers.len(), message);
        
        // for each peer, we send the message
        for peer in &self.peers {
            println!("[BroadcastLayer] Sending to peer: {}", peer);
            self.s
                .send_message(peer, message.as_bytes())
                .await
                .context("send message to peer")?;
        }
        
        self.current_seq_number += 1;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Option<Bytes>> {
        let b = self.s.receive_message().await.context("receive message")?;
        if let Some(bytes) = b {
            if !bytes.is_empty() {
                let msg = String::from_utf8(bytes.to_vec()).context("parse utf8 string from bytes")?;
                let trimmed = msg.trim();
                if !trimmed.is_empty() {
                    println!("[BroadcastLayer] Received message: {}", trimmed);
                    self.messages.push(trimmed.to_string());
                }
                Ok(Some(bytes))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
