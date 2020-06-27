#[cfg(test)]
mod unit_tests;

mod msg;

use crate::error::transport;
use crate::{
    ports::transport::{Envelope, Transport},
    Result,
};
pub use msg::Msg;

struct App<TransportImpl: Transport> {
    transport: TransportImpl,
}

impl<TransportImpl> App<TransportImpl>
where
    TransportImpl: Transport,
    <TransportImpl as Transport>::Envelope: Envelope<Msg = Msg>,
{
    pub fn new(transport: TransportImpl) -> Self {
        Self {
            transport,
        }
    }

    pub fn run(self) -> Result<()> {
        loop {
            match self.transport.rx_msg().map_err(transport::Error::from)? {
                (Msg::Ping, origin) => self.transport.tx_msg(Msg::Pong, origin),
                _ => unimplemented!("App message handler: unimplemented for `Msg` received."),
            }?;
        }
        // `-> Result<!>` would be preferable, but Never type is not stable as of 1.44.1
        #[allow(unreachable_code)]
        Ok(())
    }
}
