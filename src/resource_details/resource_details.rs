use std::borrow::BorrowMut;

use iced::{
    advanced::graphics::futures::backend::default, font, theme, widget::{
        button, column, container, horizontal_space, keyed_column, row, scrollable,
        shader::wgpu::{hal::empty::Resource, naga::proc},
        text,
    }, Alignment, Command, Element, Font, Length, Renderer, Theme
};
use iced_aw::{core::icons, grid, grid_row, icons::Bootstrap, Grid, GridRow};
use ordered_float::OrderedFloat;
use sysinfo::{MemoryRefreshKind, Pid, Process, ProcessRefreshKind, RefreshKind, System};

use crate::ResourceType;

use super::{
    applications_details::{ApplicationsDetails, ApplicationsDetailsMessage},
    memory_detail::{self, MemoryDetails, MemoryDetailsMessage},
};

pub enum ProcessesHeader {
    Name,
    Cpu,
    Memory,
    DiskRead,
    DiskWritten,
    Kill,
}

#[derive(Debug)]
pub struct ProcessDetails {
    pub id: Pid,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_read: u64,
    pub disk_written: u64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum SortDirection {
    Ascending,
    #[default]
    Descending,
}

#[derive(Debug)]
pub struct ProcessesDetails {
    pub processes: Vec<ProcessDetails>,
    pub sort_header: ProcessesHeader,
    pub sort_direction: SortDirection,
}

impl ProcessesDetails {
    pub fn sort_by_index(&mut self) {
        match self.sort_header {
            ProcessesHeader::Cpu => self
                .processes
                .sort_by_key(|process| OrderedFloat(process.cpu_usage)),
            ProcessesHeader::Memory => self.processes.sort_by_key(|process| process.memory_usage),
            ProcessesHeader::DiskRead => self.processes.sort_by_key(|process| process.disk_read),
            ProcessesHeader::DiskWritten => self.processes.sort_by_key(|process| process.disk_written),
            _ => (), // No sorting
        }
    }
}

struct ProcessesDetailsProcs;

impl ProcessesDetailsProcs {
    pub fn sort_by_index(
        processes: &mut Vec<ProcessDetails>,
        sort_index: u32,
        sort_direction: &SortDirection,
    ) {
        match sort_index {
            1 => {
                processes.sort_by_key(|process| OrderedFloat(process.cpu_usage));
            }
            2 => {
                processes.sort_by_key(|process| process.memory_usage);
            }
            3 => {
                processes.sort_by_key(|process| process.disk_read);
            }
            4 => {
                processes.sort_by_key(|process| process.disk_written);
            }
            _ => (), // No sorting
        };

        match sort_direction {
            SortDirection::Descending => processes.reverse(),
            SortDirection::Ascending => {}
        };
    }
}

#[derive(Debug, Clone)]
pub enum ResourceDetailsMessage {
    KillProcessId(Pid),
    SortByIndex(u32),
    SwitchSortDirection,
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
        let mut new_self = Self {
            resource: resource.clone(),
            ..Default::default()
        };

        match &resource {
            ResourceType::Processes => {
                new_self.processes_details = Some(ProcessesDetails {
                    processes: Vec::new(),
                    sort_header: 0,
                    sort_direction: SortDirection::default(),
                })
            }
            _ => {}
        }

        new_self
    }

