use crate::network::message::Message;
use crate::utils::error::Result;

pub trait NetworkInterface {
    fn broadcast_message(&self, message: Message) -> Result<()>;
    fn send_message(&self, message: Message, peer_id: &str) -> Result<()>;
    fn receive_message(&self) -> Result<Message>;
}
