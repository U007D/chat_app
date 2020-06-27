mod addr;
mod envelope;
mod static_store;
mod tx_sharer;
#[cfg(test)]
mod unit_tests;

use crate::{
    error::transport::{memory::Error, Result},
    ports::transport::Transport,
};
pub use {addr::MemoryTransportAddr, envelope::MemoryTransportEnvelope, tx_sharer::TxSharer};

use crate::error::transport::memory;
use crate::ports::transport::Envelope;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub struct MemoryTransport {
    addr: MemoryTransportAddr,
    rx: Receiver<<Self as Transport>::Envelope>,
}

impl MemoryTransport {
    pub fn new() -> Self {
        let addr = static_store::UNIQUE_ADDR.addr();

        Self {
            addr,
            rx: Self::init_channel(addr),
        }
    }

    fn init_channel(addr: MemoryTransportAddr) -> Receiver<<Self as Transport>::Envelope> {
        let (tx, rx) = channel();
        Self::register_tx(addr, tx);
        rx
    }

    fn register_tx(addr: MemoryTransportAddr, tx: Sender<<Self as Transport>::Envelope>) {
        static_store::SENDER_STORE
            .insert(addr, TxSharer::new(tx))
            .and_then::<(), _>(|_| unreachable!(Error::AddrAlreadyAdded(addr)));
    }
}

impl Transport for MemoryTransport {
    type Addr = MemoryTransportAddr;
    type Envelope = MemoryTransportEnvelope;

    fn addr(&self) -> Self::Addr {
        self.addr
    }

    fn rx_msg(&self) -> Result<(<<Self as Transport>::Envelope as Envelope>::Msg, Self::Addr)> {
        let res = self.rx.recv().map_err(memory::Error::from)?;
        Ok(res.into())
    }

    // Look for `dst` in the local `txs` list.  If not found, cache a copy of `dst`'s tx from the
    // global registry, then send.
    fn tx_msg(
        &self,
        msg: <<Self as Transport>::Envelope as Envelope>::Msg,
        dst: <Self as Transport>::Addr,
    ) -> Result<&Self>
    where
        Self: Sized,
    {
        static_store::SENDER_STORE
            .get(&dst)
            .ok_or_else(|| Error::RemoteAddrNotFound(dst))?
            .lock()
            .send((msg, self.addr()).into())
            .map_err(memory::Error::from)?;
        Ok(self)
    }
}

impl Eq for MemoryTransport {}

impl PartialEq for MemoryTransport {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const Self == rhs as *const Self
    }
}
