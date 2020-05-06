use crate::ports::{Msg, Transport, MsgTransporter, Target};
use crate::Result;
use async_trait::async_trait;
use std::sync::mpsc::{channel, Sender, Receiver};
use thiserror::Error;

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
        let remote_end = FakeTransport { sender: re_sender, receiver: re_receiver };
        (local_end, remote_end)
    }
}

#[async_trait]
impl Transport for FakeTransport {
    type Error = FakeTransportError;

    async fn recv(&self) -> Result<Msg, Self::Error> {
        self.receiver.recv().into()
    }

    async fn send(&self, message: Msg) -> Result<(), Self::Error> {
        self.sender.send(message).into()
    }
}

#[derive(Debug)]
pub struct LocalEnd {
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
}

impl LocalEnd {
    pub fn send(msg: Msg) -> Result<()> {
        unimplemented!()
    }
    pub fn recv() -> Result<Msg> {
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


