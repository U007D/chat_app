use crate::{adapters::transport::memory::channel::MemoryChannel, ports::transport::Channel};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

type Addr = <MemoryChannel as Channel>::Addr;
type Msg = <MemoryChannel as Channel>::Msg;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error(
        "Internal Error: `TX_STORE` unexpectedly found to already contain a unique `Addr`: {}.",
        _0
    )]
    AddrAlreadyAdded(Addr),
    #[error("Remote Addr {} already connected.", _0)]
    AddrAlreadyConnected(Addr),
    #[error(
        "Internal Error: False negative - Container which was verified not to already contain a \
        given `Addr` within the same critical section was found to contain the `Addr`: {}",
        _0
    )]
    AddrFalseNegative(Addr),
    #[error(transparent)]
    RecvError(#[from] std::sync::mpsc::RecvError),
    #[error("Remote Addr {} not found.", _0)]
    RemoteAddrNotFound(Addr),
    #[error(transparent)]
    SendError(#[from] std::sync::mpsc::SendError<Msg>),
}
