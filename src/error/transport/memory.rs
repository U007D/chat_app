use crate::ports::transport::Channel;
use crate::{
    adapters::transport::memory::{MemoryTransport, MemoryTransportAddr},
    ports::Transport,
};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error("Remote Addr {} already connected.", _0)]
    AddrAlreadyConnected(MemoryTransportAddr),
    #[error(
        "Internal Error: `SENDER_STORE` unexpectedly found to already contain a unique `Addr`: {}.",
        _0
    )]
    AddrAlreadyAdded(MemoryTransportAddr),
    #[error(
        "Internal Error: False negative - Container which was verified not to already contain a \
        given `Addr` within the same critical section was found to contain the `Addr`: {}",
        _0
    )]
    AddrFalseNegative(MemoryTransportAddr),
    #[error("Internal Error: Unable to acquire lock on `MEMORY_TRANSPORT_NEXT_ADDR`.")]
    NextAddrLockFailed,
    #[error(transparent)]
    RecvError(#[from] std::sync::mpsc::RecvError),
    #[error(transparent)]
    SendError(
        #[from]
        std::sync::mpsc::SendError<<<MemoryTransport as Transport>::Channel as Channel>::Msg>,
    ),
    #[error("Remote Addr {} not found.", _0)]
    RemoteAddrNotFound(MemoryTransportAddr),
    #[error("Too many `MemoryTransport` instances (> {}) created.", usize::max_value())]
    TooManyInstances,
}
