mod app;
mod message;

use app::App;
use message::Message;
use std::time::Duration;
use iced::{Size, window};


fn main() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
        .title(|_app: &App| "收音机".to_string())
        .window(window::Settings {
            size: Size::new(760.0, 540.0),
            resizable: false,
            ..Default::default()
        })
        .theme(|_app: &App| iced::Theme::Dark)
        .subscription(subscriptions)
        .run()
}

fn subscriptions(_app: &App) -> iced::Subscription<Message> {
    iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
}
