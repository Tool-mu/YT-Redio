use iced::widget::{column, text};
use iced::{window, Element, Size, Task};

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct App;

impl App {
    fn boot() -> (Self, Task<Message>) {
        (App, Task::none())
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text("收音机"),
            text("123")
        ]
        .spacing(8)
        .padding(20)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
        .title(|_app: &App| "收音机".to_string())
        .window(window::Settings {
            size: Size::new(760.0, 540.0),
            resizable: false,
            ..Default::default()
        })
        .theme(|_app: &App| iced::Theme::Dark)
        .run()
}
