mod envelope;
use crate::error::transport::Result;
pub use envelope::Envelope;
use std::fmt::Debug;

pub trait Transport {
    type Addr: Debug;
    type Envelope: Envelope<Addr = Self::Addr>;

    fn addr(&self) -> Self::Addr;
    fn rx_msg(&self) -> Result<(<Self::Envelope as Envelope>::Msg, Self::Addr)>;
    fn tx_msg(&self, msg: <Self::Envelope as Envelope>::Msg, dst: Self::Addr) -> Result<&Self>;
}
