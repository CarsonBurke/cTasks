use iced::{
    advanced::graphics::futures::backend::default,
    theme,
    widget::{
        button, column, container, horizontal_space, keyed_column, row, scrollable,
        shader::wgpu::hal::empty::Resource, text,
    },
    Alignment, Element, Length, Theme,
};
use iced_aw::{grid, grid_row, Grid, GridRow};
use sysinfo::{MemoryRefreshKind, Process, ProcessRefreshKind, RefreshKind, System};

use crate::ResourceType;

use super::{
    applications_details::{ApplicationsDetails, ApplicationsDetailsMessage},
    memory_detail::{self, MemoryDetails, MemoryDetailsMessage},
};

#[derive(Debug)]
pub struct ProcessDetails {
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_read: u64,
    pub disk_written: u64,
    pub kill: fn(&Process) -> bool,
}

#[derive(Debug)]
pub struct ProcessesDetails {
    pub processes: Vec<ProcessDetails>,
    pub sort_index: i32,
}

#[derive(Debug, Clone)]
pub enum ResourceDetailsMessage {
    MemoryDetailsMessage(MemoryDetailsMessage),
    ApplicationsDetailsMessage(ApplicationsDetailsMessage),
}

// pub type ResourceDetailsElements = MemoryDetails & ApplicationsDetails;

#[derive(Debug, Default)]
pub struct ResourceDetails {
    resource: ResourceType,
    memory_details: Option<MemoryDetails>,
    processes_details: Option<ProcessesDetails>,
}

impl ResourceDetails {
    pub fn new(resource: ResourceType) -> Self {
        Self {
            resource,
            ..Default::default()
        }
    }

    pub fn on_tick(&mut self) {
        match self.resource {
            ResourceType::Applications => {}
            ResourceType::Processes => {
                let system_info = System::new_with_specifics(
                    RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
                );

                let mut processes = Vec::new();

                for (pid, process) in system_info.processes() {
                    let disk_usage = process.disk_usage();

                    processes.push(ProcessDetails {
                        name: process.name().to_string(),
                        cpu_usage: process.cpu_usage(),
                        memory_usage: process.memory(),
                        disk_read: disk_usage.read_bytes,
                        disk_written: disk_usage.written_bytes,
                        kill: move |process| process.kill(),
                    })
                }

                self.processes_details = Some(ProcessesDetails { processes });
            }
            ResourceType::Memory => {
                let system_info = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                self.memory_details = Some(MemoryDetails {
                    ram_usage: system_info.used_memory(),
                    ram_total: system_info.total_memory(),
                    swap_usage: system_info.used_swap(),
                    swap_total: system_info.total_swap(),
                });
            }
            ResourceType::Cpu => {}
            ResourceType::Gpu => {}
            ResourceType::Disk => {}
            ResourceType::Wifi => {}
            ResourceType::Ethernet => {}
        };
    }

