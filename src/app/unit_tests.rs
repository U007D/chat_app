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

pub enum Message {
    Ping,
    Hello,
}

#[test]
fn start__app_starts() -> Result<()> {
    // Given
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
fn ping__live_socket_replies_with_pong() {
    // Given
    let expected_socket = SocketAddr::from(([127, 0, 0, 1], 4444));
    let stringly_socket = expected_socket.to_string();
    let app = App::start().unwrap();
    let listener_socket = app.local_socket;
    let msg_string = RefCell::new(String::new());
    let msg_string_ref = &msg_string;

    // When
    let sut = connect(stringly_socket, |out| {
        out.send("ping").unwrap();

        move |msg: ws::Message| {
            println!("Received message {:?}", msg);
            (*msg_string_ref.borrow_mut()) = msg.to_string();
            out.close(CloseCode::Normal)
        }
    })
    .unwrap();

    // Then
    assert_eq!(&*msg_string.borrow(), "pong");
}
