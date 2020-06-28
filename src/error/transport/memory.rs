use crate::{
    adapters::transport::memory::{MemoryTransport, MemoryTransportAddr},
    ports::transport::Transport,
};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error(
        "Internal Error: `SENDER_STORE` unexpectedly found to already contain a unique `Addr`: {}.",
        _0
    )]
    AddrAlreadyAdded(MemoryTransportAddr),
    #[error("Internal Error: Unable to acquire lock on `NEXT_ADDR_STORE`.")]
    NextAddrLockFailed,
    #[error(transparent)]
    RecvError(#[from] std::sync::mpsc::RecvError),
    #[error("Internal Error: Unexpectedly unable to acquire (lock) `Sender` instance.")]
    PoisonedMutexError,
    #[error(transparent)]
    SendError(#[from] std::sync::mpsc::SendError<<MemoryTransport as Transport>::Envelope>),
    #[error("Remote Addr {} not found.", _0)]
    RemoteAddrNotFound(MemoryTransportAddr),
    #[error("Too many `MemoryTransport` instances (> {}) created.", usize::max_value())]
    TooManyInstances,
}

// When `T` contains `std::sync::Sender`, as it does for `MemoryTransport`, an error such as
// `Error::PoisonedMutexError(#[from] PoisonError) is not `Sync` (`Sender` is `!Sync`). Newtyping
// `PoisonError` does not work either because `PoisonError` does not implement `Clone`,
// `Eq`/`PartialEq`, or even `Debug` (!).  Simpler just to drop the `PoisonError`.
impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::PoisonedMutexError
    }
}
