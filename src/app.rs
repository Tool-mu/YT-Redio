// 应用状态

use crate::message::{Band, Message};
use iced::widget::{button, column, row, text};
use iced::{Element, Task};

pub const FAKE_STATIONS: [&str; 5] = [
    "中国之声", "2", "3", "4", "5"
];

#[derive(Debug, Clone, Default)]
pub struct FlipState {
    pub open: bool,
}

pub struct App {
    pub band: Band,
    pub stations: Vec<String>,
    pub current: usize,
    pub power_on: bool,
    pub flip: FlipState,
    pub ticks: u64,
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        let app = Self {
            band: Band::National,
            stations: FAKE_STATIONS.iter().map(|s| s.to_string()).collect(),
            current: 0,
            power_on: true,
            flip: FlipState::default(),
            ticks: 0,
        };

        (app, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => self.ticks += 1,
            Message::BandChange(band) => self.band = band,
            Message::PowerToggle => self.power_on = !self.power_on,
            Message::FlipToggle => self.flip.open = !self.flip.open,
            Message::StationStep(dir) => {
                let count = self.stations.len();
                if count > 0 {
                    self.current = (self.current as i32 + dir).rem_euclid(count as i32) as usize;
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let current = self.stations.get(self.current).map(String::as_str).unwrap_or("-");

        column![
            text(format!("· {current}")),
            text(format!(
                "波段 {:?} · 电源 {} · 翻盖 {} · 已运行 {} 秒",
                self.band, self.power_on, self.flip.open, self.ticks
            )),
            row![
                button(text("◀")).on_press(Message::StationStep(-1)),
                button(text("▶")).on_press(Message::StationStep(1)),
            ]
            .spacing(8),
            row![
                button(text("国家台")).on_press(Message::BandChange(Band::National)),
                button(text("省市台")).on_press(Message::BandChange(Band::Province)),
                button(text("收藏")).on_press(Message::BandChange(Band::Favorites)),
            ]
            .spacing(8),
            row![
                button(text("电源")).on_press(Message::PowerToggle),
                button(text("翻盖")).on_press(Message::FlipToggle),
            ]
            .spacing(8),
        ]
        .spacing(12)
        .padding(20)
        .into()
    }
}
