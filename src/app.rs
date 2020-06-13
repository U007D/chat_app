#[cfg(test)]
mod unit_tests;

mod msg;

use crate::ports::Transport;
pub use msg::Msg;

struct App<TransportImpl: Transport> {
    transport: TransportImpl,
}

impl<TransportImpl: Transport> App<TransportImpl> {
    pub fn new(transport: TransportImpl) -> Self {
        Self { transport }
    }
}
