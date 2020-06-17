use super::*;
use crate::error::transport::memory::registry::{Error, Result};
use std::rc::Rc;

#[derive(Debug)]
pub struct MemoryTransportRegistry(Vec<Rc<MemoryTransport>>);

impl MemoryTransportRegistry {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, mx: MemoryTransport) -> MemoryTransportAddr {
        let id = self.0.len();
        self.0.push(Rc::new(mx));
        MemoryTransportAddr(id)
    }

    pub fn get(&self, mx_id: MemoryTransportAddr) -> Result<&mut MemoryTransport> {
        self.0.get(mx_id.0).map_or_else(
            || Err(Error::IndexOutOfBounds(mx_id)),
            |mut rc| {
                Rc::get_mut(&mut rc).ok_or_else(|| Error::ExistingReferenceProhibitsExclusiveAccess)
            },
        )
    }
}
