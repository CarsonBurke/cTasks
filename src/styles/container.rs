use iced::{border::Radius, widget, Background, Border, Theme};

use crate::constants::custom_theme::{BACKGROUND_1, BACKGROUND_2, BACKGROUND_3};

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

pub fn resource_details_header() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_2)),
        ..Default::default()
    }
}

pub fn resource_details_child() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_2)),
        border: Border {
            radius: [12., 12., 12., 12.].into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn primary_process_grid_row() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_1)),
        border: Border {
            radius: [12., 12., 12., 12.].into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn alternate_process_grid_row() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_2)),
        border: Border {
            radius: [12., 12., 12., 12.].into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn divider_background_1() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_1)),
        ..Default::default()
    }
}