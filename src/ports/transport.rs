use std::error::Error;
use crate::ports::{Msg, MsgTransporter};

pub trait Transport {
    type Error: Error;

    fn recv(&self) -> Result<MsgTransporter, Self::Error>;
    fn send(&self, MsgTransporter) -> Result<(), Self::Error>;
}
