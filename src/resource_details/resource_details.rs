use std::{borrow::BorrowMut, collections::VecDeque};

use iced::{
    advanced::graphics::futures::backend::default,
    alignment,
    theme::{self, Text},
    widget::{
        self, button, checkbox, column, container, horizontal_space, keyed_column, row, scrollable,
        shader::wgpu::{hal::empty::Resource, naga::proc},
        text, text_input, vertical_space, Themer,
    },
    window::Action,
    Alignment, Command, Element, Length, Theme,
};
use iced_aw::{grid, grid_row, BootstrapIcon, Grid, GridRow};
use ordered_float::OrderedFloat;
use plotters_iced::{Chart, ChartWidget};
use sysinfo::{MemoryRefreshKind, Pid, Process, ProcessRefreshKind, RefreshKind, System};

use crate::{
    constants::{custom_theme, font_sizes, padding, sizings, HISTORY_TICKS},
    general_widgets::{section_box::section_box, split_table_double::split_table_double},
    styles::{
        self,
        container::{
            alternate_process_grid_row, divider_background_1, primary_process_grid_row,
            resource_details_child, resource_details_header,
        },
    },
    ResourceHistory, ResourceHistoryTick, ResourceType,
};

use super::{
    applications_details::{ApplicationsDetails, ApplicationsDetailsMessage},
    chart::ResourceChart,
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

#[derive(Debug, Default, Clone, PartialEq)]
pub enum SortDirection {
    Ascending,
    #[default]
    Descending,
}

#[derive(Debug)]
pub struct ProcessesDetails {
    pub processes: Vec<ProcessDetails>,
    pub sort_index: u32,
    pub sort_direction: SortDirection,
}

struct ProcessesDetailsProcs;

impl ProcessesDetailsProcs {
    pub fn sort_by_index(
        processes: &mut Vec<ProcessDetails>,
        sort_index: u32,
        sort_direction: &SortDirection,
    ) {
        match sort_index {
            0 => {
                processes.sort_by_key(|process| process.name.to_lowercase());
            }
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

#[derive(Debug)]
pub struct CpuDetails {
    pub cpu_usage_percent: f32,
    pub physical_core_count: u32,
    pub logical_core_count: u32,
    pub cpu_chart: ResourceChart,
    pub brand: String,
    pub frequency: u64,
}

#[derive(Debug, Clone)]
pub enum ResourceDetailsMessage {
    KillProcessId(Pid),
    SortByIndex(u32),
    SwitchSortDirection,
    ChangeSwapiness,
}

// pub type ResourceDetailsElements = MemoryDetails & ApplicationsDetails;

#[derive(Debug, Default)]
pub struct ResourceDetails {
    pub resource: ResourceType,
    preview_values: Option<u32>,
    memory_details: Option<MemoryDetails>,
    processes_details: Option<ProcessesDetails>,
    cpu_details: Option<CpuDetails>,
}

impl ResourceDetails {
    pub fn new(resource: ResourceType) -> Self {
        let mut new_self = Self {
            ..Default::default()
        };
        new_self.apply_resource_type(resource);

        new_self
    }

    pub fn apply_resource_type(&mut self, resource: ResourceType) {
        self.resource = resource.clone();

        match &resource {
            ResourceType::Processes => {
                self.processes_details = Some(ProcessesDetails {
                    processes: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::default(),
                })
            }
            ResourceType::Memory => {
                self.memory_details = Some(MemoryDetails {
                    ram_usage: 0,
                    ram_total: 0,
                    swap_usage: 0,
                    swap_total: 0,
                    ram_chart: ResourceChart::new(),
                    swap_chart: ResourceChart::new(),
                })
            }
            ResourceType::Cpu => {
                self.cpu_details = Some(CpuDetails {
                    cpu_usage_percent: 0.0,
                    physical_core_count: 0,
                    logical_core_count: 0,
                    brand: String::new(),
                    frequency: 0,
                    cpu_chart: ResourceChart::new(),
                })
            }
            _ => {}
        };
    }

    pub fn on_tick(
        &mut self,
        system_info: &mut System,
        cpu_usage_percent: f32,
        physical_cpu_count: u32,
        logical_cpu_count: u32,
        cpu_brand: String,
        cpu_frequency: u64,
        resource_history: &ResourceHistory,
    ) {
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

                    let Some(path) = process.exe() else { continue };

                    if path.exists() {
                        path.read_dir().ok().map(|read| {
                            read.filter_map(|file_res| {
                                file_res.ok().and_then(|file| {
                                    let file_path = file.path();

                                    let Ok(ini) = ini::Ini::load_from_file(file_path) else {
                                        return Some(());
                                    };

                                    let Some(desktop_entry) = ini.section(Some("Desktop Entry"))
                                    else {
                                        return Some(());
                                    };

                                    if let Some(icon) = desktop_entry.get("Icon") {
                                        println!("has icon, {}", icon);
                                    }

                                    Some(())
                                })
                            })
                        });
                    }

                    processes.push(ProcessDetails {
                        name: process.name().to_string(),
                        id: pid.clone(),
                        cpu_usage: process.cpu_usage() / logical_cpu_count as f32,
                        memory_usage: process.memory(),
                        disk_read: disk_usage.read_bytes,
                        disk_written: disk_usage.written_bytes,
                    })
                }

                ProcessesDetailsProcs::sort_by_index(
                    &mut processes,
                    processes_details.sort_index,
                    &processes_details.sort_direction,
                );

                processes_details.processes = processes;
            }
            ResourceType::Memory => {
                let system_info = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                let Some(memory_details) = &mut self.memory_details else {
                    return;
                };

                memory_details.ram_usage = system_info.used_memory();
                memory_details.ram_total = system_info.total_memory();
                memory_details.swap_usage = system_info.used_swap();
                memory_details.swap_total = system_info.total_swap();

                // RAM usage history

                memory_details.ram_chart.data_points = resource_history.ram.clone();

                // Swap usage history
                memory_details.swap_chart.data_points = resource_history.swap.clone();
            }
            ResourceType::Cpu => {
                let Some(cpu_details) = &mut self.cpu_details else {
                    return;
                };

                cpu_details.cpu_usage_percent = cpu_usage_percent;
                cpu_details.physical_core_count = physical_cpu_count;
                cpu_details.logical_core_count = logical_cpu_count;
                cpu_details.brand = cpu_brand;
                cpu_details.frequency = cpu_frequency;

                // cpu usage history

                cpu_details.cpu_chart.data_points = resource_history.cpu.clone();
            }
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

                    processes_details.sort_index = sort_index;
                    // Also reset the sort direction since the user is sorting a different category
                    processes_details.sort_direction = SortDirection::default();

                    ProcessesDetailsProcs::sort_by_index(
                        &mut processes_details.processes,
                        processes_details.sort_index,
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
                ResourceDetailsMessage::ChangeSwapiness => {
                    println!("change swapiness")
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
                    return text("Waiting for tick").into();
                };

                let header = container(row!["Processes"])
                    .center_x()
                    .style(resource_details_header())
                    .width(Length::Fill)
                    .padding(padding::MAIN);

                let processes_header_strings =
                    vec!["Name", "CPU", "Memory", "Disk Read", "Disk Written", "Kill"];

                let processes_headers = GridRow::with_elements({
                    let mut elements = Vec::new();

                    let mut i: u32 = 0;
                    for string in processes_header_strings {
                        if i == processes_details.sort_index {
                            elements.push(
                                button(
                                    row![
                                        text(string),
                                        // Icon
                                        text(String::from({
                                            match processes_details.sort_direction {
                                                SortDirection::Descending => {
                                                    BootstrapIcon::CaretDownFill
                                                }
                                                SortDirection::Ascending => {
                                                    BootstrapIcon::CaretUpFill
                                                }
                                            }
                                        }))
                                        .font(iced_aw::BOOTSTRAP_FONT)
                                    ]
                                    .spacing(10),
                                )
                                .width(Length::Fill)
                                .on_press(ResourceDetailsMessage::SwitchSortDirection)
                                .style(theme::Button::Text),
                            )
                        } else {
                            elements.push(
                                button(string)
                                    .width(Length::Fill)
                                    .on_press(ResourceDetailsMessage::SortByIndex(i))
                                    .style(theme::Button::Text),
                            );
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

                let main = container(
                    Grid::with_rows({
                        let mut rows = Vec::new();
                        rows.push(processes_headers);
                        rows.push(processes_totals);

                        let mut i: u32 = 0;

                        for process_details in &processes_details.processes {
                            let is_odd = i % 2 == 1;
                            // let styler = if is_odd {
                            //     alternate_process_grid_row()
                            // } else {
                            //     primary_process_grid_row()
                            // };

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
                                button(text("Kill")).on_press(
                                    ResourceDetailsMessage::KillProcessId(process_details.id)
                                ),
                            ));

                            i += 1;
                        }

                        rows
                    })
                    .column_width(Length::Shrink)
                    .row_spacing(10)
                    .column_spacing(0),
                )
                .padding(padding::MAIN)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center);

                let content = column![header, scrollable(main)];

                let container = container(content);
                container.into()
            }
            ResourceType::Memory => {
                let Some(memory_details) = &self.memory_details else {
                    return text("Waiting for tick").into();
                };

                let header = container(row!["Memory"])
                    .center_x()
                    .style(resource_details_header())
                    .width(Length::Fill)
                    .padding(padding::MAIN);

                let ram_details = column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::Memory.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT)
                            .size(font_sizes::H2),
                        text(String::from("Random Access Memory")).size(font_sizes::H2),
                        // i in the top right that takes someone to a description of what RAM is
                    ]
                    .spacing(10),
                    container({
                        if memory_details.ram_usage == 0 || memory_details.ram_total == 0 {
                            column!["No RAM data to display"]
                        } else {
                            column![
                                row![
                                    column![
                                        text(String::from("Usage"))
                                            .style(Text::Color(custom_theme::GREY_TEXT)),
                                        text(format!(
                                            "{:.2} / {:.2} GB",
                                            memory_details.ram_usage as f64 / 1024. / 1024. / 1024.,
                                            memory_details.ram_total as f64 / 1024. / 1024. / 1024.
                                        )),
                                    ],
                                    horizontal_space(),
                                    column![
                                        row![
                                            horizontal_space(),
                                            text(String::from("Percent used"))
                                                .style(Text::Color(custom_theme::GREY_TEXT))
                                        ],
                                        row![
                                            horizontal_space(),
                                            text(format!(
                                                "{:.1}%",
                                                memory_details.ram_usage as f64
                                                    / memory_details.ram_total as f64
                                                    * 100.
                                            )),
                                        ],
                                    ]
                                ]
                                .padding(padding::MAIN),
                                container(row![])
                                    .style(divider_background_1())
                                    .width(Length::Fill)
                                    .height(1),
                                container(memory_details.ram_chart.view()) //.padding(padding::MAIN),
                            ]
                            .spacing(5)
                        }
                    })
                    .style(resource_details_child())
                    .width(Length::Fill)
                    .center_y()
                ]
                .spacing(padding::PORTION)
                .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH);

                let swap_details = section_box(
                    (BootstrapIcon::HddRack, String::from("Swap")),
                    column![{
                        if memory_details.swap_usage == 0 || memory_details.swap_total == 0 {
                            column!["No Swap data to display"]
                        } else {
                            column![
                                split_table_double(vec![(
                                    (
                                        "Usage".to_string(),
                                        format!(
                                            "{:.2} / {:.2} GB",
                                            memory_details.swap_usage as f64
                                                / 1024.
                                                / 1024.
                                                / 1024.,
                                            memory_details.swap_total as f64
                                                / 1024.
                                                / 1024.
                                                / 1024.
                                        )
                                    ),
                                    (
                                        "Percent used".to_string(),
                                        format!(
                                            "{:.1}%",
                                            memory_details.swap_usage as f64
                                                / memory_details.swap_total as f64
                                                * 100.
                                        )
                                    )
                                )]),
                                container(row![])
                                    .style(divider_background_1())
                                    .width(Length::Fill)
                                    .height(1),
                                container(memory_details.swap_chart.view())
                            ]
                        }
                    }],
                );
                // column![
                //     row![
                //         text(iced_aw::graphics::icons::BootstrapIcon::HddRack.to_string())
                //             .font(iced_aw::BOOTSTRAP_FONT)
                //             .size(font_sizes::H2),
                //         text(String::from("Swap")).size(font_sizes::H2) // i in the top right that takes someone to a description of what Swap is
                //     ]
                //     .spacing(padding::MAIN),
                //     container({
                //         if memory_details.swap_usage == 0 || memory_details.swap_total == 0 {
                //             column!["No Swap data to display"]
                //         } else {
                //             column![
                //                 split_table_double(vec![(
                //                     (
                //                         "Usage".to_string(),
                //                         format!(
                //                             "{:.2} / {:.2} GB",
                //                             memory_details.swap_usage as f64
                //                                 / 1024.
                //                                 / 1024.
                //                                 / 1024.,
                //                             memory_details.swap_total as f64
                //                                 / 1024.
                //                                 / 1024.
                //                                 / 1024.
                //                         )
                //                     ),
                //                     (
                //                         "Percent used".to_string(),
                //                         format!(
                //                             "{:.1}%",
                //                             memory_details.swap_usage as f64
                //                                 / memory_details.swap_total as f64
                //                                 * 100.
                //                         )
                //                     )
                //                 )]),
                //                 container(row![])
                //                     .style(divider_background_1())
                //                     .width(Length::Fill)
                //                     .height(1),
                //                 container(memory_details.swap_chart.view())
                //             ]
                //         }
                //     })
                //     .style(resource_details_child())
                //     .width(Length::Fill)
                //     .center_y()
                // ]
                // .spacing(padding::PORTION)
                // .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH);

                // let swap_details = column![
                //     row![
                //         text(iced_aw::graphics::icons::BootstrapIcon::HddRack.to_string())
                //             .font(iced_aw::BOOTSTRAP_FONT)
                //             .size(font_sizes::H2),
                //         text(String::from("Swap")).size(font_sizes::H2) // i in the top right that takes someone to a description of what Swap is
                //     ]
                //     .spacing(padding::MAIN),
                //     container({
                //         if memory_details.swap_usage == 0 || memory_details.swap_total == 0 {
                //             column!["No Swap data to display"]
                //         } else {
                //             column![
                //                 row![
                //                     column![
                //                         text(String::from("Usage"))
                //                             .style(Text::Color(custom_theme::GREY_TEXT)),
                //                         text(format!(
                //                             "{:.2} / {:.2} GB",
                //                             memory_details.swap_usage as f64
                //                                 / 1024.
                //                                 / 1024.
                //                                 / 1024.,
                //                             memory_details.swap_total as f64
                //                                 / 1024.
                //                                 / 1024.
                //                                 / 1024.
                //                         )),
                //                     ],
                //                     horizontal_space(),
                //                     column![
                //                         row![
                //                             horizontal_space(),
                //                             text(String::from("Percent used"))
                //                                 .style(Text::Color(custom_theme::GREY_TEXT))
                //                         ],
                //                         row![
                //                             horizontal_space(),
                //                             text(format!(
                //                                 "{:.1}%",
                //                                 memory_details.swap_usage as f64
                //                                     / memory_details.swap_total as f64
                //                                     * 100.
                //                             )),
                //                         ],
                //                     ]
                //                 ]
                //                 .padding(padding::MAIN),
                //                 container(row![])
                //                     .style(divider_background_1())
                //                     .width(Length::Fill)
                //                     .height(1),
                //                 container(memory_details.swap_chart.view()) //.padding(padding::MAIN),
                //             ]
                //             .spacing(5)
                //         }
                //     })
                //     .style(resource_details_child())
                //     .width(Length::Fill)
                //     .center_y()
                // ]
                // .spacing(padding::PORTION)
                // .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH);

                let thermals = column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::Thermometer.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT)
                            .size(font_sizes::H2),
                        text(String::from("Thermals")).size(font_sizes::H2)
                    ]
                    .spacing(padding::MAIN),
                    container(column![column![
                        text(String::from("Temperature"))
                            .style(Text::Color(custom_theme::GREY_TEXT)),
                        text(String::from("25℃")),
                    ]
                    .padding(padding::MAIN)
                    .spacing(padding::PORTION),])
                    .style(resource_details_child())
                    .width(Length::Fill)
                    .center_y()
                ]
                .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH)
                .spacing(padding::PORTION);

                let about = column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::InfoCircle.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT)
                            .size(font_sizes::H2),
                        text(String::from("About")).size(font_sizes::H2)
                    ]
                    .spacing(padding::MAIN),
                    container(column![
                        column![
                            text(String::from("Speed")).style(Text::Color(custom_theme::GREY_TEXT)),
                            text(String::from("25℃")),
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                        container(row![])
                            .style(divider_background_1())
                            .width(Length::Fill)
                            .height(1),
                        column![
                            text(String::from("Slots used"))
                                .style(Text::Color(custom_theme::GREY_TEXT)),
                            text(String::from("25℃")),
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                        container(row![])
                            .style(divider_background_1())
                            .width(Length::Fill)
                            .height(1),
                        column![
                            text(String::from("RAM Type"))
                                .style(Text::Color(custom_theme::GREY_TEXT)),
                            text(String::from("25℃")),
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                        container(row![])
                            .style(divider_background_1())
                            .width(Length::Fill)
                            .height(1),
                        column![
                            text(String::from("Swapiness"))
                                .style(Text::Color(custom_theme::GREY_TEXT)),
                            text(String::from("N/A")),
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                    ])
                    .style(resource_details_child())
                    .width(Length::Fill)
                    .center_y()
                ]
                .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH)
                .spacing(padding::PORTION);

                // modify swapiness
                // other?
                let advanced = column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::Tools.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT)
                            .size(font_sizes::H2),
                        text(String::from("Advanced")).size(font_sizes::H2) // i in the top right that takes someone to a description of what Swap is
                    ]
                    .spacing(padding::MAIN),
                    container(column![
                        row![
                            text(String::from("Swapiness")),
                            horizontal_space(),
                            row![
                                text_input("current swapiness", "val"),
                                button("change").on_press(ResourceDetailsMessage::ChangeSwapiness)
                            ]
                            .spacing(padding::PORTION),
                        ]
                        .padding(padding::MAIN),
                        container(row![])
                            .style(divider_background_1())
                            .width(Length::Fill)
                            .height(1),
                        row![text(String::from("text"))],
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

                let content = column![header, scrollable(main)];

                let container = container(content);
                container.into()
            }
            ResourceType::Cpu => {
                let Some(cpu_details) = &self.cpu_details else {
                    return text("Waiting for tick").into();
                };

                let header = container(row!["CPU"])
                    .center_x()
                    .style(resource_details_header())
                    .width(Length::Fill)
                    .padding(padding::MAIN);

                let cpu_details_ui = column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::Cpu.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT)
                            .size(font_sizes::H2),
                        text(String::from("CPU")).size(font_sizes::H2),
                        horizontal_space(),
                        checkbox("logical cores", false)
                    ]
                    .spacing(padding::MAIN),
                    container(column![
                        row![
                            column![
                                text(String::from("Percent used"))
                                    .style(Text::Color(custom_theme::GREY_TEXT)),
                                text(format!("{:.1}%", cpu_details.cpu_usage_percent)),
                            ],
                            horizontal_space(),
                            column![
                                row![
                                    horizontal_space(),
                                    text(String::from("Frequency"))
                                        .style(Text::Color(custom_theme::GREY_TEXT))
                                ],
                                row![
                                    horizontal_space(),
                                    text(format!("{:.2}Hz", cpu_details.frequency))
                                ],
                            ]
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                        container(row![])
                            .style(divider_background_1())
                            .width(Length::Fill)
                            .height(1),
                        cpu_details.cpu_chart.view(),
                    ])
                    .style(resource_details_child())
                    .width(Length::Fill)
                    .center_y()
                ]
                .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH)
                .spacing(padding::PORTION);

                let thermals = column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::Thermometer.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT)
                            .size(font_sizes::H2),
                        text(String::from("Thermals")).size(font_sizes::H2)
                    ]
                    .spacing(padding::MAIN),
                    container(column![column![
                        text(String::from("Temperature"))
                            .style(Text::Color(custom_theme::GREY_TEXT)),
                        text(String::from("25℃")),
                    ]
                    .padding(padding::MAIN)
                    .spacing(padding::PORTION),])
                    .style(resource_details_child())
                    .width(Length::Fill)
                    .center_y()
                ]
                .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH)
                .spacing(padding::PORTION);

                let about = column![
                    row![
                        text(iced_aw::graphics::icons::BootstrapIcon::InfoCircle.to_string())
                            .font(iced_aw::BOOTSTRAP_FONT)
                            .size(font_sizes::H2),
                        text(String::from("About")).size(font_sizes::H2)
                    ]
                    .spacing(padding::MAIN),
                    container(column![
                        column![
                            text(String::from("Physical cores"))
                                .style(Text::Color(custom_theme::GREY_TEXT)),
                            text(format!("{}", cpu_details.physical_core_count)),
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                        container(row![])
                            .style(divider_background_1())
                            .width(Length::Fill)
                            .height(1),
                        column![
                            text(String::from("Logical cores"))
                                .style(Text::Color(custom_theme::GREY_TEXT)),
                            text(format!("{}", cpu_details.logical_core_count)),
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                        container(row![])
                            .style(divider_background_1())
                            .width(Length::Fill)
                            .height(1),
                        column![
                            text(String::from("Brand")).style(Text::Color(custom_theme::GREY_TEXT)),
                            text(format!("{}", cpu_details.brand)),
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                        container(row![])
                            .style(divider_background_1())
                            .width(Length::Fill)
                            .height(1),
                        column![
                            text(String::from("Frequency"))
                                .style(Text::Color(custom_theme::GREY_TEXT)),
                            text(format!("{}Hz", cpu_details.frequency)),
                        ]
                        .padding(padding::MAIN)
                        .spacing(padding::PORTION),
                    ])
                    .style(resource_details_child())
                    .width(Length::Fill)
                    .center_y()
                ]
                .max_width(sizings::MAX_MAIN_CONTENT_CHILDREN_WIDTH)
                .spacing(padding::PORTION);

                let main = container(
                    column![cpu_details_ui, thermals, about]
                        .spacing(20)
                        .align_items(alignment::Alignment::Center),
                )
                .center_x()
                .width(Length::Fill)
                .padding(padding::SECTION);

                let content = column![header, scrollable(main)];

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
