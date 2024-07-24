use iced::{
    alignment, theme, widget::{button, column, container, horizontal_space, row, scrollable, text, text_input}, Alignment, Command, Element, Length, Theme
};
use iced_aw::{style, BootstrapIcon};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use crate::{
    constants::{font_sizes, padding, sizings},
    general_widgets::{
        icons::bootstrap_icon, section::section_box, seperators::seperator_background_1,
        split_table_double::split_table_double, split_table_single::split_table_single,
    },
    preferences::Preferences,
    styles::{self, container::{divider_background_1, resource_details_child, resource_details_header}},
    MemoryData, ResourceHistory,
};

use super::chart::{ResourceChart, ResourceChartMessage};

#[derive(Debug, Clone)]
pub enum MemoryPageMessage {
    ResourceChartMessage(ResourceChartMessage),
    ChangeSwapiness(u32),
}

#[derive(Debug)]
pub struct MemoryPage {
    pub ram_chart: ResourceChart,
    pub swap_chart: ResourceChart,
}

impl MemoryPage {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            ram_chart: ResourceChart::new(preferences),
            swap_chart: ResourceChart::new(preferences),
        }
    }

    pub fn update_history(&mut self, resource_history: &ResourceHistory) {
        self.ram_chart.data_points = resource_history.ram.clone();
        self.swap_chart.data_points = resource_history.swap.clone();
    }

    pub fn update(&mut self, message: MemoryPageMessage) -> Command<MemoryPageMessage> {
        match message {
            _ => Command::none(),
        }
    }

    pub fn view(&self, preferences: &Preferences, data: &MemoryData) -> Element<MemoryPageMessage> {
        let header = container(row!["Memory"])
            .center_x()
            .style(resource_details_header())
            .width(Length::Fill)
            .padding(padding::MAIN);

        let ram_details =
            section_box(
                (
                    bootstrap_icon(BootstrapIcon::Memory),
                    text(String::from("Random Access Memory")),
                    row![],
                ),
                {
                    if data.ram_usage == 0 || data.ram_total == 0 {
                        column!["No RAM data to display"]
                    } else {
                        column![
                            container(self.ram_chart.view(None).map(move |message| {
                                MemoryPageMessage::ResourceChartMessage(message)
                            })),
                            seperator_background_1(),
                            split_table_double(vec![(
                                (
                                    text("Usage".to_string()),
                                    text(format!(
                                        "{:.2} / {:.2} GB",
                                        data.ram_usage as f64 / 1024. / 1024. / 1024.,
                                        data.ram_total as f64 / 1024. / 1024. / 1024.
                                    ))
                                ),
                                (
                                    text("Percent used".to_string()),
                                    text(format!(
                                        "{:.1}%",
                                        data.ram_usage as f64 / data.ram_total as f64 * 100.
                                    ))
                                )
                            )]),
                        ]
                    }
                },
            );

        let swap_details = section_box(
            (
                bootstrap_icon(BootstrapIcon::HddRack),
                text(String::from("Swap")),
                row![],
            ),
            {
                if data.swap_usage == 0 || data.swap_total == 0 {
                    column!["No Swap data to display"]
                } else {
                    let swap_percent = data.swap_usage * 100 / data.swap_total;

                    column![
                        container(self.swap_chart.view(None).map(move |message| {
                            MemoryPageMessage::ResourceChartMessage(message)
                        })),
                        seperator_background_1(),
                        split_table_double(vec![(
                            (
                                text("Usage".to_string()),
                                text(format!(
                                    "{:.2} / {:.2} GB",
                                    data.swap_usage as f64 / 1024. / 1024. / 1024.,
                                    data.swap_total as f64 / 1024. / 1024. / 1024.
                                ))
                            ),
                            (
                                text("Percent used".to_string()),
                                text(format!(
                                    "{:.1}%",
                                    data.swap_usage as f64 / data.swap_total as f64 * 100.
                                ))
                            )
                        )])
                    ]
                }
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
            column![split_table_single(vec![
                (text(String::from("Speed")), text(String::from("25℃"))),
                (text(String::from("Slots used")), text(String::from("25℃"))),
                (
                    text(String::from("RAM type")),
                    text(String::from("SODIMM?"))
                ),
                (text(String::from("Swapiness")), text(String::from("N/A"))),
            ])],
        );

        // modify swapiness
        // other?
        let advanced = column![
            row![
                text(iced_aw::graphics::icons::BootstrapIcon::Tools.to_string())
                    .font(iced_aw::BOOTSTRAP_FONT)
                    .size(font_sizes::H1),
                text(String::from("Advanced")).size(font_sizes::H1) // i in the top right that takes someone to a description of what Swap is
            ]
            .spacing(padding::MAIN),
            container(column![
                row![
                    text(String::from("Swapiness")),
                    horizontal_space(),
                    row![
                        text_input("current swapiness", "val"),
                        button("change")
                    ]
                    .spacing(padding::PORTION),
                ]
                .padding(padding::MAIN),
                container(row![])
                    .style(divider_background_1())
                    .width(Length::Fill)
                    .height(1),
            ])
            .style(resource_details_child())
            .width(Length::Fill)
            .center_y()
        ]
        .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH)
        .spacing(padding::PORTION);

        let main = container(
            column![ram_details, swap_details, thermals, about, advanced]
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
