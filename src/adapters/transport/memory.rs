#![allow(clippy::module_name_repetitions)]
mod channel;
mod id;
mod registry;
#[cfg(test)]
mod unit_tests;

use crate::{app::Msg, error::transport::Result, ports::Transport};
use std::{collections::HashMap, sync::mpsc::channel};
pub use {channel::MemoryChannel, id::MemoryTransportId, registry::MemoryTransportRegistry};

#[derive(Debug, PartialEq)]
pub struct MemoryTransport {
    chans: HashMap<usize, MemoryChannel>,
}

impl MemoryTransport {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            chans: HashMap::new(),
        }
    }

    fn add_chan(&mut self, id: <Self as Transport>::Id, chan: MemoryChannel) -> &mut Self {
        self.chans.insert(id as usize, chan);
        self
    }
}

impl Transport for MemoryTransport {
    type Channel = MemoryChannel;
    type Id = Self;
    type Msgs = ();

    fn connect_to(&mut self, mut remote: Self::Id) -> Result<&mut Self> {
        let (local_tx, remote_rx) = channel();
        let (remote_tx, local_rx) = channel();

        remote.add_chan(self.id(), MemoryChannel::new(remote_tx, remote_rx));
        self.add_chan(remote.id(), MemoryChannel::new(local_tx, local_rx));
        Ok(self)
    }

    fn id(&self) -> Self::Id {
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
