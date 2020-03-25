use super::*;
use crate::{Error, Result};
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, RefCell};
use std::net::{Ipv4Addr, TcpListener};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::RwLock;
use ws::{connect, CloseCode, Handler};
use std::{time, thread};

pub enum Message {
    Ping,
    Hello,
}

#[test]
fn start__app_starts() -> Result<()> {
    // Given
    // Using localhost because we assume it shows up first in the list
    let expected_socket = SocketAddr::from(([127, 0, 0, 1], 4444));
    let sut = App::start;

    // When
    let actual_app = sut()?;

    // Then
    assert_eq!(actual_app.local_socket, expected_socket);
    Ok(())
}

// https://github.com/housleyjk/ws-rs/blob/master/examples/client.rs
#[test]
fn ping__live_socket_replies_to_ping_with_pong() {
    // Given
    let url = "ws://127.0.0.1:4444";
    let app = App::start().unwrap();
    let msg = RefCell::new(None);
    let msg_ref = &msg;

    // When
    let sut = connect(url, |out| {
        out.send(bincode::serialize(&ChatMessage::Ping).unwrap()).unwrap();
        move |msg: ws::Message| {
            match msg {
                ws::Message::Binary(data) => {
                    (*msg_ref.borrow_mut()) = Some(bincode::deserialize(&data[..]).unwrap());
                    out.close(CloseCode::Normal)
                },
                _ => panic!("We expected a ws::Message::Binary")
            }
        }
    })
    .unwrap();

    // Then
    assert_eq!(*msg.borrow(), Some(ChatMessage::Pong));
}

#[test]
fn ping__live_socket_replies_to_pong_with_unexpected_message_message() {
    // Given
    let url = "ws://127.0.0.1:4444";
    let app = App::start().unwrap();
    let listener_socket = app.local_socket;
    let msg = RefCell::new(None);
    let msg_ref = &msg;

    // When
    let sut = connect(url, |out| {
        // Use a ref because bincode is efficient in how it serializes into vec of bytes
        out.send(bincode::serialize(&ChatMessage::Pong).unwrap()).unwrap();
        move |msg: ws::Message| {
            match msg {
                // Calling this for a side effect
                ws::Message::Binary(data) => {
                    (*msg_ref.borrow_mut()) = Some(bincode::deserialize(&data[..]).unwrap());
                    out.close(CloseCode::Normal)
                },
                _ => panic!("We expected a ws::Message::Binary")
            }
        }
    })
        .unwrap();

    // Then
    assert_eq!(*msg.borrow(), Some(ChatMessage::UnexpectedMessage(Box::new(ChatMessage::Pong))));
}

// TODO - Unit test for unexpected return values of app start
// Unexpected ChatMessage (anything other than Ping) - returns the content of one of the errors - error is the payload of the chat message I am already sending

#[test]
fn unexpected_sent_message_returns_descriptive_error() {
    // Given
    let url = "ws://127.0.0.1:4444";
    let app = App::start().unwrap();
    let msg = RefCell::new(None);
    let msg_ref = &msg;
    let bad_message = ChatMessage::Pong;

    //When
    let sut = connect(url, |out| {
        out.send(bincode::serialize(&bad_message).unwrap()).unwrap();
        move |msg: ws::Message| {
            match msg {
                ws::Message::Binary(data) => {
                    (*msg_ref.borrow_mut()) = Some(bincode::deserialize(&data[..]).unwrap());
                    out.close(CloseCode::Normal)
                },
                _ => panic!("We expected a ws::Message::Binary")
            }
        }
    })
        .unwrap();

    //Then
     assert_eq!(*msg.borrow(), Some(ChatMessage::UnexpectedMessage(Box::new(bad_message))));
}
// ws Text instead of binary
// Deserialize failure (nested match)
