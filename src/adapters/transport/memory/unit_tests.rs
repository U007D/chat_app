#![allow(non_snake_case, clippy::wildcard_imports)]
use super::*;
use crate::{app::Msg, ports::transport::Transport};
use assert2::assert;

#[test]
fn connect_to__a_transport_can_connect_to_another_transport() {
    // Given two memory_transport instances
    let remote = MemoryTransport::new();
    let mut sut = MemoryTransport::new();

    // When a connection is attempted
    let res = sut.connect_to(remote.addr());

    // Then connection is been successfully established
    assert!(res.is_ok(), "{:?}", res)
}

#[test]
fn send_msg__a_transport_can_send_a_message_to_another_transport() -> Result<()> {
    // Given two memory_transport instances with a `Msg` in the receiver's receive queue
    let mut remote = dbg!(MemoryTransport::new());
    let remote_addr = remote.addr();
    let sut = dbg!(MemoryTransport::with_connection(remote_addr)?);

    // When a message is read
    let res = sut.tx_msg(Msg::Hello, remote_addr);

    // Then connection is been successfully established
    assert!(res.is_ok(), "{:?}", res);
    // And the `Msg` is received by the intended recipient
    assert!(remote.rx_msg() == Ok((&Msg::Hello, sut.addr())));

    Ok(())
}
