#![allow(non_snake_case)]
use super::*;
use crate::{app::Msg, error::transport::Result, ports::Transport};
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

#[test]
fn send_msg__a_transport_can_send_a_message_to_another_transport() -> Result<()> {
    // Given two memory_transport instances with a `Msg` in the receiver's receive queue
    let mut remote_txp = MemoryTransport::new();
    let remote_addr = remote_txp.addr();
    let mut sut = MemoryTransport::with_connection(remote_addr)?;

    // When a message is read
    let res = sut.tx_msg(Msg::Hello, remote_addr);

    // Then connection is been successfully established
    assert!(res.is_ok(), "{:?}", res);
    // And the `Msg` is received by the intended recipient
    assert!(remote_txp.rx_msg() == Ok(Msg::Hello));

    Ok(())
}
