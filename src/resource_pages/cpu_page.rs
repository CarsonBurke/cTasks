use iced::{
    alignment,
    widget::{checkbox, column, container, row, scrollable, text},
    Command, Element, Length,
};
use iced_aw::{BootstrapIcon, Wrap};

use crate::{
    constants::{
        padding,
        sizings::{self, DEFAULT_CHART_HEIGHT},
    }, general_widgets::{
        icons::bootstrap_icon,
        section::{section, section_box, section_box_headless},
        seperators::seperator_background_1,
        split_table_double::split_table_double,
        split_table_single::split_table_single,
    }, preferences::Preferences, styles::{self, container::resource_details_header}, types::resource_data::CpuData, utils::format_hz, ResourceHistory
};

use super::{
    chart::{ResourceChart, ResourceChartMessage},
};

#[derive(Debug, Clone)]
pub enum CpuPageMessage {
    ResourceChartMessage(ResourceChartMessage),
    ToggleLogicalCores(bool),
}

#[derive(Debug)]
pub struct CpuPage {
    pub cpu_chart: ResourceChart,
    pub logical_core_charts: Vec<ResourceChart>,
    pub show_logical_cores: bool,
}

impl CpuPage {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            cpu_chart: ResourceChart::new(preferences),
            logical_core_charts: Vec::new(),
            show_logical_cores: preferences.show_logical_core_charts,
        }
    }

    pub fn update(&mut self, message: CpuPageMessage) -> Command<CpuPageMessage> {
        match message {
            CpuPageMessage::ToggleLogicalCores(new_state) => {
                self.show_logical_cores = new_state;

                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn update_history(&mut self, resource_history: &ResourceHistory) {
        self.cpu_chart.data_points = resource_history.cpu.clone();

        for (i, chart) in &mut self.logical_core_charts.iter_mut().enumerate() {
            chart.data_points = resource_history.logical_cores[i].clone();
        }
    }

    pub fn view(&self, preferences: &Preferences, data: &CpuData, physical_core_count: u32, logical_core_count: u32, brand: String) -> Element<CpuPageMessage> {

        let header = container(row!["CPU"])
            .center_x()
            .style(resource_details_header())
            .width(Length::Fill)
            .padding(padding::MAIN);

        let cpu_details_ui = {
            if preferences.show_logical_core_charts {
                section(
                    (
                        bootstrap_icon(BootstrapIcon::Cpu),
                        text(String::from("CPU")),
                        row![
                            checkbox("logical cores", preferences.show_logical_core_charts)
                                .on_toggle(CpuPageMessage::ToggleLogicalCores)
                        ],
                    ),
                    column![Wrap::with_elements({
                        let mut children: Vec<Element<'_, CpuPageMessage>> = Vec::new();

                        for (i, usage_percent) in data.logical_cores_usage_percents.iter().enumerate() {
                            children.push(
                                section_box_headless(column![
                                    self.logical_core_charts[i]
                                        .view(Some(Length::Fixed(DEFAULT_CHART_HEIGHT / 2.)))
                                        .map(move |message| {
                                            CpuPageMessage::ResourceChartMessage(message)
                                        }),
                                    seperator_background_1(),
                                    split_table_double(vec![(
                                        (
                                            text(String::from("Percent used")),
                                            text(format!("{:.1}%", usage_percent)),
                                        ),
                                        (
                                            text(String::from("Frequency")),
                                            text(format_hz(
                                                preferences,
                                                data.logical_cores_frequencies[i] as f32
                                            ))
                                        ),
                                    )]),
                                ])
                                .max_width(
                                    sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH as f32 / 3.
                                        - padding::MAIN as f32 * 3.,
                                )
                                //.max_width((sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH as f32 - padding::MAIN as f32) / 2./* sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH as f32 / 2. - padding::MAIN as f32 */)
                                .into(),
                            );
                        }

                        children
                    })
                    .line_spacing(padding::MAIN as f32)
                    .spacing(padding::MAIN as f32)],
                )
            } else {
                section_box(
                    (
                        bootstrap_icon(BootstrapIcon::Cpu),
                        text(String::from("CPU")),
                        row![
                            checkbox("logical cores", preferences.show_logical_core_charts)
                                .on_toggle(CpuPageMessage::ToggleLogicalCores)
                        ],
                    ),
                    column![
                        self.cpu_chart.view(None).map(move |message| {
                            CpuPageMessage::ResourceChartMessage(message)
                        }),
                        seperator_background_1(),
                        split_table_double(vec![(
                            (
                                text(String::from("Percent used")),
                                text(format!("{:.1}%", data.cpu_usage_percent)),
                            ),
                            (
                                text(String::from("Frequency")),
                                text(format_hz(preferences, data.frequency as f32))
                            )
                        )]),
                    ],
                )
            }
        };

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
                (
                    text(String::from("Physical cores")),
                    text(format!("{}", physical_core_count)),
                ),
                (
                    text(String::from("Logical cores")),
                    text(format!("{}", logical_core_count)),
                ),
                (
                    text(String::from("Brand")),
                    text(format!("{}", brand)),
                ),
                (
                    text(String::from("Max frequency")),
                    text(format_hz(preferences, 0.)),
                ),
                (text(String::from("Architecture")), text("x_96"),),
                (text(String::from("Virtualization")), text("Muh virtual"),),
            ])],
        );

        let main = container(
            column![cpu_details_ui, thermals, about]
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
