use iced::{widget::{self, container}, Background, Theme};

pub fn main_content(theme: &Theme) -> widget::container::Style {
    widget::container::Style {
        background: Some(Background::from(Theme::palette(theme).primary)),
        ..Default::default()
    }
}

pub fn sidebar(theme: &Theme) -> widget::container::Style {
    widget::container::Style {
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}
