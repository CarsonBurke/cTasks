use iced::{
    widget::{column, container, row, text, Column},
    Element, Length,
};

use crate::{
    constants::{font_sizes, padding, sizings},
    styles::container::resource_details_child,
};

/// (Icon, Header text), Column<body elements>
pub fn section_box<'a, Message: 'a>(
    header: (iced_aw::BootstrapIcon, String),
    body: Column<'a, Message>,
) -> Column<'a, Message> {
    let header = row![
        text(header.0)
            .font(iced_aw::BOOTSTRAP_FONT)
            .size(font_sizes::H2),
        text(String::from(header.1)).size(font_sizes::H2),
    ];

    let body_content = container(body)
        .style(resource_details_child())
        .width(Length::Fill)
        .center_y();

    let content = column![header, body_content]
        .spacing(padding::PORTION)
        .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH);

    content
}
