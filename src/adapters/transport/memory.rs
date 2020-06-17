#![allow(clippy::module_name_repetitions)]
mod addr;
mod channel;
mod factory;
mod registry;
#[cfg(test)]
mod unit_tests;

use crate::{app::Msg, error::transport::Result, ports::Transport};
use std::{collections::HashMap, sync::mpsc::channel};
pub use {
    addr::MemoryTransportAddr, channel::MemoryChannel, factory::MemoryTransportFactory,
    registry::MemoryTransportRegistry,
};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MemoryTransport {
    chans: HashMap<<Self as Transport>::Addr, MemoryChannel>,
}

impl MemoryTransport {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            chans: HashMap::new(),
        }
    }

    fn add_chan(&mut self, addr: <Self as Transport>::Addr, chan: MemoryChannel) -> &mut Self {
        self.chans.insert(addr, chan);
        self
    }
}

impl Transport for MemoryTransport {
    type Channel = MemoryChannel;
    type Addr = &'static Self;
    type Msgs = ();

    fn addr(&self) -> Self::Addr {
        self
    }

    fn connect_to(&mut self, mut remote: Self::Addr) -> Result<&mut Self> {
        let (local_tx, remote_rx) = channel();
        let (remote_tx, local_rx) = channel();

        remote.add_chan(self.addr(), MemoryChannel::new(remote_tx, remote_rx));
        self.add_chan(remote.addr(), MemoryChannel::new(local_tx, local_rx));
        Ok(self)
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
