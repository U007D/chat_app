use std::any::Any;
use std::io::Error as IoError;
use thiserror::Error;
use ws::Error as WsError;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("No IP Address found")]
    NoIpAddrFound,
    #[error("IP Type mismatched")]
    IpTypeMismatch,
    #[error("Got an Io Error that is: {0}")]
    IoError(#[from] IoError),
    #[error("Got a WebSocket error that is: {0}")]
    WebSocket(#[from] WsError),
    #[error("Got error on App Start: {0:?}")]
    AppStartError(Box<dyn Any + Send + 'static>),
}
