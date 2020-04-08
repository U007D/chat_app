use super::*;
use net2::TcpBuilder;
use ws::{WebSocket, Sender, Factory, Handler};
use url::Url;
use std::thread;
use std::sync::mpsc::{channel, Sender as ChannelSender};
use std::time::{Duration, Instant};

struct FirstClient {
    sender: Option<ChannelSender<()>>,
}

impl Factory for FirstClient {
    type Handler = impl Fn(ws::Message) -> Result<(), ws::Error>;
    // type Handler = impl Handler;

    fn connection_made(&mut self, out: Sender) -> Self::Handler {
        out.send(bincode::serialize(&ChatMessage::Hello).unwrap()).unwrap();



        let channel_sender = self.sender.take().unwrap();

        move |msg: ws::Message| {
            panic!("inside msg handler");
            //channel_sender.send(());
            Ok(())
        }
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

    let (sender, receiver) = channel();

    // established connection
    let handler = thread::spawn(|| {
        let mut socket = WebSocket::new(FirstClient {sender: Some(sender)})
            .unwrap()
            .bind("127.0.0.2:4445")
            .unwrap();

        socket
            .connect(Url::parse("ws://127.0.0.1:4444").unwrap())
            .unwrap();

        socket
            .run()
            .unwrap();
    });

    println!("Before! {:?}", Instant::now());
    dbg!(receiver.recv_timeout(Duration::from_secs(10)));
    println!("After! {:?}", Instant::now());

    // when
    let response = send_message(&ChatMessage::Hello);

    // Then
    assert_eq!(response, ChatMessage::IpList(vec![Ipv4Addr::new(127,0,0,2)]));
}