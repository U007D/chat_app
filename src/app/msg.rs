#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Msg {
    Hello,
    /*IpList(Vec<Ipv4Addr>),*/
    Ping,
    Pong,
    CouldNotSerialize,
    UnexpectedMessage,
    CouldNotDeserialize,
    NonBinaryMessageReceived,
}
