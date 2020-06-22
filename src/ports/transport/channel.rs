use crate::error::transport::memory::channel::{Error, Result};

pub trait Channel {
    type Addr;
    type Error: Into<Error>;
    type Msg;

    fn rx(&self) -> Result<Self::Msg>
    where
        Self: Sized;
    fn tx(&self, msg: Self::Msg, addr: Self::Addr) -> Result<&Self>
    where
        Self: Sized;
}
