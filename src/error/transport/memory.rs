pub mod channel;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error(transparent)]
    MemoryChannelError(#[from] channel::Error),
    #[error("Internal Error: Unable to acquire lock on `MEMORY_TRANSPORT_NEXT_ADDR`.")]
    NextAddrLockFailed,
    #[error("Too many `MemoryTransport` instances (> {}) created.", usize::max_value())]
    TooManyInstances,
}
