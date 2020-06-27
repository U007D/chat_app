use std::sync::{mpsc::Sender, Mutex, MutexGuard};

// Imbues `Mutex<Sender<T>>` with `Sync`.
pub struct TxSharer<T>(Mutex<Sender<T>>);

impl<T> TxSharer<T> {
    pub fn new(tx: Sender<T>) -> Self {
        Self(Mutex::new(tx))
    }

    pub fn lock(&self) -> MutexGuard<'_, Sender<T>> {
        self.0
            .lock()
            .expect("Internal Error: Unexpectedly could not acquire (lock) `Sender` instance.")
    }
}
