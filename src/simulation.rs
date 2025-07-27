use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::{Bytes, BytesMut, Buf};
use std::collections::HashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait Simulation {
    async fn send_message(&mut self, address: &str, message: &[u8]) -> Result<()>;
    async fn receive_message(&mut self) -> Result<Option<Bytes>>;
}

pub struct RealWorldFunctionality {
    pub peers: Arc<Mutex<HashMap<String, TcpStream>>>,
    pub message_receiver: mpsc::Receiver<Bytes>,
    _message_sender: mpsc::Sender<Bytes>,
}

impl RealWorldFunctionality {
    pub async fn build(endpoint: &str, peer_endpoints: HashMap<String, String>) -> Result<Self> {
        // Create message channel
        let (tx, rx) = mpsc::channel(100);
        
        // Bind to our own endpoint
        let listener = TcpListener::bind(endpoint)
            .await
            .context("bind listener")?;
        
        println!("Node listening on: {}", endpoint);
        
        let peers = Arc::new(Mutex::new(HashMap::new()));
        let peers_clone = peers.clone();
        let tx_clone = tx.clone();
        
        // Spawn listener task
        tokio::spawn(async move {
            loop {
                if let Ok((mut stream, addr)) = listener.accept().await {
                    println!("Accepted connection from: {}", addr);
                    let tx = tx_clone.clone();
                    
                    // Handle incoming messages from this connection
                    tokio::spawn(async move {
                        let mut buffer = BytesMut::with_capacity(4096);
                        let mut read_buf = vec![0; 1024];
                        
                        loop {
                            match stream.read(&mut read_buf).await {
                                Ok(0) => {
                                    println!("Connection closed from: {}", addr);
                                    break;
                                }
                                Ok(n) => {
                                    buffer.extend_from_slice(&read_buf[..n]);
                                    
                                    // Process messages separated by newlines
                                    while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                                        let message = buffer.split_to(pos);
                                        buffer.advance(1); // Skip the newline
                                        
                                        if !message.is_empty() {
                                            println!("Received message: {}", String::from_utf8_lossy(&message));
                                            let _ = tx.send(message.freeze()).await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Read error from {}: {}", addr, e);
                                    break;
                                }
                            }
                        }
                    });
                }
            }
        });
        
        // Try to connect to peers
        let peers_for_connection = peers.clone();
        tokio::spawn(async move {
            // Wait a bit for other nodes to start
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            
            for (id, endpoint) in peer_endpoints {
                match TcpStream::connect(&endpoint).await {
                    Ok(stream) => {
                        println!("Connected to peer {}: {}", id, endpoint);
                        peers_for_connection.lock().await.insert(id, stream);
                    }
                    Err(e) => {
                        eprintln!("Failed to connect to {}: {}", endpoint, e);
                    }
                }
            }
        });
        
        Ok(Self {
            peers: peers_clone,
            message_receiver: rx,
            _message_sender: tx,
        })
    }
}

#[async_trait]
impl Simulation for RealWorldFunctionality {
    async fn send_message(&mut self, address: &str, message: &[u8]) -> Result<()> {
        let mut peers = self.peers.lock().await;
        if let Some(stream) = peers.get_mut(address) {
            println!("Sending to {}: {}", address, String::from_utf8_lossy(message));
            stream
                .write_all(message)
                .await
                .context("send message to peer")?;
            stream.write_all(b"\n").await?; // Add delimiter
            stream.flush().await?;
        } else {
            eprintln!("Peer {} not connected yet, skipping message", address);
        }
        Ok(())
    }

    async fn receive_message(&mut self) -> Result<Option<Bytes>> {
        match self.message_receiver.recv().await {
            Some(message) => Ok(Some(message)),
            None => Ok(None),
        }
    }
}
