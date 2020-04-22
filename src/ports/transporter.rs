use crate::ports::{Transport, Msg, Target};

pub trait Transporter {
    fn new(source: Target, destination: Target, payload: Msg) -> Self;
    fn source(&self) -> Target,
    fn destination(&self) -> Target,
    fn payload(&self) -> Msg,
}
