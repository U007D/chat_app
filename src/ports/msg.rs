#[derive(Debug, PartialEq, Clone)]
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