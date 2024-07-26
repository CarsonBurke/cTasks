use std::{borrow::BorrowMut, collections::VecDeque, f64::consts::E};

use iced::{
    advanced::graphics::futures::backend::default,
    alignment,
    theme::{self, Text},
    widget::{
        self, button, checkbox, column, container, horizontal_space, keyed_column, row, scrollable,
        shader::wgpu::{hal::empty::Resource, naga::proc},
        text, text_input, vertical_space, Column, Themer,
    },
    window::Action,
    Alignment, Command, Element, Length, Theme,
};
use iced_aw::{grid, grid_row, BootstrapIcon, Grid, GridRow, Wrap};
use ordered_float::OrderedFloat;
use plotters_iced::{Chart, ChartWidget};
use sysinfo::{DiskKind, MemoryRefreshKind, Pid, Process, ProcessRefreshKind, RefreshKind, System};

use crate::{
    constants::{
        custom_theme, font_sizes, padding,
        sizings::{self, DEFAULT_CHART_HEIGHT},
        HISTORY_TICKS,
    },
    general_widgets::{
        icons::bootstrap_icon,
        section::{section, section_box, section_box_headless},
        seperators::seperator_background_1,
        split_table_double::split_table_double,
        split_table_single::split_table_single,
    },
    preferences::Preferences,
    styles::{
        self,
        container::{
            alternate_process_grid_row, divider_background_1, primary_process_grid_row,
            resource_details_child, resource_details_header,
        },
    },
    utils::{format_bytes, format_hz},
    ActivePreview, DiskData, ResourceData, ResourceHistory, ResourceType,
};

use super::{
    applications_page::{ApplicationsPage, ApplicationsPageMessage},
    chart::{ResourceChart, ResourceChartMessage},
    memory_page::{self, MemoryPage, MemoryPageMessage},
};

#[derive(Debug)]
pub struct DiskDetails {
    pub read_bytes: u64,
    pub written_bytes: u64,
    pub total_space: u64,
    pub total_used: u64,
    pub is_removable: bool,
    pub kind: DiskKind,
    pub written_chart: ResourceChart,
    pub read_chart: ResourceChart,
}

