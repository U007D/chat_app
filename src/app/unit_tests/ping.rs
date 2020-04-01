use super::*;

// https://github.com/housleyjk/ws-rs/blob/master/examples/client.rs
#[test]
fn ping__live_socket_replies_to_ping_with_pong() {
    // Given
    let _app = App::start().unwrap();

    // When
    let response = send_message(&ChatMessage::Ping);

    // Then
    assert_eq!(response, ChatMessage::Pong);
}

#[test]
fn ping__live_socket_replies_to_pong_with_unexpected_message_message() {
    // Given
    let _app = App::start().unwrap();

    // when
    let response = send_message(&ChatMessage::Pong);

    // Then
    assert_eq!(response, ChatMessage::UnexpectedMessage(Box::new(ChatMessage::Pong)));
}