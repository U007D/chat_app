use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Clone, Error, PartialEq)]
pub enum Error {
    // #[error("Error on App Start: {:?}", 0)]
// AppStartError(Box<dyn Any + Send + 'static>),
// #[error("Got an Io Error that is: {0}")]
// IoError(#[from] IoError),
// #[error("IP Type mismatched")]
// IpTypeMismatch,
// #[error("Missing `name` argument.  Type `chat_app --help` for usage information.")]
// MissingNameArg,
// #[error("No IP Address found")]
// NoIpAddrFound,
// #[error("Got a WebSocket error that is: {0}")]
// WebSocket(#[from] WsError),
}

// Rust unfortunately defaults to showing the `Debug` presentation of an error when exiting from
// main.  To show the nicely formatted `Display` presentation, we override this default behavior.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <Self as Display>::fmt(self, f)
    }
}
