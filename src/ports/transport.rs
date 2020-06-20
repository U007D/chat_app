#![allow(clippy::module_name_repetitions)]
mod channel;
use crate::{error::transport::Error as TransportError, error::Result};
pub use channel::Channel;

pub trait Transport {
    type Channel: Channel;
    type Addr;
    type Error: Into<TransportError>;
    type Msg;

    fn addr(&self) -> Self::Addr;
    fn connect_to(&mut self, id: Self::Addr) -> Result<&mut Self, Self::Error>;
    fn msg(&mut self) -> Self::Msg;
    fn send_msg(&self, msg: Self::Msg) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
