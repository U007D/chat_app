use crate::ports::{Transport, Msg, Target};

pub trait MsgTransporter {
    fn new(source: Target, destination: Target, payload: Msg) -> Self;
    fn source(&self) -> Target,
    fn destination(&self) -> Target,
    fn payload(&self) -> Msg,
}
