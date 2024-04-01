use iced::{widget::{self, scrollable}, Background, Border};

use crate::constants::custom_theme::{self, BACKGROUND_1, BACKGROUND_2};

pub struct Background1;

impl scrollable::StyleSheet for Background1 {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> scrollable::Appearance {
        scrollable::Appearance {
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
                        width: 1.5,
                        ..Default::default()
                    },
                },
            },
            gap: Some(Background::from(BACKGROUND_1)),
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> scrollable::Appearance {

        // If the mouse is hovering over the scrollable content but not the scrollbar itself, treat is as normal
        if !is_mouse_over_scrollbar {
            return self.active(style);
        }

        scrollable::Appearance {
            container: widget::container::Appearance {
                ..Default::default()
            },
            scrollbar: widget::scrollable::Scrollbar {
                border: Border {
                    radius: [12., 12., 12., 12.].into(),
                    ..Default::default()
                },
                background: Some(Background::from(BACKGROUND_2)),
                scroller: widget::scrollable::Scroller {
                    color: custom_theme::BACKGROUND_4,
                    border: Border {
                        radius: [12., 12., 12., 12.].into(),
                        ..Default::default()
                    },
                },
            },
            gap: Some(Background::from(BACKGROUND_1)),
        }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Appearance {
        scrollable::Appearance {
            container: widget::container::Appearance {
                ..Default::default()
            },
            scrollbar: widget::scrollable::Scrollbar {
                border: Border {
                    radius: [12., 12., 12., 12.].into(),
                    ..Default::default()
                },
                background: Some(Background::from(BACKGROUND_2)),
                scroller: widget::scrollable::Scroller {
                    color: custom_theme::BACKGROUND_5,
                    border: Border {
                        radius: [12., 12., 12., 12.].into(),
                        ..Default::default()
                    },
                },
            },
            gap: Some(Background::from(BACKGROUND_1)),
        }
    }
}
