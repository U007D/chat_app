#![allow(non_snake_case)]
pub mod fake_transport;

use crate::app::App;
use crate::ports::{Msg, Transport};

#[test]
fn recv__when_sending_app_ping_over_transport_it_responds_with_pong() {
    // Given
    let (mut local_end, remote_end) = Transport::new();
    let send_result = local_end.send(Msg::Ping);
    assert_eq!(send_result, Ok(()));

    let _sut = App::new(remote_end);

    // When
    let recv_result = local_end.recv();

    // Then
    assert_eq!(recv_result, Ok(Msg::Pong));
}
