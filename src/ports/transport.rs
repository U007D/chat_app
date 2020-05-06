use std::error::Error;
use crate::ports::{Msg, MsgTransporter};
use async_trait::async_trait;
use crate::Result;

#[async_trait]
pub trait Transport {
    type Error: Error;

    async fn recv(&self) -> Result<Msg, Self::Error>;
    async fn send(&self, message: Msg) -> Result<(), Self::Error>;
}
