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
pub enum DiskDetailsMessage {
    Mount(String),
    ResourceChartMessage(ResourceChartMessage),
}

#[derive(Debug)]
pub struct DiskDetails {
    read: u64,
    written: u64,
    space_total: u64,
    space_used: u64,
    kind: DiskKind,
    name: String,
    written_chart: ResourceChart,
    read_chart: ResourceChart,
    is_removable: bool,
}

impl DiskDetails {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            read: 0,
            written: 0,
            space_total: 0,
            space_used: 0,
            kind: DiskKind::Unknown(0),
            name: String::new(),
            is_removable: false,
            written_chart: ResourceChart::new(&preferences),
            read_chart: ResourceChart::new(&preferences),
        }
    }

    pub fn on_tick(&mut self, disk_data: &DiskData) {
        self.read = disk_data.read;
        self.written = disk_data.written;
        self.space_total = disk_data.space_total;
        self.space_used = disk_data.space_used;
        self.kind = disk_data.kind;
        self.name = disk_data.name.clone();
        self.is_removable = disk_data.in_depth.is_removable;

        // requires history

        // self.written_chart;
        // self.read_chart;
    }

    pub fn update(&mut self, message: DiskDetailsMessage) -> Command<DiskDetailsMessage> {
        match message {
            _ => Command::none(),
        }
    }

    // For some reason it won't let me use ResourceDiskMessage
    pub fn view(&self, preferences: &Preferences) -> Element<DiskDetailsMessage> {
        let header = container(row![text(self.name.clone())])
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
                            .map(move |message| DiskDetailsMessage::ResourceChartMessage(message))
                    ),
                    seperator_background_1(),
                    split_table_single(vec![(
                        text("Reads".to_string()),
                        text(format_bytes(preferences, self.read as f32))
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
                            .map(move |message| DiskDetailsMessage::ResourceChartMessage(message))
                    ),
                    seperator_background_1(),
                    split_table_single(vec![(
                        text("Writes".to_string()),
                        text(format_bytes(preferences, self.written as f32))
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
                                let result = round_bytes_list(preferences, vec![self.space_used as f32, self.space_total as f32]);

                                format!("{} / {} {}", result.0[0], result.0[1], result.1)
                            })

                        ),
                        (
                            text("Percent used".to_string()),
                            text(format!(
                                "{:.1}%",
                                self.space_used as f64 / self.space_total as f64 * 100.
                            ))
                        )
                    ),
                    (
                        (
                            text("Free space".to_string()),
                            text(format_bytes(
                                preferences,
                                self.space_total as f32 - self.space_used as f32
                            ))
                        ),
                        (
                            text("Percent remaining".to_string()),
                            text(format!(
                                "{:.1}%",
                                (1. - self.space_used as f64 / self.space_total as f64) * 100.
                            ))
                        )
                    )
                ]),
                seperator_background_1(),
                split_table_single(vec![
                    (text(String::from("Brand")), text(String::from("Unknown"))),
                    (text(String::from("Kind")), text(format!("{}", self.kind))),
                    (
                        text(String::from("Removable")),
                        text(format!("{}", self.is_removable))
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
