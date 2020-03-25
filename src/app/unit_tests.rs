#![allow(non_snake_case)]
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

fn send_message( message: &ChatMessage) -> ChatMessage {
    let msg: RefCell<Option<ChatMessage>> = RefCell::new(None);
    let msg_ref = &msg;

    let sut = connect("ws://127.0.0.1:4444", |out| {
        out.send(bincode::serialize(message).unwrap()).unwrap();
        move |msg: ws::Message| {
            match msg {
                ws::Message::Binary(data) => {
                    (*msg_ref.borrow_mut()) = Some(bincode::deserialize(&data[..]).unwrap());
                    out.close(CloseCode::Normal)
                },
                _ => panic!("We expected a ws::Message::Binary")
            }
        }
    }).unwrap();

    let response = msg.borrow().as_ref().unwrap().clone();
    response
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
    let app = App::start().unwrap();

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

#[test]
fn hello__new_server_responds_with_empty_ip_list() {
    // Given
    let _app = App::start().unwrap();

    // when
    let response = send_message(&ChatMessage::Hello);
    // Then
    assert_eq!(response, ChatMessage::IpList(Vec::<Ipv4Addr>::new()));
}
