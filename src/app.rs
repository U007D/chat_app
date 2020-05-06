use crate::ports::Transport;

#[cfg(test)]
pub(crate) mod unit_tests;

struct App {}

impl App {
    pub fn new<T: Transport>(transport: T){}
}