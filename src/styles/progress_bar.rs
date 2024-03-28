use iced::{widget, Background, Theme};

use crate::constants::custom_theme::{self, BACKGROUND_3, BACKGROUND_4, BACKGROUND_5};

pub fn primary_background_5() -> widget::progress_bar::Appearance {
    widget::progress_bar::Appearance {
        background: Background::from(BACKGROUND_5),
        bar: Background::from(custom_theme::PRIMARY),
        border_radius: [12., 12., 12., 12.].into(),
    }
}
