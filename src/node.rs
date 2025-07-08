use crate::{broadcast::BroadcastLayer, simulation::Simulation};
use anyhow::{Context, Result};
use bytes::Bytes;
use std::time::Duration;
use tracing::{info, instrument};

pub struct Node<S: Simulation> {
    id: String,
    sequence_number: usize,
    broadcast_layer: BroadcastLayer<S>,
}

impl<S: Simulation + Send + 'static> Node<S> {
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
        tokio::spawn(async move {
            if let Err(e) = async {
                loop {
                    let s = self
                        .broadcast_layer
                        .receive()
                        .await
                        .context("receive message from broadcast layer")?;

                    self.handle_message(s).context("handle message")?;
                }

                #[allow(unreachable_code)]
                Ok::<(), anyhow::Error>(()) // to satisfy the compiler
            }
            .await
            {
                tracing::error!("Node task failed: {e:?}");
            }
        });

        // generate an event every second, and broadcast it
        loop {
            // we expliceitly use SN-ID here so that we can lexigraxically sort these in a priority
            // queue
            self.broadcast_layer
                .broadcast(format!("{}-{}", self.sequence_number, self.id).as_bytes())
                .await
                .context("broadcast message")?;
            self.sequence_number += 1;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
