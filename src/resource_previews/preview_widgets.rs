use iced::{
    theme,
    widget::{row, Row, Text},
    Element,
};

use crate::constants::{custom_theme, font_sizes, padding};

pub fn preview_header<'a, Message: 'a>(icon: Text<'a>, text: Text<'a>) -> Row<'a, Message> {
    let row = row![icon.size(font_sizes::H2), text.size(font_sizes::H2),].spacing(padding::PORTION);

    row
}

pub fn preview_metrics<'a, Message: 'a>(metrics: Vec<(Text<'a>, Text<'a>)>) -> Row<'a, Message> {
    let row = Row::with_children({
        let mut children: Vec<Element<'a, Message>> = Vec::new();

        for metric in metrics {
            children.push(
                row![
                    metric
                        .0
                        .style(theme::Text::Color(custom_theme::GREY_TEXT))
                        .size(font_sizes::P),
                    metric
                        .1
                        .style(theme::Text::Color(custom_theme::GREY_TEXT))
                        .size(font_sizes::P),
                ]
                .spacing(padding::PORTION)
                .into(),
            )
        }

        children
    })
    .spacing(padding::MAIN);

    row
}
