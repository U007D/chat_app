use dashmap::DashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    // `Mutex<usize>` is used for `MEMORY_TRANSPORT_NEXT_ADDR` instead of `AtomicUsize` because
    // saturating arithmetic is being performed on the `usize`.  It is not possible to do this
    // atomically with `AtomicUsize` without `fetch_update`, which is not stable at the time of this
    // writing (1.44.1).
#[allow(clippy::mutex_atomic)]

    pub static ref MEMORY_TRANSPORT_NEXT_ADDR: Mutex<usize> =
    Mutex::new(0_usize);
    pub static ref TX_STORE:
        DashMap<Addr,
        TxDispenser <<<MemoryTransport as Transport>::Channel as Channel>::Msg>>
        = DashMap::new();
}
