use crate::simulation::Simulation;
use anyhow::{Context, Result};
use bytes::{BufMut, Bytes, BytesMut};
use futures::future::join_all;
use std::collections::HashMap;
use tokio::{io::AsyncWriteExt, net::TcpStream};

/// The broadcast layer has a few responsibilities:
/// 1. When the application wants to send a message, it needs to
pub struct BroadcastLayer<S: Simulation> {
    pub peers: HashMap<String, TcpStream>,
    pub buf: BytesMut,
    pub s: S,
}

impl<S: Simulation> BroadcastLayer<S> {
    pub async fn broadcast(&mut self, message: &[u8]) -> Result<()> {
        self.buf.clear();
        self.buf.put(message);

        let futures = self
            .peers
            .values_mut()
            .map(|stream| stream.write_all(&self.buf))
            .collect::<Vec<_>>();

        let results = join_all(futures).await;
        let r: Result<(), _> = results.into_iter().collect();

        match r {
            Ok(_) => return Ok(()),
            Err(e) => return Err(anyhow::anyhow!(e)),
        }
    }

    pub async fn receive(&mut self) -> Result<Bytes> {
        // the only time the broadcast layer is allowed to send message A to the application layer
        // is when we know that we'll never get a message that comes before A in the total order of
        // this systems messages.

        // In this faulty example, we will consider that the case when:
        // 1. We have received NUM_NODE messages with sequence number i. i.e if we have 3 nodes,
        //    and we have received 1-1, 2-1, and 3-1, we know we'll never get another message with
        //    sequence number 1, therefore it is safe to deliver to the application.
        // 2. If we have no received a message from node X in 30 seconds, we assume it to be dead,
        //    and we decreease the number of messages needed for sequence number X by 1. This is
        //    where we'll be injecting the faults.
        todo!()
    }
}