    pub fn on_tick(&mut self, system_info: &mut System, cpu_count: u32) {
        match self.resource {
            ResourceType::Applications => {}
            ResourceType::Processes => {
                let Some(processes_details) = &mut self.processes_details else {
                    return;
                };

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

                ProcessesDetailsProcs::sort_by_index(
                    &mut processes,
                    processes_details.sort_header,
                    &processes_details.sort_direction,
                );

                processes_details.processes = processes;
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

    pub fn update(&mut self, message: ResourceDetailsMessage) -> Command<ResourceDetailsMessage> {
        println!("updated");

        (|| {
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
                ResourceDetailsMessage::SortByIndex(sort_index) => {
                    let Some(processes_details) = &mut self.processes_details else {
                        return;
                    };

                    processes_details.sort_header = sort_index;
                    // Also reset the sort direction since the user is sorting a different category
                    processes_details.sort_direction = SortDirection::default();

                    ProcessesDetailsProcs::sort_by_index(
                        &mut processes_details.processes,
                        processes_details.sort_header,
                        &processes_details.sort_direction,
                    );
                }
                ResourceDetailsMessage::SwitchSortDirection => {
                    let Some(processes_details) = &mut self.processes_details else {
                        return;
                    };

                    processes_details.sort_direction = match processes_details.sort_direction {
                        SortDirection::Descending => SortDirection::Ascending,
                        SortDirection::Ascending => SortDirection::Descending,
                    };
                }
            }
        })();

        Command::none()
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
                    // .style(theme::Container::Box)
                    .width(Length::Fill);

                let processes_header_strings =
                    vec!["Name", "CPU", "Memory", "Disk Read", "Disk Written", "Kill"];





                let processes_headers = GridRow::with_elements({
                    let mut elements: Vec<Element<ResourceDetailsMessage, Theme, Renderer>> = Vec::new();

                    let mut i: u32 = 0;
                    for string in processes_header_strings {
                        if i == processes_details.sort_header {
                            elements.push(
                                button(
                                    row![
                                        text(string),
                                        // Icon
                                        text(String::from({
                                            match processes_details.sort_direction {
                                                SortDirection::Descending => {
                                                    icons::Bootstrap::CaretUpFill
                                                }
                                                SortDirection::Ascending => {
                                                    icons::Bootstrap::CaretDownFill
                                                }
                                            }
                                        }))
                                        .font(Font {
                                            family: font::Family::Name("bootstrap-icons"),
                                            ..Default::default()
                                        }),
                                    ]
                                    .spacing(10),
                                )
                                .width(Length::Fill)
                                .on_press(ResourceDetailsMessage::SwitchSortDirection)
                                /* .style(theme::Button::Text) */
                                .into(),
                            )
                        } else {
                            elements.push(
                                button(string)
                                    .width(Length::Fill)
                                    .on_press(ResourceDetailsMessage::SortByIndex(i))
                                    /* .style(theme::Button::Text) */
                                    .into(),
                            );
                        }

                        i += 1;
                    }

                    elements
                });

                // let processes_totals = grid_row!(
                //     row![
                //         text(icons::Bootstrap::BarChart.to_string()).font(Font {
                //             family: font::Family::Name("bootstrap-icons"),
                //             ..Default::default()
                //         }),
                //         text("Total")
                //     ]
                //     .spacing(5).into(),
                //     text("CPU"),
                //     text("Memory"),
                //     text("Read"),
                //     text("Written"),
                //     text("Action"),
                // );

                let processes_totals = GridRow::with_elements(vec![
                    row![
                        text(icons::Bootstrap::BarChart.to_string()).font(Font {
                            family: font::Family::Name("bootstrap-icons"),
                            ..Default::default()
                        }),
                        text("Total")
                    ]
                    .spacing(5).into(),
                    // text("CPU"),
                    // text("Memory"),
                    // text("Read"),
                    // text("Written"),
                    // text("Action"),
                ]);

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
                .column_width(0)
                .row_spacing(10)
                .column_spacing(0);

                // let main =
                //     grid!(grid_row!(processes_headers), grid_row!(processes)).width(Length::Shrink);

                // let main = column![processes_headers, processes].width(Length::Shrink);

                let content = column![header, scrollable(main.into())]
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
                    // .style(theme::Container::Box)
                    .width(Length::Fill);

                let ram_details = {
                    if memory_details.ram_usage == 0 || memory_details.ram_total == 0 {
                        column!["No RAM data to display"]
                    } else {
                        let ram_percent = memory_details.ram_usage * 100 / memory_details.ram_total;

                        column![
                            row![
                                text(icons::Bootstrap::Memory.to_string()).font(Font {
                                    family: font::Family::Name("bootstrap-icons"),
                                    ..Default::default()
                                }),
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
                                text(icons::Bootstrap::HddRack.to_string()).font(Font {
                                    family: font::Family::Name("bootstrap-icons"),
                                    ..Default::default()
                                }),
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
                            text(icons::Bootstrap::Tools.to_string()).font(Font {
                                family: font::Family::Name("bootstrap-icons"),
                                ..Default::default()
                            }),
                            text(String::from("Advanced"))
                        ]
                        .spacing(10),
                        row![
                            text(Bootstrap::InfoCircle.to_string()).font(Font {
                                family: font::Family::Name("bootstrap-icons"),
                                ..Default::default()
                            }),
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
