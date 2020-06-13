use super::MemoryTransport;

pub struct MemoryTransportId(pub(super) usize);

impl From<MemoryTransport> for MemoryTransportId {
    fn from(mx: MemoryTransport) -> Self {
        Self(&mx as usize)
    }
}
