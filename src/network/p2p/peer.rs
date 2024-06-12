use crate::network::p2p::message::Message;
use crate::crypto::identity::PublicKey;
use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::Boxed;
use libp2p::core::upgrade::{read_length_prefixed, write_length_prefixed};
use libp2p::gossipsub::Gossipsub;
use libp2p::{identity, PeerId};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::io;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct Peer {
    pub address: String,
    pub public_key: PublicKey,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Peer {
    pub async fn new(address: String, stream: TLSStream) -> Result<Self, PeerError> {
        let public_key = stream.peer_public_key()?;
        let (sender, receiver) = mpsc::channel(1024);

        Ok(Peer {
            address,
            public_key,
            sender,
            receiver,
        })
    }

    pub async fn send(&mut self, message: Message) -> Result<(), PeerError> {
        self.sender.send(message).await.map_err(|e| PeerError::SendError(e.to_string()))
    }

    pub async fn receive(&mut self) -> Result<Message, PeerError> {
        self.receiver.recv().await.ok_or(PeerError::ReceiveError)
    }

    pub async fn handle_connection(mut stream: TLSStream, mut peer: Peer) {
        debug!("Connected to peer: {}", peer.address);

        loop {
            match tokio::io::copy_bidirectional(&mut stream, &mut peer).await {
                Ok((_, _)) => break,
                Err(e) => {
                    error!("Error communicating with peer: {}", e);
                    break;
                }
            }
        }

        debug!("Disconnected from peer: {}", peer.address);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PeerError {
    SendError(String),
    ReceiveError,
    TLSError(String),
}

impl From<tls::TLSError> for PeerError {
    fn from(error: tls::TLSError) -> Self {
        PeerError::TLSError(error.to_string())
    }
}
