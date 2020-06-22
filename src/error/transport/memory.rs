mod channel;
use crate::ports::transport::Channel;
use crate::{
    adapters::transport::memory::{Addr, MemoryTransport},
    ports::Transport,
};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error("Remote Addr {} already connected.", _0)]
    AddrAlreadyConnected(Addr),
    #[error(transparent)]
    MemoryChannelError(#[from] channel::Error),
    #[error("Internal Error: Unable to acquire lock on `MEMORY_TRANSPORT_NEXT_ADDR`.")]
    NextAddrLockFailed,
    #[error(transparent)]
    RecvError(#[from] std::sync::mpsc::RecvError),
    #[error(transparent)]
    SendError(
        #[from]
        std::sync::mpsc::SendError<<<MemoryTransport as Transport>::Channel as Channel>::Msg>,
    ),
    #[error("Too many `MemoryTransport` instances (> {}) created.", usize::max_value())]
    TooManyInstances,
}
