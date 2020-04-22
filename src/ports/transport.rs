use std::error::Error;
use crate::ports::{Msg, Transporter};

pub trait Transport {
    type Error: Error;

    fn rx<T: Transporter>(&self) -> Result<T, Self::Error>;
    fn tx<T: Transporter>(&self, T) -> Result<(), Self::Error>;
}
