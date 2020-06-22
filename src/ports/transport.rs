#![allow(clippy::module_name_repetitions)]
mod channel;
use crate::{error::transport::Error as TransportError, error::Result};
pub use channel::Channel;

pub trait Transport {
    type Channel: Channel;
    type Addr;
    type Error: Into<TransportError>;

    fn addr(&self) -> Self::Addr;
    fn connect_to(&mut self, addr: Self::Addr) -> Result<&mut Self, Self::Error>;
    fn msg(&mut self) -> Result<<<Self as Transport>::Channel as Channel>::Msg, Self::Error>;
    fn send_msg(
        &self,
        msg: <<Self as Transport>::Channel as Channel>::Msg,
        addr: Self::Addr,
    ) -> Result<&Self, Self::Error>
    where
        Self: Sized;
}
