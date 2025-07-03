use crate::{broadcast::BroadcastLayer, simulation::Simulation};
use anyhow::Result;
use bytes::Bytes;

pub struct Node<S: Simulation> {
    id: String,
    sequence_number: usize,
    log: Vec<String>,
    broadcast_layer: BroadcastLayer<S>,
}

impl<S: Simulation> Node<S> {
    // create a new node
    pub fn new(id: String) -> Self {
        Self {
            id,
            sequence_number: 0,
            log: vec![],
            broadcast_layer: BroadcastLayer {
                peers: todo!(),
                buf: todo!(),
                s: todo!(),
            },
        }
    }

    // handle receiving a message from a node
    pub fn handle_message(&mut self, message: Bytes) -> Result<()> {
        Ok(())
    }
}
