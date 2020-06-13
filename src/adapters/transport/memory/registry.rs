use super::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct MemoryTransportRegistry(Vec<Rc<MemoryTransport>>);

impl MemoryTransportRegistry {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, mx: MemoryTransport) -> MemoryTransportId {
        self.0.push(Rc::new(mx));
        MemoryTransportId(
            self.0.len().checked_sub(1).unwrap_or_else(|| {
                unreachable!("With current item added to vec, len must be > 0.")
            }),
        )
    }

    pub fn get(&self, mx_id: MemoryTransportId) -> Option<&mut MemoryTransport> {
        self.0.get(mx_id.0).and_then(|mut rc| Rc::get_mut(&mut rc))
    }
}
