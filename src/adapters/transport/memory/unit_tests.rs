#![allow(non_snake_case)]
use super::*;
use crate::{app::Msg, ports::Transport};
#[test]
fn connect_to__a_txp_can_connect_to_another_txp() {
    // Given two memory_transport instances
    let remote_txp = MemoryTransport::new();
    let mut local_txp = MemoryTransport::new();
    let mut sut = |addr| local_txp.connect_to(addr);

    // When a connection is attempted
    let res = sut(remote_txp.addr());

    // Then connection is been successfully established
    assert!(res.is_ok());
}

fn connect_to__a_txp_can_receive_a_message_from_another_txp() -> crate::error::transport::Result<()>
{
    // Given two memory_transport instances with a `Msg` in the receiver's receive queue
    let mut remote_txp = MemoryTransport::new();
    let local_txp = MemoryTransport::with_connection(remote_txp.addr())?;
    local_txp.send_msg(Msg::Hello)?;
    let mut sut = || remote_txp.msg();

    // When a message is read
    let res = sut();

    // Then connection is been successfully established
    assert_eq!(res, Msg::Hello);

    Ok(())
}
