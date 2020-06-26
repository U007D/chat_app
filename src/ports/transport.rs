mod envelope;
use crate::{error::transport::Error, error::Result};
pub use envelope::Envelope;

pub trait Transport {
    type Addr;
    type Envelope: Envelope<Addr = Self::Addr>;
    type Error: Into<Error>;

    fn addr(&self) -> Self::Addr;
    fn connect_to(&mut self, addr: Self::Addr) -> Result<&mut Self, Self::Error>;
    fn rx_msg(&mut self) -> Result<(<Self::Envelope as Envelope>::Msg, Self::Addr), Self::Error>;
    fn tx_msg(
        &self,
        msg: <Self::Envelope as Envelope>::Msg,
        dst: Self::Addr,
    ) -> Result<&Self, Self::Error>;
}
