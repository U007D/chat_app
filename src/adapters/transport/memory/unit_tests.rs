#![allow(non_snake_case)]
use super::*;
use crate::adapters::transport::memory::factory::MEMORY_TRANSPORT_FACTORY;

#[test]
fn connect_to__a_txp_can_connect_to_another_txp() {
    // Given two memory_transport instances
    let local_txp = MEMORY_TRANSPORT_FACTORY.make_transport();
    let remote_txp = MEMORY_TRANSPORT_FACTORY.make_transport();
    let mut sut = |addr| local_txp.connect_to(addr);

    // When a connection is attempted
    let res = sut(remote_txp.addr());

    // Then connection is been successfully established
    assert!(res.is_ok());
}

fn connect_to__a_txp_can_receive_a_message_from_another_txp() -> Result<()> {
    // Given two memory_transport instances with a `Msg` in the receiver's receive queue
    let local_txp = MEMORY_TRANSPORT_FACTORY.make_transport();
    let remote_txp = MEMORY_TRANSPORT_FACTORY.make_transport();
    local_txp
        .connect_to(remote_txp.addr())?
        .send_msg(Msg::Hello)?;
    let sut = |sender| remote_txp.chan(sender).recv_msg();

    // When a message is read
    let res = sut(local_txp.addr());

    // Then connection is been successfully established
    assert_eq!(res, Ok(Msg::Hello));

    Ok(())
}
