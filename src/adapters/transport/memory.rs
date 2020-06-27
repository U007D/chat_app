mod addr;
mod envelope;
mod static_store;
mod tx_dispenser;
#[cfg(test)]
mod unit_tests;

use crate::{
    error::transport::memory::{Error, Result},
    ports::transport::Transport,
};
pub use {addr::MemoryTransportAddr, envelope::MemoryTransportEnvelope, tx_dispenser::TxDispenser};

use crate::ports::transport::Envelope;
use bool_ext::BoolExt;
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(Debug)]
pub struct MemoryTransport {
    addr: MemoryTransportAddr,
    rx: Receiver<<Self as Transport>::Envelope>,
    senders: HashMap<MemoryTransportAddr, Sender<<Self as Transport>::Envelope>>,
}

impl MemoryTransport {
    pub fn new() -> Self {
        let addr = Self::addr();
        let rx = Self::make_channel(addr);

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

    fn make_channel(addr: MemoryTransportAddr) -> Receiver<<Self as Transport>::Envelope> {
        let (tx, rx) = channel();
        Self::register_sender(addr, tx);
        rx
    }

    fn register_sender(addr: MemoryTransportAddr, tx: Sender<<Self as Transport>::Envelope>) {
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
    type Envelope = MemoryTransportEnvelope;
    type Error = Error;

    fn addr(&self) -> Self::Addr {
        self.addr
    }

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

    fn rx_msg(
        &mut self,
    ) -> Result<(&<<Self as Transport>::Envelope as Envelope>::Msg, Self::Addr)> {
        let res = self.rx.recv()?;
        Ok((res.msg(), res.addr()))
    }

    fn tx_msg(
        &self,
        msg: <<Self as Transport>::Envelope as Envelope>::Msg,
        dst: <Self as Transport>::Addr,
    ) -> Result<&Self, Self::Error>
    where
        Self: Sized,
    {
        self.senders
            .get(&dst)
            .ok_or_else(|| Error::RemoteAddrNotFound(dst))?
            .send((msg, self.addr()).into());
        Ok(self)
    }
}

impl Eq for MemoryTransport {}

impl PartialEq for MemoryTransport {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const Self == rhs as *const Self
    }
}
