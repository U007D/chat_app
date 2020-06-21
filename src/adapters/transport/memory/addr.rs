use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct MemoryTransportAddr(usize);

impl Display for MemoryTransportAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#0x}", self.0)
    }
}

impl From<usize> for MemoryTransportAddr {
    fn from(n: usize) -> Self {
        Self(n)
    }
}
