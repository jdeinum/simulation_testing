use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use std::collections::HashMap;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[async_trait]
pub trait Simulation {
    async fn send_message(&mut self, address: &str, message: &[u8]) -> Result<()>;
    async fn receive_message(&mut self) -> Result<Option<Bytes>>;
}

pub struct RealWorldFunctionality {
    // our peers we are sending to and receiving from
    peers: HashMap<String, TcpStream>,
    message_buffer: BytesMut,
}

#[async_trait]
impl Simulation for RealWorldFunctionality {
    async fn send_message(&mut self, address: &str, message: &[u8]) -> Result<()> {
        self.peers
            .get_mut(address)
            .context("missing peer in map")?
            .write_all(message)
            .await
            .context("send message to peer")
    }

    // read exactly one message from a peer
    // currently this is not cancellation safe. need to figure out a way to do that.
    async fn receive_message(&mut self) -> Result<Option<Bytes>> {
        todo!();
        // let mut tasks = FuturesUnordered::new();
        //
        // for (addr, stream) in self.peers.iter_mut() {
        //     let mut buf = vec![0u8; MESSAGE_SIZE];
        //     let future = async move {
        //         stream
        //             .read_exact(&mut buf)
        //             .await
        //             .ok()
        //             .context("read bytes")?;
        //         Some((addr.clone(), Bytes::from(buf)))
        //     };
        //     tasks.push(future);
        // }
        //
        // while let Some(Some((_addr, msg))) = tasks.next().await {
        //     return Ok(Some(msg));
        // }
        //
        // return Ok(None);
    }
}
