use super::*;
use std::sync::mpsc::{channel, Sender as ChannelSender};
use std::time::{Duration, Instant};
use url::Url;
use ws::{Factory, Sender, WebSocket};

#[derive(Debug)]
struct FirstClient {
    sender: Option<ChannelSender<()>>,
}

impl Factory for FirstClient {
    type Handler = impl Fn(ws::Message) -> Result<(), ws::Error>;
    // type Handler = impl Handler;

    fn connection_made(&mut self, out: Sender) -> Self::Handler {
        out.send(bincode::serialize(&ChatMessage::Hello).unwrap())
            .unwrap();

        let channel_sender = self.sender.take().unwrap();

        move |_msg: ws::Message| {
            channel_sender.send(()).map_err(|e| {
                ws::Error::new(
                    ws::ErrorKind::Custom(Box::new(e)),
                    "std::sync::mpsc::SendError",
                )
            })
        }
    }
}

#[test]
fn hello__new_server_responds_with_empty_ip_list() {
    // Given
    let _app = App::start(PORT).unwrap();

    // when
    let response = send_message(&ChatMessage::Hello);

    // Then
    assert_eq!(response, ChatMessage::IpList(Vec::<Ipv4Addr>::new()));
}

#[allow(clippy::let_unit_value)]
#[test]
fn hello__second_client_receives_non_empty_ip_list() {
    // Given
    let _app = App::start(PORT).unwrap();

    let (sender, receiver) = channel();

    // established connection
    let handler = /*thread::spawn(move ||*/ {
        let opt_sender: Option<ChannelSender<()>> = Some(sender);
        let first_client = FirstClient {
            sender: opt_sender,
        };
        let mut socket = WebSocket::new(first_client).unwrap().bind("127.0.0.2:4445")
            .unwrap();

        socket
            .connect(Url::parse("ws://127.0.0.1:4444").unwrap())
            .unwrap();

        socket.run().unwrap();
    };

    dbg!(handler);

    println!("Before! {:?}", Instant::now());
    dbg!(receiver.recv_timeout(Duration::from_secs(10)));
    println!("After! {:?}", Instant::now());

    // when
    let response = send_message(&ChatMessage::Hello);

    // Then
    assert_eq!(
        response,
        ChatMessage::IpList(vec![Ipv4Addr::new(127, 0, 0, 2)])
    );
}
