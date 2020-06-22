mod addr;
mod static_store;
mod tx_dispenser;
#[cfg(test)]
mod unit_tests;

use crate::{
    app::Msg,
    error::transport::memory::{Error, Result},
    ports::Transport,
};
pub use addr::MemoryTransportAddr;
use bool_ext::BoolExt;
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
};
pub use tx_dispenser::TxDispenser;

#[derive(Debug)]
pub struct MemoryTransport {
    addr: MemoryTransportAddr,
    rx: Receiver<<Self as Transport>::Msg>,
    senders: HashMap<MemoryTransportAddr, Sender<<Self as Transport>::Msg>>,
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

    pub fn with_connection(addr: MemoryTransportAddr) -> Result<Self, <Self as Transport>::Error> {
        let mut res = Self::new();
        res.connect_to(addr)?;
        Ok(res)
    }

    // Atomically increment `NEXT_ADDR_STORE`, but panic if it would overflow
    // (panicking_add).
    fn addr() -> MemoryTransportAddr {
        let mut guard = static_store::NEXT_ADDR_STORE
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
    ) -> (Sender<<Self as Transport>::Msg>, Receiver<<Self as Transport>::Msg>) {
        let (tx, rx) = channel();
        Self::register_sender(addr, tx.clone());
        (tx, rx)
    }

    fn register_sender(addr: MemoryTransportAddr, tx: Sender<<Self as Transport>::Msg>) {
        static_store::SENDER_STORE
            .contains_key(&addr)
            .do_false(|| {
                static_store::SENDER_STORE
                    .insert(addr, TxDispenser::new(tx))
                    .map_or_else(|| {}, |_| unreachable!(Error::AddrFalseNegative(addr)))
            })
            .do_true(|| unreachable!(Error::AddrAlreadyAdded(addr)));
    }
}

impl Transport for MemoryTransport {
    type Addr = MemoryTransportAddr;
    type Error = Error;
    type Msg = Msg;

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
                        static_store::SENDER_STORE
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
    fn rx_msg(&mut self) -> Result<<Self as Transport>::Msg> {
        Ok(self.rx.recv()?)
    }

    #[allow(unused_variables)]
    fn tx_msg(&self, msg: <Self as Transport>::Msg, addr: Self::Addr) -> Result<&Self, Self::Error>
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
