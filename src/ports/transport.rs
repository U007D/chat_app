#![allow(clippy::module_name_repetitions)]
mod channel;
use crate::{app::Msg, error::transport::Result};
pub use channel::Channel;

pub trait Transport {
    type Channel: Channel;
    type Id;
    type Msgs;

    fn connect_to(&mut self, id: &Self::Id) -> Result<Self::Channel>;
    fn id(&self) -> &Self::Id;
    fn msgs(&mut self) -> Self::Msgs;
    fn send_msg(&self, msg: Msg) -> Result<Self>
    where
        Self: Sized;
}
