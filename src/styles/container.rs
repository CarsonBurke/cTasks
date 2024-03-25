use iced::{widget::{self, container}, Background, Theme};

pub fn main_content(theme: &Theme, status: container::Status) -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(Theme::palette(theme).primary)),
        ..Default::default()
    }
}

pub fn sidebar(theme: &Theme, status: container::Status) -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}
