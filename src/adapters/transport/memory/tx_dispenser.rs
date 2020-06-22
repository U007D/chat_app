use std::sync::{mpsc::Sender, Mutex};

pub struct TxDispenser<T>(Mutex<Sender<T>>);

impl<T> TxDispenser<T> {
    pub fn new(tx: Sender<T>) -> Self {
        Self(Mutex::new(tx))
    }

    pub fn get(&self) -> Sender<T> {
        (*self.0.lock().expect("Internal Error: `Sender` instance unexpectedly not available."))
            .clone()
    }
}