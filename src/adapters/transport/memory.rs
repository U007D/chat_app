mod addr;
pub(crate) mod channel;
mod singleton;
mod tx_dispenser;
#[cfg(test)]
mod unit_tests;

use crate::{
    error::transport::memory::{Error, Result},
    ports::{transport::Channel, Transport},
};
pub use addr::MemoryTransportAddr;
use bool_ext::BoolExt;
pub use tx_dispenser::TxDispenser;

type MemoryChannel = <MemoryTransport as Transport>::Channel;
type Addr = <MemoryChannel as Channel>::Addr;
type Msg = <MemoryChannel as Channel>::Msg;

#[derive(Debug)]
pub struct MemoryTransport {
    addr: Addr,
    channel: MemoryChannel,
}

impl MemoryTransport {
    pub fn new() -> Self {
        let addr = Self::addr();

        Self {
            addr,
            channel: MemoryChannel::new(addr),
        }
    }

    // Atomically increment `MEMORY_TRANSPORT_NEXT_ADDR`, but panic if it would overflow
    // (equivalent to atomic panicking_add).
    fn addr() -> Addr {
        let mut guard = singleton::MEMORY_TRANSPORT_NEXT_ADDR
            .lock()
            .unwrap_or_else(|_| unreachable!(Error::NextAddrLockFailed));
        let addr = *guard;
        let next_addr = addr.saturating_add(1);
        (addr != next_addr)
            .some_with(|| *guard = next_addr)
            .unwrap_or_else(|| panic!(Error::TooManyInstances));
        MemoryTransportAddr::from(addr)
    }

    pub fn with_connection(addr: Addr) -> Result<Self, <Self as Transport>::Error> {
        let mut res = Self::new();
        res.connect_to(addr)?;
        Ok(res)
    }
}

impl Transport for MemoryTransport {
    type Channel = crate::adapters::transport::memory::channel::MemoryChannel;
    type Error = Error;

    fn addr(&self) -> <Self::Channel as Channel>::Addr {
        self.addr
    }

    #[allow(unused_variables)]
    fn connect_to(
        &mut self,
        addr: <Self::Channel as Channel>::Addr,
    ) -> Result<&mut Self, Self::Error> {
        self.channel.connect_to(addr)?;
        Ok(self)
    }

    #[allow(unused_variables)]
    fn msg(&mut self) -> Result<<Self::Channel as Channel>::Msg> {
        Ok(self.channel.rx()?)
    }

    #[allow(unused_variables)]
    fn send_msg(
        &self,
        msg: Msg,
        addr: <Self::Channel as Channel>::Addr,
    ) -> Result<&Self, Self::Error>
    where
        Self: Sized,
    {
        self.channel.tx(msg, addr)?;
        Ok(self)
    }
}

impl Eq for MemoryTransport {}

impl PartialEq for MemoryTransport {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const Self == rhs as *const Self
    }
}
