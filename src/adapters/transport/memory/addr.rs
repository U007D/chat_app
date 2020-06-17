use super::MemoryTransport;

#[derive(Debug)]
pub struct MemoryTransportAddr(pub(super) usize);

impl From<MemoryTransport> for MemoryTransportAddr {
    fn from(mx: MemoryTransport) -> Self {
        Self(&mx as usize)
    }
}
