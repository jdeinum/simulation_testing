use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use std::collections::HashMap;
use tokio::{io::AsyncWriteExt, net::TcpStream};

const MESSAGE_SIZE: usize = 128;

#[async_trait]
pub trait Simulation {
    async fn send_message(&mut self, address: &str, message: &[u8]) -> Result<()>;
    async fn receive_message(&mut self) -> Result<Option<Bytes>>;
}

pub struct RealWorldFunctionality {
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
    }
}
