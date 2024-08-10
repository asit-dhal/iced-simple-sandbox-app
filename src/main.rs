use iced::widget::{
    column, button, container, scrollable, row, text,
};
use iced::{Alignment, Element, Length, Sandbox, Settings};

struct AppState {
    index: i32,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementButtonPressed,
    DecrementButtonPressed,
}

impl Sandbox for AppState {
    type Message = Message;

    fn new() -> AppState {
        println!("{:?} Sandbox::new()", std::thread::current().id());
        Self {
            index: 0,
        }
    }

    fn title(&self) -> String {
        println!("{:?} Sandbox::title()", std::thread::current().id());
        String::from("AppState")
    }

    fn update(&mut self, message: Self::Message) {
        println!("{:?} Sandbox::update({:?})", std::thread::current().id(), message);
        self.index = match message {
            Message::IncrementButtonPressed => {self.index + 1},
            Message::DecrementButtonPressed => {self.index - 1},
        };
    }

    fn view(&self) -> Element<Self::Message> {
        println!("{:?} Sandbox::view()", std::thread::current().id());
        let incerement_btn = button(
            "Increment"
        )
        .width(150)
        .on_press(Message::IncrementButtonPressed);

        let label_text = text("Value:").width(75);
        let index_text = text(self.index.to_string()).width(75);

        let decrement_btn = button(
            "Decrement"
        )
        .width(150)
        .on_press(Message::DecrementButtonPressed);

        let content = column![
            incerement_btn,
            row![label_text, index_text,],
            decrement_btn,
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> iced::Theme {
        println!("{:?} Sandbox::theme()", std::thread::current().id());
        iced::Theme::Dark
    }
}
fn main() -> iced::Result {
    AppState::run(Settings::default())
}