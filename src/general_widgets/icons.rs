use iced::{widget::{text, Text}, Renderer, Theme};
use iced_aw::BootstrapIcon;

// pub fn bootstrap_icon<'a, Message: 'a>(icon_name: BootstrapIcon) -> iced::advanced::widget::Text<'_, _, _>/* -> Text<'a, Message> */ {
//     text(icon_name).font(iced_aw::BOOTSTRAP_FONT)
// }

pub fn bootstrap_icon<'a>(icon_name: BootstrapIcon) -> Text<'a>/* -> Text<'a, Message> */ {
    text(icon_name).font(iced_aw::BOOTSTRAP_FONT)
}