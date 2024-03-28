use iced::{widget, Background, Border};

use crate::constants::custom_theme::{self, BACKGROUND_1, BACKGROUND_2};

pub fn background_1() -> widget::scrollable::Appearance {
    widget::scrollable::Appearance {
        container: widget::container::Appearance {
            ..Default::default()
        },
        scrollbar: widget::scrollable::Scrollbar {
            border: Border {
                radius: [12., 12., 12., 12.].into(),
                ..Default::default()
            },
            background: Some(Background::from(BACKGROUND_1)),
            scroller: widget::scrollable::Scroller {
                color: custom_theme::BACKGROUND_3,
                border: Border {
                    radius: [12., 12., 12., 12.].into(),
                    ..Default::default()
                },
            },
        },
        gap: Some(Background::from(BACKGROUND_1)),
    }
    // widget::scrollbar::Appearance {
    //     background: Background::from(BACKGROUND_5),
    //     bar: Background::from(custom_theme::PRIMARY),
    //     border_radius: [12., 12., 12., 12.].into(),
    // }
}
