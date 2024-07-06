use iced::{
    alignment, theme,
    widget::{column, container, row, scrollable, text},
    Alignment, Element, Length, Theme,
};
use iced_aw::style;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use crate::{preferences::Preferences, MemoryData};

use super::chart::ResourceChart;

#[derive(Debug, Clone)]
pub enum MemoryPageMessage {

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

    pub fn view(&self, preferences: &Preferences, data: &MemoryData) -> Element<MemoryPageMessage> {
        let header = container(row!["Memory"])
            .center_x()
            .style(theme::Container::Box)
            .width(Length::Fill);

        let ram_details = {
            if data.ram_usage == 0 || data.ram_total == 0 {
                column!["No RAM data to display"]
            } else {
                let ram_percent = data.ram_usage * 100 / data.ram_total;

                column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::Memory.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT),
                        text(String::from("Random Access Memory")),
                        // i in the top right that takes someone to a description of what RAM is
                    ]
                    .spacing(10),
                    row![
                        text(format!(
                            "{:.2} / {:.2} GB",
                            data.ram_usage as f64 / 1024. / 1024. / 1024.,
                            data.ram_total as f64 / 1024. / 1024. / 1024.
                        )),
                        /* use a dot like with lists */ text(String::from(" • ")),
                        text(format!("{}%", ram_percent))
                    ],
                    text(String::from("graph")),
                ]
                .spacing(5)
            }
        };

        let swap_details = {
            if data.swap_usage == 0 || data.swap_total == 0 {
                column!["No Swap data to display"]
            } else {
                let swap_percent = data.swap_usage * 100 / data.swap_total;

                column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::HddRack.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT),
                        text(String::from("Swap")) // i in the top right that takes someone to a description of what Swap is
                    ]
                    .spacing(10),
                    row![
                        text(format!(
                            "{:.2} / {:.2} GB",
                            data.swap_usage as f64 / 1024. / 1024. / 1024.,
                            data.swap_total as f64 / 1024. / 1024. / 1024.
                        )),
                        /* use a dot like with lists */ text(String::from(" • ")),
                        text(format!("{}%", swap_percent))
                    ],
                    text(String::from("graph")),
                ]
                .spacing(5)
            }
        };

        let main = container(
            column![
                ram_details,
                swap_details,
                row![
                    text(iced_aw::graphics::icons::BootstrapIcon::Tools.to_string())
                        .font(iced_aw::BOOTSTRAP_FONT),
                    text(String::from("Advanced"))
                ]
                .spacing(10),
                row![
                    text(iced_aw::graphics::icons::BootstrapIcon::InfoCircle.to_string())
                        .font(iced_aw::BOOTSTRAP_FONT),
                    text(String::from("About"))
                ]
                .spacing(10),
            ]
            .spacing(20),
        )
        .center_x()
        .width(Length::Shrink);

        let content = column![header, scrollable(main)]
            .spacing(20)
            .align_items(Alignment::Center);

        let container = container(content);
        container.into()
    }
}
