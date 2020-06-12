#[cfg(test)]
pub mod memory;

use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[cfg(test)]
    #[error(transparent)]
    MemoryTransportError(#[from] memory::Error),
}
