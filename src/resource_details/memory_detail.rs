use iced::{
    alignment, font, theme, widget::{column, container, row, scrollable, text, Container}, Alignment, Element, Font, Length, Theme
};
use iced_aw::style;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

#[derive(Debug, Default, Clone)]
pub enum MemoryDetailsMessage {
    #[default]
    State,
}

#[derive(Debug, Default)]
pub struct MemoryDetails {
    pub ram_usage: u64,
    pub ram_total: u64,
    pub swap_usage: u64,
    pub swap_total: u64,
}

impl MemoryDetails {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn on_tick(&mut self) {
        let system_info = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );

        self.ram_usage = system_info.used_memory();
        self.ram_total = system_info.total_memory();
        self.swap_usage = system_info.used_swap();
        self.swap_total = system_info.total_swap();
    }

    pub fn view(&self) -> Element<MemoryDetailsMessage> {
        let header = container(row!["Memory"])
            .center_x()
            .width(Length::Fill);

        let ram_details = {
            if self.ram_usage == 0 || self.ram_total == 0 {
                column!["No RAM data to display"]
            } else {
                let ram_percent = self.ram_usage * 100 / self.ram_total;

                column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::Memory.to_string())
                        .font(Font {family: font::Family::Name("bootstrap-icons"), ..Default::default() }),
                        text(String::from("Random Access Memory")),
                        // i in the top right that takes someone to a description of what RAM is
                    ]
                    .spacing(10),
                    row![
                        text(format!(
                            "{:.2} / {:.2} GB",
                            self.ram_usage as f64 / 1024. / 1024. / 1024.,
                            self.ram_total as f64 / 1024. / 1024. / 1024.
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
            if self.swap_usage == 0 || self.swap_total == 0 {
                column!["No Swap data to display"]
            } else {
                let swap_percent = self.swap_usage * 100 / self.swap_total;

                column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::HddRack.to_string())
                        .font(Font {family: font::Family::Name("bootstrap-icons"), ..Default::default() }),
                        text(String::from("Swap")) // i in the top right that takes someone to a description of what Swap is
                    ]
                    .spacing(10),
                    row![
                        text(format!(
                            "{:.2} / {:.2} GB",
                            self.swap_usage as f64 / 1024. / 1024. / 1024.,
                            self.swap_total as f64 / 1024. / 1024. / 1024.
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
                    .font(Font {family: font::Family::Name("bootstrap-icons"), ..Default::default() }),
                    text(String::from("Advanced"))
                ]
                .spacing(10),
                row![
                    text(iced_aw::graphics::icons::BootstrapIcon::InfoCircle.to_string())
                    .font(Font {family: font::Family::Name("bootstrap-icons"), ..Default::default() }),
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
