use iced::{Application, Column, Command, Element, Text};

#[derive(Default)]
pub struct ChatWindow {}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Application for ChatWindow {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Chat App")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .push(Text::new("Welcome to the Chat App").size(50))
            .into()
    }
}
