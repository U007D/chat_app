#![allow(non_snake_case)]
use crate::ports::{Msg, Transport};
pub(crate) mod fake_transport;
use fake_transport::TransportEnd;
use crate::app::App;

#[test]
fn ping__live_socket_replies_to_ping_with_pong() {
    // Given
    let (mut local_end, remote_end) = TransportEnd::new();
    App::new(remote_end);

    // When
    let send_result = local_end.send(Msg::Ping);
    assert_eq!(send_result, Ok(()));

    // Then
    let recv_result = local_end.recv().unwrap();
    assert_eq!(recv_result, Ok(Msg::Pong));
}

