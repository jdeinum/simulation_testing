use std::{sync::Arc, time::Duration};

use crate::{broadcast::BroadcastLayer, simulation::Simulation};
use anyhow::{Context, Result};
use bytes::Bytes;
use tokio::sync::Mutex;
use tracing::{info, instrument};

pub struct Node<S: Simulation> {
    id: String,
    sequence_number: usize,
    broadcast_layer: BroadcastLayer<S>,
}

impl<S: Simulation> Node<S> {
    // create a new node
    pub fn new(id: String, broadcast_layer: BroadcastLayer<S>) -> Self {
        Self {
            id,
            sequence_number: 0,
            broadcast_layer,
        }
    }

    // handle receiving a message from a node
    #[instrument(skip_all, fields(node = self.id))]
    pub fn handle_message(&mut self, message: Bytes) -> Result<()> {
        // convert the message to a string
        let s = String::from_utf8(message.as_ref().to_vec()).context("convert bytes to string")?;
        info!("received {s}");
        Ok(())
    }

    pub async fn run(mut self) -> Result<()> {
        // spawn a simple handler that prints messages received from the broadcast layer
        tokio::spawn(async || {
            loop {
                let s = self
                    .broadcast_layer
                    .receive()
                    .await
                    .context("receive message from broadcast layer")?;

                self.handle_message(s).context("handle message")?;
            }
        });

        // generate an event every second, and broadcast it
        loop {
            self.broadcast_layer
                .broadcast(format!("{}-{}", self.id, self.sequence_number).as_bytes())
                .await
                .context("broadcast message")?;
            self.sequence_number += 1;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
