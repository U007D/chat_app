#![allow(clippy::result_unwrap_used, clippy::wildcard_imports)]
use super::*;
use crate::{
    adapters::transport::memory::MemoryTransport, error::transport, ports::transport::Transport,
};
use assert2::assert;

#[test]
fn ping__app_responds_with_pong() -> transport::Result<()> {
    // Given an app injected with a `MemoryTransport` given a `Msg::Ping`
    let remote = MemoryTransport::new();
    let remote_addr = remote.addr();
    let mut sut = MemoryTransport::with_connection(remote_addr)?;
    let local_addr = sut.addr();

    // When sent a `Msg::Ping`
    sut.tx_msg((Msg::Ping, local_addr)).unwrap();

    // Then `Msg::Pong` should be received
    assert!(sut.rx_msg() == Ok((Msg::Pong, remote_addr)));

    Ok(())
}
