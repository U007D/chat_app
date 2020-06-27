mod unique_addr;

use crate::{
    adapters::transport::memory::{MemoryTransport, MemoryTransportAddr, TxSharer},
    ports::transport::Transport,
};
use dashmap::DashMap;
use lazy_static::lazy_static;
use unique_addr::UniqueAddr;

// All instances of `MemoryTransport` share this singleton (thread-safe) backing store.  This
// enables enumeration and discovery and communication between all `MemoryTransport` instances.

lazy_static! {
    pub(super) static ref UNIQUE_ADDR: UniqueAddr = UniqueAddr::new();
    pub(super) static ref SENDER_STORE: DashMap<MemoryTransportAddr, TxSharer<<MemoryTransport as Transport>::Envelope>> =
        DashMap::new();
}
