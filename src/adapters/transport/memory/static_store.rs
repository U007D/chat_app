use crate::{
    adapters::transport::memory::{MemoryTransport, MemoryTransportAddr, TxDispenser},
    ports::Transport,
};
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

/// All instances of `MemoryTransport` share this singleton (thread-safe) backing store.  This
/// enables enumeration and discovery and communication between all `MemoryTransport` instances.
lazy_static! {
    // `Mutex<usize>` is used for `NEXT_ADDR_STORE` instead of `AtomicUsize` because saturating
    // arithmetic is being performed on the `usize`.  It is not possible to do this atomically with
    // `AtomicUsize` at the time of this writing  (1.44.1).  `fetch_update` (stabilizes in 1.45)
    // will support this behavior on an `AtomicUsize`.
    #[allow(clippy::mutex_atomic)]
    pub(super) static ref NEXT_ADDR_STORE: Mutex<usize> = Mutex::new(0_usize);

    pub(super) static ref SENDER_STORE:
        DashMap<MemoryTransportAddr,
        TxDispenser<<MemoryTransport as Transport>::Msg>>
        = DashMap::new();
}
