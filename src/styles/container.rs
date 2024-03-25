use iced::{widget, Background, Theme};

use crate::constants::custom_theme::{BACKGROUND_1, BACKGROUND_2};

// pub fn main_content(theme: &Theme) -> widget::container::Appearance {
//     widget::container::Appearance {
//         background: Some(Background::from(Theme::palette(theme).background)),
//         ..Default::default()
//     }
// }

// pub fn sidebar(theme: &Theme) -> widget::container::Appearance {
//     widget::container::Appearance {
//         background: Some(Background::from(Theme::palette(theme).background)),
//         ..Default::default()
//     }
// }

pub fn main_content() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_1)),
        ..Default::default()
    }
}

pub fn sidebar() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_2)),
        ..Default::default()
    }
}
