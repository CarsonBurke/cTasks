use iced::{
    application::{Appearance, StyleSheet},
    theme::{self, Button},
    widget::{self, button, container},
    Background, Color, Theme,
};

pub struct MyButtonStyleSheet;

impl StyleSheet for MyButtonStyleSheet {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background_color: Color::from_rgb(0.1, 0.2, 0.3),
            text_color: Color::from_rgb(0.9, 0.9, 0.9),
        }
    }
}

pub fn button_appearance(theme: &Theme) -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}

pub fn primary_blended(theme: &Theme) -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}

pub fn secondary_blended(theme: &Theme) -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}