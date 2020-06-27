#![allow(non_snake_case, clippy::result_unwrap_used, clippy::wildcard_imports)]
use super::*;
use crate::{app::Msg, ports::transport::Transport};
use assert2::assert;

#[test]
fn tx_msg__a_transport_can_send_a_message_to_another_transport() {
    // Given two memory_transport instances with a `Msg` in the receiver's receive queue
    let remote = MemoryTransport::new();
    let local = MemoryTransport::new();

    // When a message is sent
    local.tx_msg(Msg::Hello, remote.addr()).unwrap();

    // Then the `Msg` is received by the intended recipient
    assert!(remote.rx_msg() == Ok((Msg::Hello, local.addr())));
}
