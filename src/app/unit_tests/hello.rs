use super::*;

#[test]
fn hello__new_server_responds_with_empty_ip_list() {
    // Given
    let _app = App::start().unwrap();

    // when
    let response = send_message(&ChatMessage::Hello);

    // Then
    assert_eq!(response, ChatMessage::IpList(Vec::<Ipv4Addr>::new()));
}

#[test]
fn hello__() {
    // Given
    let _app = App::start().unwrap();

    // when
    let response = send_message(&ChatMessage::Hello);

    // Then
    assert_eq!(response, ChatMessage::IpList(Vec::<Ipv4Addr>::new()));
}