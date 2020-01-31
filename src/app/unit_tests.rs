use crate::{
    Error,
    Result
};
use super::*;
use std::net::{TcpListener, Ipv4Addr};
use ws::{connect, CloseCode, Handler};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub enum Message {
    Ping,
    Hello
}

#[test]
fn start__app_starts() -> Result<()> {
    // Given
    let expected_socket = SocketAddr::from(([127,0,0,1], 4444));
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
    let expected_socket = SocketAddr::from(([127,0,0,1], 4444));
    let stringly_socket = expected_socket.to_string();
    let app = App::start().unwrap();
    let listener_socket = app.local_socket;
    let refcell = RefCell::<String>::new(String::new());
    let alt_input_string = refcell.clone();
    let input_string = Rc::<RefCell<String>>::new(refcell);

    // When
    let sut = connect(stringly_socket, |out| {    let alt_input_string = refcell.clone();

        out.send("ping").unwrap();

        move |msg: ws::Message| {
            println!("Received message {:?}", &msg);
            *input_string.borrow_mut() = msg.to_string();
            out.close(CloseCode::Normal)
        }
    }).unwrap();

    // Then
    assert_eq!(alt_input_string, RefCell::new(String::from("pong")));
}
