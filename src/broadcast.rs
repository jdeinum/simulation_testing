use crate::simulation::Simulation;
use anyhow::Result;
use bytes::{BufMut, Bytes, BytesMut};
use futures::future::join_all;
use std::collections::HashMap;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub struct BroadcastLayer<S: Simulation> {
    pub peers: HashMap<String, TcpStream>,
    pub buf: BytesMut,
    pub s: S,
}

impl<S: Simulation> BroadcastLayer<S> {
    pub async fn broadcast(&mut self, message: &str) -> Result<()> {
        self.buf.clear();
        self.buf.put(message.as_bytes());

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
        todo!()
    }
}
