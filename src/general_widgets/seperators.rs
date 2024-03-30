use iced::{widget::{container, row, Container}, Length};

use crate::styles::container::divider_background_1;

pub fn seperator_background_1<'a, Message: 'a>() -> Container<'a, Message> {
    container(row![])
    .style(divider_background_1())
    .width(Length::Fill)
    .height(1)
}