#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Msg {
    Hello,
    /*IpList(Vec<Ipv4Addr>),*/
    Ping,
    Pong,
    CouldNotSerialize,
    UnexpectedMessage(Box<Msg>),
    CouldNotDeserialize,
    NonBinaryMessageReceived,
}
