// UI 模块
pub mod texture;

use crate::app::App;
use crate::message::Message;
use iced::widget::{image, stack};
use iced::Element;
use texture::{BODY_H, BODY_W};

pub fn view(app: &App) -> Element<'_, Message> {
    let tex = texture::get();

    stack![
        image(&tex.body).width(BODY_W).height(BODY_H),
        app.view(),
    ]
    .into()
}