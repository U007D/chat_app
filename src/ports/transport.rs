use std::error::Error;
use crate::ports::{Msg};
use async_trait::async_trait;
use crate::Result;

#[async_trait]
pub trait Transport {
    type Error: Error;

    async fn recv(&mut self) -> Result<Msg, Self::Error>;
    async fn send(&mut self, message: Msg) -> Result<(), Self::Error>;
}
