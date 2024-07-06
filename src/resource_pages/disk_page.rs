use iced::{
    alignment,
    widget::{column, container, row, scrollable, text, Container},
    Command, Element, Length,
};
use iced_aw::BootstrapIcon;
use sysinfo::DiskKind;

use crate::{
    constants::padding,
    general_widgets::{
        icons::bootstrap_icon, section::section_box, seperators::seperator_background_1,
        split_table_double::split_table_double, split_table_single::split_table_single,
    },
    preferences::Preferences,
    styles::{self, container::resource_details_header},
    utils::{format_bytes, round_bytes_list},
    DiskData,
};

use super::{
    chart::{ResourceChart, ResourceChartMessage},
    resource_details::ResourceDetailsMessage,
};

#[derive(Debug, Clone)]
pub enum DiskPageMessage {
    ResourceChartMessage(ResourceChartMessage),
}

#[derive(Debug)]
pub struct DiskPage {
    written_chart: ResourceChart,
    read_chart: ResourceChart,
}

impl DiskPage {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            written_chart: ResourceChart::new(preferences),
            read_chart: ResourceChart::new(preferences),
        }
    }

    pub fn update(&mut self, message: DiskPageMessage) -> Command<DiskPageMessage> {
        match message {
            _ => Command::none(),
        }
    }

    // For some reason it won't let me use ResourceDiskMessage
    pub fn view(
        &self,
        preferences: &Preferences,
        data: &DiskData,
    ) -> Element<DiskPageMessage> {
        let header = container(row![text(data.name.clone())])
            .center_x()
            .style(resource_details_header())
            .width(Length::Fill)
            .padding(padding::MAIN);

        let read_ui = section_box(
            (
                bootstrap_icon(BootstrapIcon::Eye),
                text(String::from("Read")),
                row![],
            ),
            {
                column![
                    container(
                        self.read_chart
                            .view(None)
                            .map(move |message| DiskPageMessage::ResourceChartMessage(message))
                    ),
                    seperator_background_1(),
                    split_table_single(vec![(
                        text("Reads".to_string()),
                        text(format_bytes(preferences, data.read as f32))
                    )]),
                ]
            },
        );

        let write_ui = section_box(
            (
                bootstrap_icon(BootstrapIcon::Pen),
                text(String::from("Written")),
                row![],
            ),
            {
                column![
                    container(
                        self.written_chart
                            .view(None)
                            .map(move |message| DiskPageMessage::ResourceChartMessage(message))
                    ),
                    seperator_background_1(),
                    split_table_single(vec![(
                        text("Writes".to_string()),
                        text(format_bytes(preferences, data.written as f32))
                    )]),
                ]
            },
        );

        let thermals = section_box(
            (
                bootstrap_icon(BootstrapIcon::Thermometer),
                text(String::from("Thermals")),
                row![],
            ),
            split_table_single(vec![(
                text(String::from("Temperature")),
                text(String::from("25℃")), /* format!("{:.2}°C") */
            )]),
        );

        let about = section_box(
            (
                bootstrap_icon(BootstrapIcon::InfoCircle),
                text(String::from("About")),
                row![],
            ),
            column![
                split_table_double(vec![
                    (
                        (
                            text("Usage".to_string()),
                            text({
                                let result = round_bytes_list(
                                    preferences,
                                    vec![data.space_used as f32, data.space_total as f32],
                                );

                                format!("{} / {} {}", result.0[0], result.0[1], result.1)
                            })
                        ),
                        (
                            text("Percent used".to_string()),
                            text(format!(
                                "{:.1}%",
                                data.space_used as f64 / data.space_total as f64 * 100.
                            ))
                        )
                    ),
                    (
                        (
                            text("Free space".to_string()),
                            text(format_bytes(
                                preferences,
                                data.space_total as f32 - data.space_used as f32
                            ))
                        ),
                        (
                            text("Percent remaining".to_string()),
                            text(format!(
                                "{:.1}%",
                                (1. - data.space_used as f64 / data.space_total as f64)
                                    * 100.
                            ))
                        )
                    )
                ]),
                seperator_background_1(),
                split_table_single(vec![
                    (text(String::from("Brand")), text(String::from("Unknown"))),
                    (
                        text(String::from("Kind")),
                        text(format!("{}", data.kind))
                    ),
                    (
                        text(String::from("Removable")),
                        text(format!(
                            "{}",
                            data.in_depth.as_ref().unwrap().is_removable
                        ))
                    ),
                ])
            ],
        );

        let main = container(
            column![read_ui, write_ui, thermals, about]
                .spacing(20)
                .align_items(alignment::Alignment::Center),
        )
        .center_x()
        .width(Length::Fill)
        .padding(padding::SECTION);

        let content = column![
            header,
            scrollable(main).style(iced::theme::Scrollable::Custom(Box::new(
                styles::scrollable::Background1 {},
            )))
        ];

        let container = container(content);
        container.into()
    }
}
