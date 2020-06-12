use crate::ports::transport::Channel;

#[derive(Debug)]
pub struct MemoryChannel {}

impl Channel for MemoryChannel {}
