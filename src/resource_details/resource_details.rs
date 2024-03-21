use std::borrow::BorrowMut;

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
use ordered_float::OrderedFloat;
use sysinfo::{MemoryRefreshKind, Pid, Process, ProcessRefreshKind, RefreshKind, System};

use crate::ResourceType;

use super::{
    applications_details::{ApplicationsDetails, ApplicationsDetailsMessage},
    memory_detail::{self, MemoryDetails, MemoryDetailsMessage},
};

#[derive(Debug)]
pub struct ProcessDetails {
    pub id: Pid,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_read: u64,
    pub disk_written: u64,
}

#[derive(Debug)]
pub struct ProcessesDetails {
    pub processes: Vec<ProcessDetails>,
    pub sort_index: u32,
    pub sort_ascending: bool,
}

impl ProcessesDetails {
    pub fn sort_by_index(&mut self) {
        match self.sort_index {
            1 => self
                .processes
                .sort_by_key(|process| OrderedFloat(process.cpu_usage)),
            2 => self.processes.sort_by_key(|process| process.memory_usage),
            3 => self.processes.sort_by_key(|process| process.disk_read),
            4 => self.processes.sort_by_key(|process| process.disk_written),
            _ => (), // No sorting
        }
    }
}

struct ProcessesDetailsProcs;

impl ProcessesDetailsProcs {
    pub fn sort_by_index(
        processes: &mut Vec<ProcessDetails>,
        sort_index: u32,
        sort_ascending: bool,
    ) {
        match sort_index {
            1 => {
                processes.sort_by_key(|process| OrderedFloat(process.cpu_usage));
                processes.reverse()
            }
            2 => {
                processes.sort_by_key(|process| process.memory_usage);
                processes.reverse()
            }
            3 => {
                processes.sort_by_key(|process| process.disk_read);
                processes.reverse()
            }
            4 => {
                processes.sort_by_key(|process| process.disk_written);
                processes.reverse()
            }
            _ => (), // No sorting
        }
    }
}

#[derive(Debug, Clone)]
pub enum ResourceDetailsMessage {
    KillProcessId(Pid),
}

// pub type ResourceDetailsElements = MemoryDetails & ApplicationsDetails;

#[derive(Debug, Default)]
pub struct ResourceDetails {
    pub resource: ResourceType,
    preview_values: Option<u32>,
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

    pub fn on_tick(&mut self, system_info: &mut System, cpu_count: u32) {
        match self.resource {
            ResourceType::Applications => {}
            ResourceType::Processes => {

                system_info.refresh_processes();

                let mut processes = Vec::new();

                for (pid, process) in system_info.processes() {
                    let disk_usage = process.disk_usage();

                    processes.push(ProcessDetails {
                        name: process.name().to_string(),
                        id: pid.clone(),
                        cpu_usage: process.cpu_usage() / cpu_count as f32,
                        memory_usage: process.memory(),
                        disk_read: disk_usage.read_bytes,
                        disk_written: disk_usage.written_bytes,
                    })
                }

                if let Some(processes_details) = &self.processes_details {
                    ProcessesDetailsProcs::sort_by_index(
                        &mut processes,
                        processes_details.sort_index,
                        processes_details.sort_ascending,
                    );
                }

                self.processes_details = Some(ProcessesDetails {
                    processes,
                    sort_index: 1,
                    sort_ascending: false,
                });
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

    fn update(&mut self, message: ResourceDetailsMessage) {
        match message {
            ResourceDetailsMessage::KillProcessId(pid) => {
                let Some(processes_details) = &mut self.processes_details else {
                    return;
                };

                let system_info = System::new_with_specifics(
                    RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
                );

                let Some(process) = system_info.process(pid) else {
                    return;
                };

                // The process still exists. Kill it

                process.kill();
                println!("Killed {}", process.name());
            }
        }
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

                let processes_header_strings =
                    vec!["Name", "CPU", "Memory", "Disk Read", "Disk Written", "Kill"];

                let processes_headers = GridRow::with_elements({
                    let mut elements = Vec::new();

                    let mut i: u32 = 0;
                    for string in processes_header_strings {
                        if i == processes_details.sort_index {
                            elements.push(text(format!["{} ▼", string]))
                        } else {
                            elements.push(text(string));
                        }

                        i += 1;
                    }

                    elements
                });

                let processes_totals = grid_row!(
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::BarChart.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT),
                        text("Total")
                    ]
                    .spacing(5),
                    text("CPU"),
                    text("Memory"),
                    text("Read"),
                    text("Written"),
                    text("Action"),
                );

                let main = Grid::with_rows({
                    let mut rows = Vec::new();
                    rows.push(processes_headers);
                    rows.push(processes_totals);

                    for process_details in &processes_details.processes {
                        rows.push(grid_row!(
                            text(format!["{}", process_details.name]),
                            text(format!["{:.2}%", process_details.cpu_usage]),
                            text(format![
                                "{:.2} MB",
                                process_details.memory_usage as f64 / 1024. / 1024.
                            ]),
                            text(format![
                                "{:.2} MB",
                                process_details.disk_read as f64 / 1024. / 1024.
                            ]),
                            text(format![
                                "{:.2} MB",
                                process_details.disk_written as f64 / 1024. / 1024.
                            ]),
                            button(text("Kill")).on_press(ResourceDetailsMessage::KillProcessId(
                                process_details.id
                            )),
                        ))
                    }

                    rows
                })
                .column_width(Length::Shrink);

                // let main =
                //     grid!(grid_row!(processes_headers), grid_row!(processes)).width(Length::Shrink);

                // let main = column![processes_headers, processes].width(Length::Shrink);

                let content = column![header, scrollable(main)]
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
