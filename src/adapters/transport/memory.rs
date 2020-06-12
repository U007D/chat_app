#![allow(clippy::module_name_repetitions)]
mod channel;
#[cfg(test)]
mod unit_tests;

use crate::{
    app::Msg,
    error::transport::{memory::Error, Result},
    ports::Transport,
};
pub use channel::MemoryChannel;

#[derive(Debug, Eq, PartialEq)]
pub struct MemoryTransport {}

impl MemoryTransport {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Transport for MemoryTransport {
    type Channel = MemoryChannel;
    type Id = Self;
    type Msgs = ();

    fn connect_to(&mut self, id: &Self::Id) -> Result<Self::Channel> {
        match self.id() == id.id() {
            true => Ok(MemoryChannel {}),
            false => Err(Error::RemoteTransportNotFound(dbg!(self.id()) as *const Self).into()),
        }
    }

    fn id(&self) -> &Self::Id {
        self
    }

    fn msgs(&mut self) -> Self::Msgs {
        unimplemented!()
    }

    fn send_msg(&self, _msg: Msg) -> Result<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
