#![allow(clippy::module_name_repetitions)]
mod channel;
mod id;
use crate::{app::Msg, Result};
pub use {channel::IChannel, id::IId};

pub trait ITransport {
    type Channel: IChannel;
    type Id: IId;
    type Msgs: Iterator<Item = Msg>;

    fn connect_to(&mut self, id: Self::Id) -> Result<Self::Channel>;
    fn msgs(&mut self) -> Self::Msgs;
    fn send_msg(&self, msg: Msg) -> Result<Self>
    where
        Self: Sized;
}
