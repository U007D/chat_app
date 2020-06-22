use crate::{adapters::transport::memory::MemoryTransport, ports::Transport};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;
type Addr = <MemoryTransport as Transport>::Addr;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error(
        "Internal Error: `TX_STORE` unexpectedly found to already contain a unique `Addr`: {}.",
        _0
    )]
    AddrAlreadyAdded(Addr),
    #[error(
        "Internal Error: False negative - Container which was verified not to already contain a \
        given `Addr` within the same critical section was found to contain the `Addr`: {}",
        _0
    )]
    AddrFalseNegative(Addr),
    #[error("Remote Addr {} not found.", _0)]
    RemoteAddrNotFound(Addr),
}
