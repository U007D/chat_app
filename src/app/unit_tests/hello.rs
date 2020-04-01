use super::*;
use net2::TcpBuilder;
use ws::{WebSocket, Sender, Factory, Handler};
use url::Url;

struct SecondClient {

}

impl Factory for SecondClient {
    type Handler = impl Handler;

    fn connection_made(&mut self, out: Sender) -> Self::Handler {
        out.send(bincode::serialize(&ChatMessage::Hello).unwrap()).unwrap();

        // move |msg: ws::Message| {
        //     // match msg {
        //     //     ws::Message::Binary(data) => {
        //     //     },
        //     //     _ => panic!("We expected a ws::Message::Binary")
        //     // };
        // }
    }
}

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
fn hello__second_client_receives_non_empty_ip_list() {
    // Given
    let _app = App::start().unwrap();

    // established connecgtion
    let socket = WebSocket::new(SecondClient {})
        .unwrap()
        .bind("127.0.0.2:4445")
        .unwrap()
        .connect(Url::Parse("ws://127.0.0.1:4444"))
        .unwrap()
        .run()
        .unwrap();


    // when
    let response = send_message(&ChatMessage::Hello);

    // Then
    assert_eq!(response, ChatMessage::IpList(vec![Ipv4Addr::new(127,0,0,2)]));
}