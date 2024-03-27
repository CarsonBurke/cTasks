use iced::widget;

use crate::constants::custom_theme;

pub fn grey() -> widget::text::Appearance {
    widget::text::Appearance {
        color: Some(custom_theme::GREY_TEXT),
        ..Default::default()
    }
}