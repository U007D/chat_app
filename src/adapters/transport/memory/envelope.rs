use crate::{
    adapters::transport::memory::MemoryTransportAddr, app::Msg, ports::transport::Envelope,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryTransportEnvelope {
    msg: Msg,
    sender: MemoryTransportAddr,
}

impl Envelope for MemoryTransportEnvelope {
    type Addr = MemoryTransportAddr;
    type Msg = Msg;
}

impl From<(<MemoryTransportEnvelope as Envelope>::Msg, <MemoryTransportEnvelope as Envelope>::Addr)>
    for MemoryTransportEnvelope
{
    fn from((msg, sender): (<Self as Envelope>::Msg, <Self as Envelope>::Addr)) -> Self {
        Self {
            msg,
            sender,
        }
    }
}

impl From<MemoryTransportEnvelope>
    for (<MemoryTransportEnvelope as Envelope>::Msg, <MemoryTransportEnvelope as Envelope>::Addr)
{
    fn from(envelope: MemoryTransportEnvelope) -> Self {
        (envelope.msg, envelope.sender)
    }
}
