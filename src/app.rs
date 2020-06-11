mod msg;

use crate::ports::ITransport;
pub use msg::Msg;

struct App<Transport: ITransport> {
    transport: Transport,
}
