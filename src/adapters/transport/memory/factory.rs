use crate::{
    adapters::transport::{MemoryTransport, MemoryTransportAddr},
    error::transport::memory::factory::Result,
};
use crossbeam_utils::atomic::AtomicCell;
use futures::TryFutureExt;
use std::sync::atomic::AtomicUsize;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc, RwLock,
    },
};

static NULL_FACTORY: &mut Arc<RwLock<MemoryTransportFactory>> =
    &mut 0 as &mut Arc<RwLock<MemoryTransportFactory>>;

static MEMORY_TRANSPORT_FACTORY: Arc<RwLock<MemoryTransportFactory>> =
    Arc::new(RwLock::new(Option::<MemoryTransportFactoryState>));

#[derive(Debug)]
struct MemoryTransportFactory {
    addr: AtomicUsize,
    transports: HashMap<MemoryTransportAddr, MemoryTransport>,
}

impl MemoryTransportFactory {
    pub fn new() -> Result<Arc<RwLock<MemoryTransportFactoryState>>> {
        // Initialize singleton factory, if required
        if let write_guard = MEMORY_TRANSPORT_FACTORY.try_write()? {
            *write_guard.get_or_insert_with(|| {
                *write_guard = Some(MemoryTransportFactory {
                    addr: AtomicUsize::new(0),
                    transports: HashMap::new(),
                })
            });
        }
        // Synchronize in the event of write-contention (wait for other task to initialize)
        MEMORY_TRANSPORT_FACTORY.try_read()?;

        // MEMORY_TRANSPORT_FACTORY has unconditionally been initialized exactly once at this point
        Ok(Arc::clone(&MEMORY_TRANSPORT_FACTORY))
    }

    pub fn make_memory_transport(&mut self) -> &MemoryTransport {
        let addr = self.addr.fetch_add(1, Ordering::SeqCst);
        // Detect and panic on `self.addr` overflow
        assert!(
            self.addr.load(Ordering::SeqCst) > addr,
            "`MemoryTransportFactory::addr` overflow."
        );

        let mta = MemoryTransportAddr::new(addr);
        self.transports
            .insert(mta, MemoryTransport::new(mta))
            .and_then(|orig| {
                unreachable!(
                    "Internal error: Duplicate address detected for index {}: {:?}",
                    mta, orig
                )
            });

        self.transports
            .get(&mta)
            .expect("Internal error: Just-added-item not found in Transport Registry.")
    }
}
