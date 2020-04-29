use std::error::Error;
use crate::ports::{Msg, MsgTransporter};
use async_trait::async_trait;

#[async_trait]
pub trait Transport {
    type Error: Error;

    async fn recv<T: MsgTransporter>(&self) -> Result<T, Self::Error>;
    async fn send<T: MsgTransporter>(&self, transporter: T) -> Result<(), Self::Error>;
}
