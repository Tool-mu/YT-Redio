mod app;
mod message;
mod ui;

use app::App;
use iced::{Size, window};
use message::Message;
use std::time::Duration;

fn main() -> iced::Result {
    iced::application(App::boot, App::update, ui::view)
        .title(|_app: &App| "收音机".to_string())
        .window(window::Settings {
            size: Size::new(ui::texture::BODY_W, ui::texture::BODY_H),
            resizable: false,
            icon: ui::texture::window_icon(),
            ..Default::default()
        })
        .theme(|_app: &App| iced::Theme::Dark)
        .subscription(subscriptions)
        .run()
}

fn subscriptions(_app: &App) -> iced::Subscription<Message> {
    iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
}
