use iced::{
    event::Status, theme::{self}, widget::{self, button, container}, Background, Color, Theme
};

pub fn button_appearance(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}