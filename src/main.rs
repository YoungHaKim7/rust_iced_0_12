use iced::{
    widget::{Button, Column, Text},
    Alignment, Element, Sandbox, Settings, Size,
};

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

impl Sandbox for Counter {
    // 1. fn
    // 2 .associated type
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Cool Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new(self.value.to_string()).size(50))
            .push(Button::new(Text::new("+")).on_press(Message::Increment))
            .into()
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings {
        window: iced::window::Settings {
            size: Size {
                width: 300.,
                height: 200.,
            },
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}
