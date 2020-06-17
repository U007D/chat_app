#![allow(non_snake_case)]
use super::*;

#[test]
fn connect_to__a_mxp_can_connect_to_another_mxp() {
    // Given two memory transport instances
    let mut local_mxp = MemoryTransport::new();
    let remote_mxp = MemoryTransport::new();
    let mut sut = |target| local_mxp.connect_to(target);

    // When a connection is attempted
    let res = sut(remote_mxp.addr());

    // Then connection is been successfully established
    assert!(res.is_ok());
}

fn connect_to__a_mxp_can_receive_a_message_from_another_mxp() -> Result<()> {
    // Given two memory transport instances with a `Msg` in the receiver's receive queue
    let mut local_mxp = MemoryTransport::new();
    let remote_mxp = MemoryTransport::new();
    local_mxp
        .connect_to(remote_mxp.addr())?
        .send_msg(Msg::Hello)?;
    let sut = |sender| remote_mxp.chan(sender).recv_msg();

    // When a message is read
    let res = sut(local_mxp.addr());

    // Then connection is been successfully established
    assert_eq!(res, Ok(Msg::Hello));

    Ok(())
}
