use std::sync::mpsc::{channel, Receiver, Sender};

use crate::adapters::transport::memory::TxDispenser;
use crate::{
    adapters::transport::memory::{singleton, MemoryTransportAddr},
    error::transport::memory::channel::{Error, Result},
    ports::transport::Channel,
};
use bool_ext::BoolExt;
use std::collections::HashMap;

type Addr = <MemoryChannel as Channel>::Addr;
type Msg = <MemoryChannel as Channel>::Msg;

#[derive(Debug)]
pub struct MemoryChannel {
    rx: Receiver<Msg>,
    txs: HashMap<Addr, Sender<Msg>>,
}

impl MemoryChannel {
    pub fn new(addr: Addr) -> Self {
        Self {
            rx: Self::make_channel(addr),
            txs: HashMap::new(),
        }
    }

    fn make_channel(addr: Addr) -> Receiver<Msg> {
        let (tx, rx) = channel();
        Self::register_channel(addr, tx);
        rx
    }

    fn register_channel(addr: Addr, tx: Sender<Msg>) {
        singleton::TX_STORE
            .contains_key(&addr)
            .do_false(|| {
                singleton::TX_STORE
                    .insert(addr, TxDispenser::new(tx))
                    .map_or_else(|| {}, |_| unreachable!(Error::AddrFalseNegative(addr)))
            })
            .do_true(|| unreachable!(Error::AddrAlreadyAdded(addr)));
    }

    #[allow(unused_variables)]
    pub(super) fn connect_to(&mut self, addr: Addr) -> Result<&mut Self> {
        self.txs.contains_key(&addr).map(
            || Err(Error::AddrAlreadyConnected(addr)),
            || {
                self.txs
                    .insert(
                        addr,
                        singleton::TX_STORE
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
}

impl Channel for MemoryChannel {
    type Addr = MemoryTransportAddr;
    type Error = Error;
    type Msg = crate::app::Msg;

    fn rx(&self) -> Result<Self::Msg> {
        Ok(self.rx.recv()?)
    }

    fn tx(&self, msg: Self::Msg, addr: Addr) -> Result<&Self> {
        Ok(self
            .txs
            .get(&addr)
            .ok_or_else(|| Error::RemoteAddrNotFound(addr))?
            .send(msg)
            .map(|_| self)?)
    }
}

impl Eq for MemoryChannel {}

impl PartialEq for MemoryChannel {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const Self == rhs as *const Self
    }
}
