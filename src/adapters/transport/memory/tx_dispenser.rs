use std::sync::{mpsc::Sender, Mutex};

pub struct TxDispenser<T>(Mutex<Sender<T>>);

impl<T> TxDispenser<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self(Mutex::new(sender))
    }

    pub fn get(&self) -> Sender<T> {
        (*self.0.lock().expect("Internal Error: `Sender` instance unexpectedly not available."))
            .clone()
    }
}
