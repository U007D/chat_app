use crate::ports::{Msg, Transport};
use crate::Result;
use async_trait::async_trait;
use futures::channel::mpsc::{channel, Sender, Receiver};
use thiserror::Error;
use futures::{StreamExt, SinkExt};

#[derive(Debug)]
pub struct FakeTransport {
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
}

impl FakeTransport {
    pub fn new() -> (LocalEnd, Self) {
        let (le_sender, re_receiver) = channel();
        let (re_sender, le_receiver) = channel();
        let local_end = LocalEnd {
            sender: le_sender,
            receiver: le_receiver
        };
        let remote_end = FakeTransport {
            sender: re_sender,
            receiver: re_receiver
        };

        (local_end, remote_end)
    }
}

#[async_trait]
impl Transport for FakeTransport {
    type Error = FakeTransportError;

    async fn recv(&mut self) -> Result<Msg, Self::Error> {
        Ok(self.receiver.next().await?)
    }

    async fn send(&mut self, message: Msg) -> Result<(), Self::Error> {
        Ok(self.sender.send(message).await?)
    }
}

#[derive(Debug)]
pub struct LocalEnd {
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
}

impl LocalEnd {
    pub fn send(&self, msg: &Msg) -> Result<()> {
        unimplemented!()
    }
    pub fn recv(&self) -> Result<Msg> {
        unimplemented!()
    }
}

#[derive(Debug, Error)]
pub enum FakeTransportError {
    #[error("Channel receiver failed: {}", 0)]
    Receive(#[from] std::sync::mpsc::RecvError),
    #[error("Channel sender failed: {}", 0)]
    Send(#[from] std::sync::mpsc::SendError<Msg>)
}


