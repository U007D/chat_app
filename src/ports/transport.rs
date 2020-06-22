use crate::{error::transport::Error, error::Result};

pub trait Transport {
    type Addr;
    type Error: Into<Error>;
    type Msg;

    fn addr(&self) -> Self::Addr;
    fn connect_to(&mut self, addr: Self::Addr) -> Result<&mut Self, Self::Error>;
    fn rx_msg(&mut self) -> Result<Self::Msg, Self::Error>;
    fn tx_msg(&self, msg: Self::Msg, addr: Self::Addr) -> Result<&Self, Self::Error>;
}
