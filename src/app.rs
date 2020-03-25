#[cfg(test)]
mod unit_tests;

use std::default::Default;
use std::net::SocketAddr;
use std::thread::{Builder, JoinHandle};

use get_if_addrs::{get_if_addrs, IfAddr};
use iced::{Application, Column, Command, Element, Text};
use ws;

use crate::Error;
use crate::Result;
use ws::listen;
use serde::{Serialize, Deserialize};
use bincode;

pub struct App {
    pub local_socket: SocketAddr,
    pub listener_thread: JoinHandle<Result<()>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
enum ChatMessage {
    Ping,
    Pong,
    CouldNotSerialize,
    UnexpectedMessage(Box<ChatMessage>),
    CouldNotDeserialize,
    NonBinaryMessageReceived
}

impl App {
    pub fn start() -> Result<Self> {
        // Build socket
        const PORT: u16 = 4444;
        let addrs = get_if_addrs()?;
        let local_addr = addrs.into_iter().nth(0).map_or_else(
            || Err(Error::NoIpAddrFound),
            |intrfc| match intrfc.addr {
                IfAddr::V4(addr) => Ok(addr.ip),
                IfAddr::V6(_) => Err(Error::IpTypeMismatch),
            },
        )?;
        let local_socket = SocketAddr::new(local_addr.into(), PORT);

        // Start listener
        let thread_builder = Builder::new();
        let listener_thread = thread_builder.spawn(move || {
            let result = listen(local_socket, |sender| {
                // The handler needs to take ownership of sender, so we use move
                move |raw_msg| {
                    // Handle messages received on this connection
                    println!("Server got message '{}'. ", raw_msg);

                    let reply = match raw_msg {
                        ws::Message::Binary(message) => {
                            match bincode::deserialize(&message) {
                                Ok(ChatMessage::Ping) => ChatMessage::Pong,
                                Ok(bad_message) => ChatMessage::UnexpectedMessage(Box::new(bad_message)),
                                _ => ChatMessage::CouldNotDeserialize,
                            }
                        },
                        _ => ChatMessage::NonBinaryMessageReceived
                    };
                    // TODO  - remove unwrap and share default response from struct
                    let default_response =
                        bincode::serialize(&ChatMessage::CouldNotSerialize).unwrap();
                    let serialized_reply= bincode::serialize
                        (&reply).unwrap_or(default_response);

                    // Use the out channel to send messages back
                    sender.send(serialized_reply)
                }
            })
            .map_err(Error::from);
            result
        })?;


        Ok(Self {
            local_socket,
            listener_thread,
        })
    }
}

#[derive(Default)]
pub(crate) struct ChatWindow {}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Application for ChatWindow {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Chat App")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new("Welcome to the Chat App").size(50))
            .into()
    }
}
