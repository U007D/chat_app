use crate::adapters::transport::MemoryTransport;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error("Remote Transport not found: {:p}", _0)]
    RemoteTransportNotFound(*const MemoryTransport),
}
