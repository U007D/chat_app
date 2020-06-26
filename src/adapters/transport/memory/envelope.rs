use crate::{
    adapters::transport::memory::MemoryTransportAddr, app::Msg, ports::transport::Envelope,
};

#[derive(Debug)]
pub struct MemoryTransportEnvelope {
    msg: Msg,
    sender: MemoryTransportAddr,
}

impl MemoryTransportEnvelope {
    pub fn new(msg: <Self as Envelope>::Msg, sender: MemoryTransportAddr) -> Self {
        Self {
            msg,
            sender,
        }
    }
}

impl Envelope for MemoryTransportEnvelope {
    type Addr = MemoryTransportAddr;
    type Msg = Msg;

    fn addr(&self) -> Self::Addr {
        self.sender
    }

    fn msg(&self) -> &Self::Msg {
        &self.msg
    }
}

impl From<(Self::Msg, Self::Addr)> for MemoryTransportEnvelope {
    fn from((msg, sender): (Self::Msg, Self::Addr)) -> Self {
        Self::new(msg, sender)
    }
}
