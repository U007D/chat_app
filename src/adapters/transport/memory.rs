#![allow(clippy::module_name_repetitions)]

use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
};

use bool_ext::BoolExt;
use dashmap::DashMap;

pub use addr::MemoryTransportAddr;
use channel::MemoryChannel;
use lazy_static::lazy_static;
pub use sender_dispenser::SenderDispenser;

use crate::ports::transport::Channel;
use crate::{
    app::Msg,
    error::transport::memory::{Error, Result},
    ports::Transport,
};

mod addr;
mod channel;
mod sender_dispenser;
#[cfg(test)]
mod unit_tests;

lazy_static! {
    // `Mutex<usize>` is used for `MEMORY_TRANSPORT_NEXT_ADDR` instead of `AtomicUsize` because
    // saturating arithmetic is being performed on the `usize`.  It is not possible to do this
    // atomically with `AtomicUsize` without `fetch_update`, which is not stable at the time of this
    // writing (1.44.1).
#[allow(clippy::mutex_atomic)]

    static ref MEMORY_TRANSPORT_NEXT_ADDR: Mutex<usize> =
    Mutex::new(0_usize);
    static ref SENDER_STORE:
        DashMap<MemoryTransportAddr,
        SenderDispenser<<<MemoryTransport as Transport>::Channel as Channel>::Msg>>
        = DashMap::new();
}

#[derive(Debug)]
pub struct MemoryTransport {
    addr: MemoryTransportAddr,
    rx: Receiver<<<Self as Transport>::Channel as Channel>::Msg>,
    senders: HashMap<MemoryTransportAddr, Sender<<<Self as Transport>::Channel as Channel>::Msg>>,
}

impl MemoryTransport {
    pub fn new() -> Self {
        let addr = Self::addr();
        let (tx, rx) = Self::make_channel(addr);

        Self {
            addr,
            rx,
            senders: HashMap::new(),
        }
    }

    // Atomically increment `MEMORY_TRANSPORT_NEXT_ADDR`, but panic if it would overflow
    // (panicking_add).
    fn addr() -> MemoryTransportAddr {
        let mut guard = MEMORY_TRANSPORT_NEXT_ADDR
            .lock()
            .unwrap_or_else(|_| unreachable!(Error::NextAddrLockFailed));
        let addr = *guard;
        let next_addr = addr.saturating_add(1);
        (addr != next_addr)
            .some_with(|| *guard = next_addr)
            .unwrap_or_else(|| panic!(Error::TooManyInstances));
        MemoryTransportAddr::from(addr)
    }

    fn make_channel(
        addr: MemoryTransportAddr,
    ) -> (
        Sender<<<Self as Transport>::Channel as Channel>::Msg>,
        Receiver<<<Self as Transport>::Channel as Channel>::Msg>,
    ) {
        let (tx, rx) = channel();
        Self::register_sender(addr, tx.clone());
        (tx, rx)
    }

    fn register_sender(
        addr: MemoryTransportAddr,
        tx: Sender<<<Self as Transport>::Channel as Channel>::Msg>,
    ) {
        SENDER_STORE
            .contains_key(&addr)
            .do_false(|| {
                SENDER_STORE
                    .insert(addr, SenderDispenser::new(tx))
                    .map_or_else(|| {}, |_| unreachable!(Error::AddrFalseNegative(addr)))
            })
            .do_true(|| unreachable!(Error::AddrAlreadyAdded(addr)));
    }

    pub fn with_connection(addr: MemoryTransportAddr) -> Result<Self, <Self as Transport>::Error> {
        let mut res = Self::new();
        res.connect_to(addr)?;
        Ok(res)
    }
}

impl Transport for MemoryTransport {
    type Channel = MemoryChannel;
    type Addr = MemoryTransportAddr;
    type Error = Error;

    fn addr(&self) -> Self::Addr {
        self.addr
    }

    #[allow(unused_variables)]
    fn connect_to(&mut self, addr: Self::Addr) -> Result<&mut Self, Self::Error> {
        self.senders.contains_key(&addr).map(
            || Err(Error::AddrAlreadyConnected(addr)),
            || {
                self.senders
                    .insert(
                        addr,
                        SENDER_STORE
                            .get(&addr)
                            .ok_or_else(|| Error::RemoteAddrNotFound(addr))?
                            .get(),
                    )
                    .map_or_else(
                        || Ok(()),
                        |_| {
                            unreachable!(Error::AddrFalseNegative(addr));
                        },
                    )
            },
        )?;
        Ok(self)
    }

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
