pub trait Envelope {
    type Addr;
    type Msg;

    fn addr(&self) -> Self::Addr;
    fn msg(&self) -> &Self::Msg;
}
