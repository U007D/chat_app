#![allow(clippy::module_name_repetitions)]
mod channel;
use crate::{app::Msg, error::transport::Result};
pub use channel::Channel;

pub trait Transport {
    type Channel: Channel;
    type Addr;
    type Msgs;

    fn addr(&self) -> Self::Addr;
    fn connect_to(&mut self, id: Self::Addr) -> Result<&mut Self>;
    fn msgs(&mut self) -> Self::Msgs;
    fn send_msg(&self, msg: Msg) -> Result<Self>
    where
        Self: Sized;
}
