use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ChatMessage {
    Hello,
    IpList(Vec<Ipv4Addr>),
    Ping,
    Pong,
    CouldNotSerialize,
    UnexpectedMessage(Box<ChatMessage>),
    CouldNotDeserialize,
    NonBinaryMessageReceived,
}
