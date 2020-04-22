use serde::export::Formatter;
use std::any::Any;
use std::fmt::Result as FmtResult;
use std::fmt::{Debug, Display};
use std::io::Error as IoError;
use thiserror::Error;
use ws::Error as WsError;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Error)]
pub enum Error {
    #[error("Error on App Start: {:?}", 0)]
    AppStartError(Box<dyn Any + Send + 'static>),
    #[error("Got an Io Error that is: {0}")]
    IoError(#[from] IoError),
    #[error("IP Type mismatched")]
    IpTypeMismatch,
    #[error("Missing `name` argument.  Type `chat_app --help` for usage information.")]
    MissingNameArg,
    #[error("No IP Address found")]
    NoIpAddrFound,
    #[error("Got a WebSocket error that is: {0}")]
    WebSocket(#[from] WsError),
}

// Rust defaults to showing the `Debug` presentation of an error when exiting from main.  To work
// around this to show the above-defined information, impl `Debug::fmt()` to call `Display::fmt()`
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <Self as Display>::fmt(self, f)
    }
}
