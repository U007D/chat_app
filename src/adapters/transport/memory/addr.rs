#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct MemoryTransportAddr(usize);

impl From<usize> for MemoryTransportAddr {
    fn from(n: usize) -> Self {
        Self(n)
    }
}