#[derive(Debug)]
pub struct CpuDetails {
    pub cpu_usage_percent: f32,
    pub physical_core_count: u32,
    pub logical_core_count: u32,
    pub cpu_chart: ResourceChart,
    pub brand: String,
    pub frequency: u64,
    pub logical_core_charts: Vec<ResourceChart>,
    pub logical_cores_usage_percents: Vec<f32>,
    pub logical_cores_frequencies: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceDetailsMessage {
    KillProcessId(Pid),
    SortByIndex(u32),
    SwitchSortDirection,
    ChangeSwapiness,
    ToggleLogicalCores(bool),
    ResourceChartMessage(ResourceChartMessage),
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

// pub type ResourceDetailsElements = MemoryDetails & ApplicationsDetails;

#[derive(Debug, Default)]
pub struct ResourceDetails {
    pub resource: ResourceType,
    preview_values: Option<u32>,
    memory_details: Option<MemoryPage>,
    processes_details: Option<ProcessesDetails>,
    cpu_details: Option<CpuDetails>,
    pub show_logical_cores: bool,
    disks_details: Vec<Option<DiskDetails>>,
    disks_index: usize,
}

impl ResourceDetails {
    pub fn new(preferences: &Preferences, resource: ResourceType) -> Self {
        let mut new_self = Self {
            ..Default::default()
        };
        new_self.apply_resource_type(resource, preferences);

        new_self
    }

    pub fn apply_resource_type(&mut self, resource: ResourceType, preferences: &Preferences) {
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
                self.memory_details = Some(MemoryPage {
                    ram_chart: ResourceChart::new(preferences),
                    swap_chart: ResourceChart::new(preferences),
                })
            }
            ResourceType::Cpu => {
                self.cpu_details = Some(CpuDetails {
                    cpu_usage_percent: 0.0,
                    physical_core_count: 0,
                    logical_core_count: 0,
                    brand: String::new(),
                    frequency: 0,
                    cpu_chart: ResourceChart::new(preferences),
                    logical_core_charts: Vec::new(),
                    logical_cores_usage_percents: Vec::new(),
                    logical_cores_frequencies: Vec::new(),
                })
            }
            ResourceType::Disk => {
                self.disks_details = vec![];
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
        logical_core_usage_percent: &Vec<f32>,
        logical_cores_frequencies: &Vec<u64>,
        resource_data: &ResourceData,
        preferences: &Preferences,
        active_preview: &ActivePreview,
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

                // RAM usage history

                memory_details.ram_chart.data_points = resource_history.ram.clone();

                // Swap usage history
                memory_details.swap_chart.data_points = resource_history.swap.clone();
            }
            ResourceType::Cpu => {
                let Some(cpu_details) = &mut self.cpu_details else {
                    return;
                };

                cpu_details.physical_core_count = physical_cpu_count;
                cpu_details.logical_core_count = logical_cpu_count;
                cpu_details.brand = cpu_brand;
                cpu_details.frequency = cpu_frequency;

                if self.show_logical_cores {
                    // Construct charts if they don't exist yet

                    if cpu_details.logical_core_charts.len() == 0 {
                        for _ in 0..logical_cpu_count {
                            cpu_details
                                .logical_core_charts
                                .push(ResourceChart::new(preferences));
                        }
                    }

                    // update chart data to match current core usage

                    for (i, chart) in cpu_details.logical_core_charts.iter_mut().enumerate() {
                        chart.data_points = resource_history.logical_cores[i].clone();
                    }

                    cpu_details.logical_cores_usage_percents = logical_core_usage_percent.clone();
                    cpu_details.logical_cores_frequencies = logical_cores_frequencies.clone();
                } else {
                    cpu_details.cpu_usage_percent = cpu_usage_percent;

                    // cpu usage history

                    cpu_details.cpu_chart.data_points = resource_history.cpu.clone();
                }
            }
            _ => {}
            
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
                ResourceDetailsMessage::ToggleLogicalCores(toggle_state) => {
                    println!("show logical cores: {}", toggle_state);
                    self.show_logical_cores = toggle_state;
                }
                _ => {}
            }
        })();

        Command::none()
    }

    pub fn view(&self, preferences: &Preferences) -> Element<ResourceDetailsMessage> {
        match &self.resource {
            _ => {
                let content = row![];
                container(content).into()
            }
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
                                button(text("Kill"))
                                    .on_press(ResourceDetailsMessage::KillProcessId(
                                        process_details.id
                                    ))
                                    .style(iced::theme::Button::Custom(Box::new(
                                        styles::button::Primary {},
                                    ))),
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

                let content = column![
                    header,
                    scrollable(main).style(iced::theme::Scrollable::Custom(Box::new(
                        styles::scrollable::Background1 {},
                    )))
                ];

                let container = container(content);
                container.into()
            }
            ResourceType::Gpu => {
                let content = row![];

                let container = container(content);
                container.into()
            }
            ResourceType::Disk => {
                let Some(Some(disk_details)) = self.disks_details.get(self.disks_index) else {
                    return text("Waiting for tick, or no disk data").into();
                };

                let header = container(row!["Disk name"])
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
                            container(disk_details.read_chart.view(None).map(move |message| {
                                ResourceDetailsMessage::ResourceChartMessage(message)
                            })),
                            seperator_background_1(),
                            split_table_single(vec![(
                                text("Reads".to_string()),
                                text(format_bytes(preferences, disk_details.read_bytes as f32))
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
                            container(disk_details.written_chart.view(None).map(move |message| {
                                ResourceDetailsMessage::ResourceChartMessage(message)
                            })),
                            seperator_background_1(),
                            split_table_single(vec![(
                                text("Writes".to_string()),
                                text(format_bytes(preferences, disk_details.written_bytes as f32))
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
                        split_table_double(vec![(
                            (
                                text("Usage".to_string()),
                                text(format!(
                                    "{:.2} / {:.2} GB",
                                    disk_details.total_used as f64 / 1024. / 1024. / 1024.,
                                    disk_details.total_space as f64 / 1024. / 1024. / 1024.
                                ))
                            ),
                            (
                                text("Percent used".to_string()),
                                text(format!(
                                    "{:.1}%",
                                    disk_details.total_used as f64
                                        / disk_details.total_space as f64
                                        * 100.
                                ))
                            )
                        )]),
                        split_table_single(vec![
                            (text(String::from("Brand")), text(String::from("25℃"))),
                            (
                                text(String::from("Kind")),
                                text(format!("{}", disk_details.kind))
                            ),
                            (
                                text(String::from("Is removable")),
                                text(format!("{}", disk_details.is_removable))
                            ),
                            (
                                text(String::from("RAM type")),
                                text(String::from("SODIMM?"))
                            ),
                            (text(String::from("Swapiness")), text(String::from("N/A"))),
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
            _ => {
                let content = row![];

                let container = container(content);
                container.into()
            }
        }
    }
}
