#![allow(non_snake_case, clippy::result_unwrap_used, clippy::wildcard_imports)]
use super::*;
use crate::{
    adapters::transport::memory::MemoryTransport, error::transport, ports::transport::Transport,
};
use assert2::assert;
use std::thread::spawn;

#[test]
fn ping__app_responds_with_pong() -> transport::Result<()> {
    // Given an app injected with a `MemoryTransport` given a `Msg::Ping`
    let mut remote = MemoryTransport::new();
    let remote_addr = remote.addr();

    let mut local = MemoryTransport::with_connection(remote_addr)?;
    let local_addr = local.addr();
    //let handle = spawn(move || app.run());
    remote.connect_to(local_addr);
    let app = App::new(remote);

    // When sent a `Msg::Ping`
    local.tx_msg(Msg::Ping, remote_addr).unwrap();
    app.run();
    // Then `Msg::Pong` should be received
    assert!(local.rx_msg() == Ok((Msg::Pong, remote_addr)));

    Ok(())
}
