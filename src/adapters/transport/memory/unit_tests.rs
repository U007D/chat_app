#![allow(non_snake_case)]
use super::*;
use crate::{app::Msg, ports::Transport};
use assert2::assert;

#[test]
fn connect_to__a_transport_can_connect_to_another_transport() {
    // Given two memory_transport instances
    let remote_transport = MemoryTransport::new();
    let local_transport = MemoryTransport::new();
    let mut sut = local_transport;

    // When a connection is attempted
    let res = sut.connect_to(remote_transport.addr());

    // Then connection is been successfully established
    assert!(res.is_ok(), "{:?}", res)
}
