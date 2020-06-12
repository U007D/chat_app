#![allow(non_snake_case)]
use super::*;

#[test]
fn connect_to__a_transport_instance_can_connect_to_another_transport_instance_of_same_type() {
    // Given two transport instances
    let mut local_xp = MemoryTransport::new();
    let remote_xp = MemoryTransport::new();
    let remote_id = remote_xp.id();
    let mut sut = |target| local_xp.connmemect_to(target);

    // When a connection is attempted
    let res = sut(remote_id);

    // Then connection is been successfully established
    assert!(res.is_ok());
}
