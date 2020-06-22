use crate::error::transport::channel::Result;

pub trait Channel {
    type Msg;

    fn recv_msg(&self) -> Result<Self>
    where
        Self: Sized;
    fn send_msg(&self) -> Result<Self>
    where
        Self: Sized;
}
