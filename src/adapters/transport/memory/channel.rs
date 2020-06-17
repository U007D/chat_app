use crate::app::Msg;
use crate::{error::transport::channel::Result, ports::transport::Channel};
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct MemoryChannel {
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
}

impl MemoryChannel {
    pub fn new(tx: Sender<Msg>, rx: Receiver<Msg>) -> Self {
        Self { tx, rx }
    }
}

impl Channel for MemoryChannel {
    fn recv_msg(&self) -> Result<Self> {
        unimplemented!()
    }

    fn send_msg(&self) -> Result<Self> {
        unimplemented!()
    }
}

impl Eq for MemoryChannel {}

impl PartialEq for MemoryChannel {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const Self == rhs as *const Self
    }
}
