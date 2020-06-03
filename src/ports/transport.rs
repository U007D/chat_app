use std::error::Error;
use crate::ports::{Msg};
use crate::Result;

pub trait Transport {
    type Error: Error;

    fn recv(&mut self) -> Result<Msg, Self::Error>;
    fn send(&mut self, message: Msg) -> Result<(), Self::Error>;
}
