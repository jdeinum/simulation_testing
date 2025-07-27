use crate::simulation::Simulation;
use anyhow::{Context, Result};
use bytes::{BufMut, Bytes, BytesMut};
use futures::future::join_all;
use std::collections::HashSet;
use tokio::{io::AsyncWriteExt, task::JoinSet};

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
        // vec of futures
        // let mut f = JoinSet::new();

        // for each peer, we send the message
        for peer in &self.peers {
            self.s
                .send_message(peer, message.as_bytes().as_ref())
                .await
                .context("send message to peer");
        }

        // let res: Result<Vec<_>, anyhow::Error> = f.join_all().await.into_iter().collect();
        // let _ = res?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Bytes> {
        let b = self.s.receive_message().await.context("receive message")?;
        let msg: String = String::from_utf8(&b).context("parse utf8 string from bytes")?;
        self.messages.push(msg);
    }
}
