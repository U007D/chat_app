// `Mutex<usize>` is used for `NEXT_ADDR_STORE` instead of `AtomicUsize` because saturating
// arithmetic is being performed on the `usize`.  It is not possible to do this atomically with
// `AtomicUsize` at the time of this writing  (1.44.1), although `fetch_update` (due to
// stabilize in 1.45) will enable this behavior on an `Atomic*`.
#![allow(clippy::mutex_atomic)]

#[cfg(test)]
mod unit_tests;

use crate::{adapters::transport::memory::MemoryTransportAddr, error::transport::memory::Error};
use bool_ext::BoolExt;
use std::sync::Mutex;

#[derive(Debug)]
pub struct UniqueAddr(Mutex<usize>);

impl UniqueAddr {
    pub fn new() -> Self {
        Self(Mutex::new(0))
    }

    // Atomically increment unique addr, but panic if it would overflow-- i.e. behave as if
    // performing `panicking_add()`.
    pub fn addr(&self) -> MemoryTransportAddr {
        let mut guard = self.0.lock().unwrap_or_else(|_| unreachable!(Error::NextAddrLockFailed));
        let addr = *guard;
        let next_addr = addr.saturating_add(1);
        (addr != next_addr)
            .some_with(|| *guard = next_addr)
            .unwrap_or_else(|| panic!(Error::TooManyInstances));
        MemoryTransportAddr::from(addr)
    }
}
