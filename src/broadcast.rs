use crate::simulation::Simulation;
use anyhow::Result;
use bytes::{BufMut, Bytes, BytesMut};
use futures::future::join_all;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::{io::AsyncWriteExt, select};

/// The broadcast layer has a few responsibilities:
/// 1. When the application wants to send a message, it needs to
pub struct BroadcastLayer<S: Simulation> {
    pub peers: Vec<String>,
    pub log: Arc<Mutex<HashMap<usize, Vec<String>>>>,
    pub current_sequence_num: usize,
    pub s: Arc<Mutex<S>>,
}

impl<S: Simulation> BroadcastLayer<S> {
    // broadcast a message to our peers
    // NOTE: We have no retry mechanism because we assume the message will get through. This is of
    // course a silly assumption. We are injecting faults by messing with the order of messages. If
    // the system stopped procuding events, we can be certain that all messages would arrive
    // (eventually consistent)
    pub async fn broadcast(&mut self, message: &[u8]) -> Result<()> {
        let futures = self
            .peers
            .iter()
            .map(|p| self.s.lock().unwrap().send_message(p, message))
            .collect::<Vec<_>>();

        let results = join_all(futures).await;
        let r: Result<(), _> = results.into_iter().collect();

        match r {
            Ok(_) => return Ok(()),
            Err(e) => return Err(anyhow::anyhow!(e)),
        }
    }

    // just listen for messages on all of the sockets
    // these have to be cancellation safe so we use a combination of peek and read to get the
    // message
    async fn add_messages_to_queue(&mut self) -> Result<()> {
        todo!()
    }

    // the only time the broadcast layer is allowed to send message A to the application layer
    // is when we know that we'll never get a message that comes before A in the total order of
    // this systems messages.

    // In this faulty example, we will consider that the case when:
    // 1. We have received NUM_NODE messages with sequence number i. i.e if we have 3 nodes,
    //    and we have received 1-1, 1-2, and 1-3, we know we'll never get another message with
    //    sequence number 1, therefore it is safe to deliver to the application.
    // 2. If we have no received a message from node X in 30 seconds, we assume it to be dead,
    //    and we decreease the number of messages needed for sequence number X by 1. This is
    //    where we'll be injecting the faults.
    pub async fn receive(&mut self) -> Result<Bytes> {
        todo!()
        // if we have received NUM_NODES messages, we can deliver these messages to the
        // application

        // if 30 seconds have passed since we have heard from a node, we consider it dead
        // and the number of nodes we need to receive messages from decreases by 1
    }
}
