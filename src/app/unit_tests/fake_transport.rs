use crate::ports::{Msg, Transport};
use crate::Result;
use async_trait::async_trait;
use std::sync::mpsc::{channel, Sender, Receiver};
use thiserror::Error;

#[derive(Debug)]
pub struct TransportEnd {
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
}

impl TransportEnd {
    pub fn new() -> (Self, Self) {
        let (le_sender, re_receiver) = channel();
        let (re_sender, le_receiver) = channel();
        let local_end = TransportEnd {
            sender: le_sender,
            receiver: le_receiver,
        };
        let remote_end = TransportEnd {
            sender: re_sender,
            receiver: re_receiver,
        };

        (local_end, remote_end)
    }
}

impl Transport for TransportEnd {
    type Error = FakeTransportError;

    fn recv(&mut self) -> Result<Msg, Self::Error> {
        Ok(self.receiver.recv()?)
    }

    fn send(&mut self, message: Msg) -> Result<(), Self::Error> {
        Ok(self.sender.send(message)?)
    }
}

#[derive(Debug, Error)]
pub enum FakeTransportError {
    #[error("Channel receiver failed: {}", 0)]
    Receive(#[from] std::sync::mpsc::RecvError),
    #[error("Channel sender failed: {}", 0)]
    Send(#[from] std::sync::mpsc::SendError<Msg>),
}


