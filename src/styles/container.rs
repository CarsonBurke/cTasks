use iced::{widget, Background, Theme};

pub fn primary(theme: &Theme) -> widget::container::Appearance {
    widget::container::Appearance {
        text_color: Some(iced::Color::from_rgb(132.0, 123.0, 124.0)),
        background: Some(Background::from(Theme::palette(theme).primary)),
        ..Default::default()
    }
}

pub fn secondary(theme: &Theme) -> widget::container::Appearance {
    widget::container::Appearance {
        text_color: Some(iced::Color::from_rgb(132.0, 123.0, 124.0)),
        background: Some(Background::from(Theme::palette(theme).primary)),
        ..Default::default()
    }
}

pub fn main_content(theme: &Theme) -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(Theme::palette(theme).primary)),
        ..Default::default()
    }
}

pub fn sidebar(theme: &Theme) -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}
