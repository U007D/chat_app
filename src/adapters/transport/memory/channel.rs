use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    adapters::transport::memory::{singleton, MemoryTransport},
    error::transport::channel::{Error, Result},
    ports::{transport::Channel, Transport},
};
use bool_ext::BoolExt;
use std::collections::HashMap;

type MemoryTransportAddr = <MemoryTransport as Transport>::Addr;
type Msg = <MemoryChannel as Channel>::Msg;

#[derive(Debug)]
pub struct MemoryChannel {
    rx: Receiver<Msg>,
    txs: HashMap<MemoryTransportAddr, Sender<Msg>>,
}

impl MemoryChannel {
    pub fn new(addr: MemoryTransportAddr) -> Self {
        Self {
            rx: Self::make_channel(addr),
            txs: HashMap::new(),
        }
    }

    fn make_channel(addr: MemoryTransportAddr) -> Receiver<Msg> {
        let (tx, rx) = channel();
        Self::register_channel(addr, tx);
        rx
    }

    fn register_channel(addr: MemoryTransportAddr, tx: Sender<Msg>) {
        singleton::TX_STORE
            .contains_key(&addr)
            .do_false(|| {
                TX_STORE
                    .insert(addr, SenderDispenser::new(tx))
                    .map_or_else(|| {}, |_| unreachable!(Error::AddrFalseNegative(addr)))
            })
            .do_true(|| unreachable!(Error::AddrAlreadyAdded(addr)));
    }

    #[allow(unused_variables)]
    fn connect_to(&mut self, addr: Self::Addr) -> Result<&mut Self, Self::Error> {
        self.txs.contains_key(&addr).map(
            || Err(Error::AddrAlreadyConnected(addr)),
            || {
                self.txs
                    .insert(
                        addr,
                        TX_STORE.get(&addr).ok_or_else(|| Error::RemoteAddrNotFound(addr))?.get(),
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
    type Msg = Msg;

    fn recv_msg(&self) -> Result<Self> {
        unimplemented!()
    }

    fn send_msg(&self) -> Result<Self> {
        unimplemented!()
    }
}

impl Eq for MemoryChannel {}

impl PartialEq for MemoryChannel {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const Self == rhs as *const Self
    }
}