    pub fn view(&self) -> Element<ResourceDetailsMessage> {
        match &self.resource {
            ResourceType::Applications => {
                let content = row![];

                let container = container(content);
                container.into()
            }
            ResourceType::Processes => {
                let Some(processes_details) = &self.processes_details else {
                    return text("Loading...").into();
                };

                let header = container(row!["Processes"])
                    .center_x()
                    .style(theme::Container::Box)
                    .width(Length::Fill);

                let processes_headers = grid_row!(
                    text("Name"),
                    text("CPU"),
                    text("Memory"),
                    text("Disk Read"),
                    text("Disk Written"),
                    text("Kill")
                );

                // let processes: Element<_> = {
                //     keyed_column(processes_details.processes.iter().enumerate().map(
                //         |(i, process_details)| {
                //             (
                //                 i,
                //                 row![
                //                     text(format!["{}", process_details.name]),
                //                     horizontal_space(),
                //                     text(format!["{}", process_details.cpu_usage]),
                //                     horizontal_space(),
                //                     text(format!["{}", process_details.memory_usage]),
                //                     horizontal_space(),
                //                     text(format!["{}", process_details.disk_read]),
                //                     horizontal_space(),
                //                     text(format!["{}", process_details.disk_written]),
                //                     horizontal_space(),
                //                     button(text("Kill")),
                //                 ]
                //                 .width(Length::Shrink)
                //                 .spacing(10)
                //                 .into(),
                //             )
                //         },
                //     ))
                //     .width(Length::Shrink)
                //     .into()
                // };

                // let test =
                //     grid!(grid_row!(text(String::from("Header"))), {
                //         processes_details.processes.iter().enumerate().map(
                //             |(i, process_details)| {
                //                 grid_row!(
                //                     text(format!["{}", process_details.name]),
                //                     horizontal_space(),
                //                     text(format!["{}", process_details.cpu_usage]),
                //                     horizontal_space(),
                //                     text(format!["{}", process_details.memory_usage]),
                //                     horizontal_space(),
                //                     text(format!["{}", process_details.disk_read]),
                //                     horizontal_space(),
                //                     text(format!["{}", process_details.disk_written]),
                //                     horizontal_space(),
                //                     button(text("Kill")),
                //                 )
                //                 .into()
                //             },
                //         )
                //     })
                //     .width(Length::Shrink);

                // let main: Grid<ResourceDetailsMessage, Theme, iced::Renderer> =
                //     grid!(processes_headers);

                // for process_details in &processes_details.processes {
                //     main.push(grid_row!(
                //         text(format!["{}", process_details.name]),
                //         text(format!["{}", process_details.cpu_usage]),
                //         text(format!["{}", process_details.memory_usage]),
                //         text(format!["{}", process_details.disk_read]),
                //         text(format!["{}", process_details.disk_written]),
                //         button(text("Kill")),
                //     ));
                // }

                let test = Grid::with_rows({
                    let mut rows = Vec::new();
                    rows.push(processes_headers);

                    for process_details in &processes_details.processes {
                        rows.push(grid_row!(
                            text(format!["{}", process_details.name]),
                            text(format!["{}", process_details.cpu_usage]),
                            text(format!["{}", process_details.memory_usage]),
                            text(format!["{}", process_details.disk_read]),
                            text(format!["{}", process_details.disk_written]),
                            button(text("Kill")),
                        ))
                    }

                    rows
                }).column_width(Length::Shrink);

                // let main =
                //     grid!(grid_row!(processes_headers), grid_row!(processes)).width(Length::Shrink);

                // let main = column![processes_headers, processes].width(Length::Shrink);

                let content = column![header, scrollable(test)]
                    .spacing(20)
                    .align_items(Alignment::Center);

                let container = container(content);
                container.into()
            }
            ResourceType::Memory => {
                let Some(memory_details) = &self.memory_details else {
                    return text("Loading...").into();
                };

                let header = container(row!["Memory"])
                    .center_x()
                    .style(theme::Container::Box)
                    .width(Length::Fill);

                let ram_details = {
                    if memory_details.ram_usage == 0 || memory_details.ram_total == 0 {
                        column!["No RAM data to display"]
                    } else {
                        let ram_percent = memory_details.ram_usage * 100 / memory_details.ram_total;

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
                                    memory_details.ram_usage as f64 / 1024. / 1024. / 1024.,
                                    memory_details.ram_total as f64 / 1024. / 1024. / 1024.
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
                    if memory_details.swap_usage == 0 || memory_details.swap_total == 0 {
                        column!["No Swap data to display"]
                    } else {
                        let swap_percent =
                            memory_details.swap_usage * 100 / memory_details.swap_total;

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
                                    memory_details.swap_usage as f64 / 1024. / 1024. / 1024.,
                                    memory_details.swap_total as f64 / 1024. / 1024. / 1024.
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
            ResourceType::Cpu => {
                let content = row![];

                let container = container(content);
                container.into()
            }
            ResourceType::Gpu => {
                let content = row![];

                let container = container(content);
                container.into()
            }
            ResourceType::Disk => {
                let content = row![];

                let container = container(content);
                container.into()
            }
            ResourceType::Wifi => {
                let content = row![];

                let container = container(content);
                container.into()
            }
            ResourceType::Ethernet => {
                let content = row![];

                let container = container(content);
                container.into()
            }
        }
    }
}
