use iced::{
    widget::{column, container, row, text, Column, Row, Text},
    Element, Length,
};

use crate::{
    constants::{font_sizes, padding, sizings},
    styles::container::resource_details_child,
};

pub fn section_box<'a, Message: 'a>(
    header: (Text<'a>, Text<'a>),
    body: Column<'a, Message>,
) -> Column<'a, Message> {

    let header =
        row![header.0.size(font_sizes::H2), header.1.size(font_sizes::H2)].spacing(padding::MAIN);

    let body_content = container(body)
        .style(resource_details_child())
        .width(Length::Fill)
        .center_y();

    let content = column![header, body_content]
        .spacing(padding::PORTION)
        .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH);

    content
}
