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
    pub fn start() -> Result<Self> {}
}
