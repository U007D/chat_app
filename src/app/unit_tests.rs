#![allow(non_snake_case, clippy::result_unwrap_used, clippy::wildcard_imports)]
use super::*;
use crate::{adapters::transport::memory::MemoryTransport, ports::transport::Transport};
use assert2::assert;
use std::thread::spawn;

#[test]
fn ping__app_responds_with_pong() {
    // Given an app injected with a `MemoryTransport` given a `Msg::Ping`
    let local = MemoryTransport::new();
    let remote = MemoryTransport::new();
    let remote_addr = remote.addr();
    let app = App::new(remote);

    let _ = spawn(move || app.run());

    // When sent a `Msg::Ping`
    local.tx_msg(Msg::Ping, remote_addr).unwrap();

    // Then `Msg::Pong` should be received
    assert!(local.rx_msg() == Ok((Msg::Pong, remote_addr)));
}
