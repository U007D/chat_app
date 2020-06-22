#![allow(clippy::module_name_repetitions)]
mod singleton;

mod addr;
mod channel;
mod tx_dispenser;
#[cfg(test)]
mod unit_tests;

use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
};

use bool_ext::BoolExt;
use dashmap::DashMap;

pub use addr::Addr;
use channel::MemoryChannel;
use lazy_static::lazy_static;
pub use tx_dispenser::TxDispenser;

use crate::ports::transport::Channel;
use crate::{
    app::Msg,
    error::transport::memory::{Error, Result},
    ports::Transport,
};

#[derive(Debug)]
pub struct MemoryTransport {
    addr: Addr,
    channel: <Self as Transport>::Channel,
}

impl MemoryTransport {
    pub fn new() -> Self {
        let addr = Self::addr();

        Self {
            addr,
            channel: Channel::new(),
        }
    }

    // Atomically increment `MEMORY_TRANSPORT_NEXT_ADDR`, but panic if it would overflow
    // (panicking_add).
    fn addr() -> Addr {
        let mut guard = singleton::MEMORY_TRANSPORT_NEXT_ADDR
            .lock()
            .unwrap_or_else(|_| unreachable!(Error::NextAddrLockFailed));
        let addr = *guard;
        let next_addr = addr.saturating_add(1);
        (addr != next_addr)
            .some_with(|| *guard = next_addr)
            .unwrap_or_else(|| panic!(Error::TooManyInstances));
        Addr::from(addr)
    }

    pub fn with_connection(addr: Addr) -> Result<Self, <Self as Transport>::Error> {
        let mut res = Self::new();
        res.connect_to(addr)?;
        Ok(res)
    }
}

impl Transport for MemoryTransport {
    type Channel = MemoryChannel;
    type Addr = Addr;
    type Error = Error;

    fn addr(&self) -> Self::Addr {
        self.addr
    }

    #[allow(unused_variables)]
    fn connect_to(&mut self, addr: Self::Addr) -> Result<&mut Self, Self::Error> {}

    #[allow(unused_variables)]
    fn msg(&mut self) -> Result<<<Self as Transport>::Channel as Channel>::Msg> {
        Ok(self.rx.recv()?)
    }

    #[allow(unused_variables)]
    fn send_msg(
        &self,
        msg: <<Self as Transport>::Channel as Channel>::Msg,
        addr: Self::Addr,
    ) -> Result<&Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(self
            .senders
            .get(&addr)
            .ok_or_else(|| Error::RemoteAddrNotFound(addr))?
            .send(msg)
            .map(|_| self)?)
    }
}

impl Eq for MemoryTransport {}

impl PartialEq for MemoryTransport {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const Self == rhs as *const Self
    }
}
