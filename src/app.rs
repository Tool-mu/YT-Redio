// 应用状态

use crate::message::{Band, Message};
use iced::widget::{button, column, row, text};
use iced::{Element, Task};

pub const FAKE_STATIONS: [&str; 5] = ["中国之声", "2", "3", "4", "5"];

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

    pub fn step_index(current: usize, dir: i32, count: usize) -> usize {
        if count == 0 {
            current
        } else {
            (current as i32 + dir).rem_euclid(count as i32) as usize
        }
        // todo!()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => self.ticks += 1,
            Message::BandChange(band) => self.band = band,
            Message::PowerToggle => self.power_on = !self.power_on,
            Message::FlipToggle => self.flip.open = !self.flip.open,
            Message::StationStep(dir) => {
                self.current = Self::step_index(self.current, dir, self.stations.len())
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let current = self
            .stations
            .get(self.current)
            .map(String::as_str)
            .unwrap_or("-");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_wraps_around() {
        assert_eq!(App::step_index(0, -1, 5), 4);
        assert_eq!(App::step_index(4, 1, 5), 0);
        assert_eq!(App::step_index(2, 1, 5), 3);
    }

    #[test]
    fn step_with_no_stations_is_noop() {
        assert_eq!(App::step_index(0, 1, 0), 0);
    }

    #[test]
    fn power_toggle_flips_twice() {
        let (mut app, _) = App::boot();
        assert!(app.power_on);

        let _ = app.update(Message::PowerToggle);
        assert!(!app.power_on);

        let _ = app.update(Message::PowerToggle);
        assert!(app.power_on);
    }

    #[test]
    fn tick_counts_up() {
        let (mut app, _) = App::boot();

        let _ = app.update(Message::Tick);
        let _ = app.update(Message::Tick);
        let _ = app.update(Message::Tick);

        assert!(app.ticks == 3)
    }
}
