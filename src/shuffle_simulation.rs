use async_trait::async_trait;
use bytes::Bytes;
use std::collections::VecDeque;
use anyhow::Result;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use crate::simulation::Simulation;

pub struct ShuffleSimulation {
    pub seed: u64,
    pub message_queue: VecDeque<(String, Bytes)>,
    pub sent_messages: Vec<(String, Vec<u8>)>,
    rng: SmallRng,
}

impl ShuffleSimulation {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            message_queue: VecDeque::new(),
            sent_messages: Vec::new(),
            rng: SmallRng::seed_from_u64(seed),
        }
    }
    
    pub fn inject_message(&mut self, from: String, message: &[u8]) {
        self.message_queue.push_back((from, Bytes::copy_from_slice(message)));
        
        // If seed is divisible by 10, shuffle the message queue
        if self.seed % 10 == 0 {
            let mut messages: Vec<_> = self.message_queue.drain(..).collect();
            messages.shuffle(&mut self.rng);
            self.message_queue.extend(messages);
            println!("Shuffled message queue (seed {} is divisible by 10)", self.seed);
        }
    }
}

#[async_trait]
impl Simulation for ShuffleSimulation {
    async fn send_message(&mut self, address: &str, message: &[u8]) -> Result<()> {
        self.sent_messages.push((address.to_string(), message.to_vec()));
        
        // Simulate the message being received (for testing)
        // In a real scenario, this would be handled by the receiving node
        self.inject_message(address.to_string(), message);
        
        Ok(())
    }

    async fn receive_message(&mut self) -> Result<Option<Bytes>> {
        if let Some((_from, message)) = self.message_queue.pop_front() {
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }
}