#![allow(clippy::module_name_repetitions)]
mod addr;
mod channel;
mod sender_dispenser;
#[cfg(test)]
mod unit_tests;

use crate::{
    app::Msg,
    error::transport::memory::{Error, Result},
    ports::Transport,
};
pub use addr::MemoryTransportAddr;
use bool_ext::BoolExt;
use channel::MemoryChannel;
use lazy_static::lazy_static;
pub use sender_dispenser::SenderDispenser;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref MEMORY_TRANSPORT_NEXT_ADDR: Mutex<usize> = Mutex::new(0_usize);
    static ref SENDER_STORE: HashMap<MemoryTransportAddr, SenderDispenser<Msg>> = HashMap::new();
}

#[derive(Debug)]
pub struct MemoryTransport {
    addr: MemoryTransportAddr,
    channels: Vec<MemoryChannel>,
}

impl MemoryTransport {
    fn new() -> Self {
        Self {
            // Atomically increment `MEMORY_TRANSPORT_NEXT_ADDR`, but panic if it would overflow.
            addr: {
                let mut guard = MEMORY_TRANSPORT_NEXT_ADDR
                    .lock()
                    .expect("Internal Error: `next_addr` instance unexpectedly not available.");
                let addr = *guard;
                let next_addr = addr.saturating_add(1);
                (addr != next_addr)
                    .some_with(|| *guard = next_addr)
                    .unwrap_or_else(|| {
                        panic!(format!(
                            "Too many (> {}) `MemoryTransport` instances created.",
                            usize::max_value()
                        ))
                    });
                MemoryTransportAddr::from(addr)
            },
            channels: Vec::new(),
        }
    }

    fn with_connection(addr: MemoryTransportAddr) -> Result<Self, <Self as Transport>::Error> {
        let mut res = Self::new();
        res.connect_to(addr)?;
        Ok(res)
    }
}

impl Transport for MemoryTransport {
    type Channel = MemoryChannel;
    type Addr = MemoryTransportAddr;
    type Error = Error;
    type Msg = Msg;

    fn addr(&self) -> Self::Addr {
        self.addr
    }

    fn connect_to(&mut self, id: Self::Addr) -> Result<&mut Self, Self::Error> {
        unimplemented!()
    }

    fn msg(&mut self) -> Self::Msg {
        unimplemented!()
    }

    fn send_msg(&self, msg: Self::Msg) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
