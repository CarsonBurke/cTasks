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
    },
    general_widgets::{
        icons::bootstrap_icon,
        section::{section, section_box, section_box_headless},
        seperators::seperator_background_1,
        split_table_double::split_table_double,
        split_table_single::split_table_single,
    },
    preferences::Preferences,
    styles::{self, container::resource_details_header},
    types::resource_data::{BatteryData, CpuData},
    utils::format_hz,
    ResourceHistory,
};

use super::chart::{ResourceChart, ResourceChartMessage};

#[derive(Debug, Clone)]
pub enum BatteryPageMessage {
    ResourceChartMessage(ResourceChartMessage),
}

#[derive(Debug)]
pub struct BatteryPage {
    pub battery_charge_chart: ResourceChart,
}

impl BatteryPage {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            battery_charge_chart: ResourceChart::new(preferences),
        }
    }

    pub fn update(&mut self, message: BatteryPageMessage) -> Command<BatteryPageMessage> {
        match message {
            _ => Command::none(),
        }
    }

    pub fn update_history(&mut self, resource_history: &ResourceHistory) {
        self.battery_charge_chart.data_points = resource_history.battery_charge.clone();
    }

    pub fn view(
        &self,
        preferences: &Preferences,
        data: &BatteryData,
    ) -> Element<BatteryPageMessage> {
        let header = container(row!["Battery"])
            .center_x()
            .style(resource_details_header())
            .width(Length::Fill)
            .padding(padding::MAIN);

        let charge_details =
            section_box(
                (
                    bootstrap_icon(BootstrapIcon::Memory),
                    text(String::from("Random Access Memory")),
                    row![],
                ),
                {
                    column![
                        container(self.battery_charge_chart.view(None).map(move |message| {
                            BatteryPageMessage::ResourceChartMessage(message)
                        })),
                        seperator_background_1(),
                        split_table_single(vec![(
                            text("Percent charge".to_string()),
                            text(format!("{:.1?}%", data.state_of_charge * 100.))
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
                text({
                    if let Some(temperature) = data.temperature {
                        format!("{:.2}°C", temperature.value)
                    } else {
                        "Unknown".to_string()
                    }
                }),
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
                    text(String::from("Designed capacity")),
                    text(format!("{}", data.designed_capacity.value)),
                ),
                (
                    text(String::from("Current capacity")),
                    text(format!("{}", data.current_capacity.value)),
                ),
                (
                    text(String::from("Cycles")),
                    text({
                        if let Some(cycles) = data.cycles {
                            format!("{}", cycles)
                        } else {
                            "Unknown".to_string()
                        }
                    })
                ),
                (
                    text(String::from("Voltage")),
                    text(format!("{:.2} volts", data.voltage.value)),
                ),
                (
                    text(String::from("Health")),
                    text(format!("{:.0}%", data.state_of_health.value * 100.)),
                ),
            ])],
        );

        let main = container(
            column![charge_details, thermals, about]
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
