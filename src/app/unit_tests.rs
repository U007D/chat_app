use crate::ports::Msg;
mod fake_transport;
use fake_transport::FakeTransport;
use crate::app::App;

#[test]
fn ping__live_socket_replies_to_ping_with_pong() {
    // Given
    let (local_end, remote_end) = FakeTransport::new();
    let app = App::new(remote_end);

    // When
    local_end.send(&Msg::Ping).unwrap();

    // Then
    assert_eq!(local_end.recv().unwrap(), Msg::Pong);
}

