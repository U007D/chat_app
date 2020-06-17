use crate::adapters::transport::{MemoryTransport, MemoryTransportAddr};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, sync::atomic::AtomicUsize};

pub static MEMORY_TRANSPORT_FACTORY: MemoryTransportFactory = MemoryTransportFactory::new();

#[derive(Debug, Default)]
pub struct MemoryTransportFactory {
    next_addr: AtomicUsize,
    transports: HashMap<MemoryTransportAddr, Arc<Mutex<MemoryTransport>>>,
}

impl MemoryTransportFactory {
    const fn new() -> Self {
        Self {
            next_addr: AtomicUsize::new(0),
            transports: HashMap::new(),
        }
    }

    pub fn make_memory_transport(&mut self) -> Arc<Mutex<MemoryTransport>> {
        let addr = self.next_addr.fetch_add(1, Ordering::SeqCst);
        // Detect and panic on `self.addr` overflow
        assert!(
            self.next_addr.load(Ordering::SeqCst) > addr,
            "`MemoryTransportFactory::addr` overflow."
        );

        let mta = MemoryTransportAddr::from(addr);
        self.transports
            .insert(mta, Arc::new(Mutex::new(MemoryTransport::new(mta))))
            .and_then(|orig| {
                unreachable!(
                    "Internal error: Duplicate address detected for index {}: {:?}",
                    mta, orig
                )
            });

        Arc::clone(
            self.transports
                .get(&mta)
                .expect("Internal error: Just-added-item not found in Transport Registry."),
        )
    }
}
