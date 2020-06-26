use super::transport;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Clone, Error, PartialEq)]
pub enum Error {
    #[error(transparent)]
    TransportError(#[from] transport::Error),
}

// Rust unfortunately defaults to showing the `Debug` presentation of an error when exiting from
// main.  To show the nicely formatted `Display` presentation, we override this default behavior.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <Self as Display>::fmt(self, f)
    }
}
