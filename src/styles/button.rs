use iced::{
    application::{Appearance, StyleSheet},
    theme::{self, Button},
    widget::{self, button::{self}, container},
    Background, Color, Theme,
};

use crate::constants::custom_theme;

pub fn primary() -> widget::button::Appearance {

    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(custom_theme::PRIMARY)),
        ..Default::default()
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

pub struct Primary;

impl button::StyleSheet for Primary {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(
                50.0 / 255.0,
                217.0 / 255.0,
                147.0 / 255.0,
                1.0,
            ))),
            text_color: Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                width: 1.0,
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.3, 0.3, 0.3, 0.3,
            ))),
            text_color: Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 0.0.into(),
                width: 1.0,
                color: Color::from_rgb(1.0, 0.0, 0.0),
            },
            ..Default::default()
        }
    }
}