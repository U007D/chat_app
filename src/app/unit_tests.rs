#![allow(non_snake_case)]

mod start;
mod ping;
mod hello;

use super::*;
use crate::Result;
use std::cell::RefCell;
use std::net::Ipv4Addr;
use ws::{connect, CloseCode};


fn send_message( message: &ChatMessage) -> ChatMessage {
    let msg: RefCell<Option<ChatMessage>> = RefCell::new(None);
    let msg_ref = &msg;

    connect("ws://127.0.0.1:4444", |out| {
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

