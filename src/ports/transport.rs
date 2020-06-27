mod envelope;
use crate::error::transport::Result;
pub use envelope::Envelope;
use std::fmt::Debug;

pub trait Transport: Debug {
    type Addr: Debug;
    type Envelope: Envelope<Addr = Self::Addr>;

    fn addr(&self) -> Self::Addr;
    fn connect_to(&mut self, addr: Self::Addr) -> Result<&mut Self>;
    fn rx_msg(&mut self) -> Result<(<Self::Envelope as Envelope>::Msg, Self::Addr)>;
    fn tx_msg(&self, msg: <Self::Envelope as Envelope>::Msg, dst: Self::Addr) -> Result<&Self>;
}
