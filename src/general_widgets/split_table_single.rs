// text on left or right. Header above content.

// read-only
// text on left and right. headers above descriptors

// struct SplitTableDouble {

// }

// impl SplitTableDouble {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

// pub fn split_table_double() -> SplitTableDouble {
// SplitTableDouble::new()
// }

use iced::{
    theme,
    widget::{column, container, horizontal_space, row, text, Column, Text},
    Element, Length,
};

use crate::{
    constants::{custom_theme, padding},
    styles::container::divider_background_1,
};

pub fn split_table_single<'a, Message: 'a>(
    params: Vec<(Text<'a>, Text<'a>)>,
) -> Column<'a, Message> {
    let content = Column::with_children({
        let mut children: Vec<Element<'a, Message>> = vec![];

        let last_index = params.len() - 1;
        let mut i = 0;

        for (header, descriptor) in params {
            let seperator = if i != last_index {
                container(row![])
                    .style(divider_background_1())
                    .width(Length::Fill)
                    .height(1)
            } else {
                container(row![])
            };

            children.push(
                column![
                    row![column![
                        header.style(theme::Text::Color(custom_theme::GREY_TEXT)),
                        descriptor,
                    ]]
                    .padding(padding::MAIN),
                    seperator,
                ]
                .into(),
            );

            i += 1;
        }

        children
    });

    content
}
