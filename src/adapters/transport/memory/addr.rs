use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Addr(usize);

impl Display for Addr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#0x}", self.0)
    }
}

impl From<usize> for Addr {
    fn from(n: usize) -> Self {
        Self(n)
    }
}
