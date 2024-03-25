use iced::{
    event::Status, theme::{self}, widget::{self, button, container}, Background, Color, Theme
};

pub fn button_appearance(theme: &Theme, status: button::Status) -> button::Appearance {
    button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}

pub fn primary_blended(theme: &Theme, status: button::Status) -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}

pub fn secondary_blended(theme: &Theme, status: button::Status) -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}