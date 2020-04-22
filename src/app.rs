mod chat_message;
mod chat_window;
#[cfg(test)]
mod unit_tests;

use std::net::{Ipv4Addr, SocketAddr};
use std::thread::{Builder, JoinHandle};

use crate::Error;
use crate::Result;
use chat_message::ChatMessage;
pub use chat_window::ChatWindow;
use get_if_addrs::{get_if_addrs, IfAddr};
use ws::listen;

pub struct App {
    pub local_socket: SocketAddr,
    pub listener_thread: JoinHandle<Result<()>>,
}

impl App {
    pub fn start(port: u16) -> Result<Self> {
        // Build socket
        let local_socket = SocketAddr::new(local_addr()?.into(), port);

        // Start listener
        let thread_builder = Builder::new();
        let listener_thread = thread_builder.spawn(move || {
            listen(local_socket, |sender| {
                // The handler needs to take ownership of sender, so we use move
                move |raw_msg| {
                    // Handle messages received on this connection
                    println!("Server got message '{:?}'. ", raw_msg);

                    let reply = match raw_msg {
                        ws::Message::Binary(message) => match bincode::deserialize(&message) {
                            Ok(ChatMessage::Ping) => ChatMessage::Pong,
                            Ok(ChatMessage::Hello) => ChatMessage::IpList(vec![]),
                            Ok(bad_message) => {
                                ChatMessage::UnexpectedMessage(Box::new(bad_message))
                            }
                            _ => ChatMessage::CouldNotDeserialize,
                        },
                        _ => ChatMessage::NonBinaryMessageReceived,
                    };
                    // TODO  - remove unwrap and share default response from struct
                    #[allow(clippy::result_unwrap_used)]
                    let default_response =
                        bincode::serialize(&ChatMessage::CouldNotSerialize).unwrap();
                    let serialized_reply = bincode::serialize(&reply).unwrap_or(default_response);
                    println!("Server sent reply'{:?}'. ", serialized_reply);

                    // Use the out channel to send messages back
                    sender.send(serialized_reply)
                }
            })
            .map_err(Error::from)
        })?;

        Ok(Self {
            local_socket,
            listener_thread,
        })
    }
}

fn local_addr() -> Result<Ipv4Addr> {
    get_if_addrs()?.into_iter().nth(0).map_or_else(
        || Err(Error::NoIpAddrFound),
        |intrfc| match intrfc.addr {
            IfAddr::V4(addr) => Ok(addr.ip),
            IfAddr::V6(_) => Err(Error::IpTypeMismatch),
        },
    )
}
