

use iced::{
    alignment, theme,
    widget::{button, column, container, row, scrollable, text},
    Command, Element, Length,
};
use iced_aw::{grid_row, BootstrapIcon, Grid, GridRow};
use sysinfo::{Pid, System};

use crate::{
    constants::padding,
    preferences::{self, Preferences},
    styles::{self, container::resource_details_header},
    types::resource_data::{ApplicationData, ProcessesData},
};

use super::{
    chart::{ResourceChart, ResourceChartMessage},
    resource_details::{ResourceDetailsMessage, SortDirection},
};

#[derive(Debug, Clone)]
pub enum ProcessesPageMessage {
    ResourceChartMessage(ResourceChartMessage),
    SwitchSortDirection,
    SortByIndex(u32),
    KillProcess(Pid),
}

#[derive(Debug)]
pub struct ProcessesPage {}

impl ProcessesPage {
    pub fn new(preferences: &Preferences) -> Self {
        Self {}
    }

    pub fn update(
        &mut self,
        message: ProcessesPageMessage,
        data: &mut ProcessesData,
        system_info: &System,
    ) -> Command<ProcessesPageMessage> {
        match message {
            ProcessesPageMessage::SortByIndex(sort_index) => {

                if data.sort_index == sort_index {
                    return Command::none()
                }

                data.sort_index = sort_index;
                // Also reset the sort direction since the user is sorting a different category
                data.sort_direction = SortDirection::default();

                data.sort_by_index();

                Command::none()
            }
            ProcessesPageMessage::SwitchSortDirection => {

                data.sort_direction = match data.sort_direction {
                    SortDirection::Descending => SortDirection::Ascending,
                    SortDirection::Ascending => SortDirection::Descending,
                };

                Command::none()
            }
            ProcessesPageMessage::KillProcess(pid) => {

                let Some(process) = system_info.process(pid) else {
                    return Command::none();
                };

                // The process still exists. Kill it

                process.kill();
                println!("Killed {}", process.name());

                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(
        &self,
        preferences: &Preferences,
        data: &ProcessesData,
    ) -> Element<ProcessesPageMessage> {
        let header = container(row!["Processes"])
            .center_x()
            .style(resource_details_header())
            .width(Length::Fill)
            .padding(padding::MAIN);

        let processes_header_strings =
            vec!["Name", "CPU", "Memory", "Disk Read", "Disk Written", "Kill"];

        /* let processes_headers = GridRow::with_elements({
            let mut elements = Vec::new();

            for (i, content) in processes_header_strings.iter().enumerate() {
                if i as u32 == data.sort_index {
                    elements.push(
                        button(
                            row![
                                text(content),
                                // Icon
                                text(String::from({
                                    match data.sort_direction {
                                        SortDirection::Descending => BootstrapIcon::CaretDownFill,
                                        SortDirection::Ascending => BootstrapIcon::CaretUpFill,
                                    }
                                }))
                                .font(iced_aw::BOOTSTRAP_FONT)
                            ]
                            .spacing(10),
                        )
                        .width(Length::Fill)
                        .on_press(ProcessesPageMessage::SwitchSortDirection)
                        .style(theme::Button::Text),
                    );

                    continue;
                }

                elements.push(
                    button(*content)
                        .width(Length::Fill)
                        .on_press(ProcessesPageMessage::SortByIndex(i as u32))
                        .style(theme::Button::Text),
                );
            }

            elements
        }); */

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
                /* rows.push(processes_headers); */
                rows.push(processes_totals);

                for (i, process_data) in (0_u32..).zip(data.in_depth.processes.iter()) {
                    let is_odd = i % 2 == 1;
                    // let styler = if is_odd {
                    //     alternate_process_grid_row()
                    // } else {c
                    //     primary_process_grid_row()
                    // };

                    rows.push(grid_row!(
                        text(process_data.name.to_string()),
                        text(format!["{:.2}%", process_data.cpu_usage]),
                        text(format![
                            "{:.2} MB",
                            process_data.memory_usage as f64 / 1024. / 1024.
                        ]),
                        text(format![
                            "{:.2} MB",
                            process_data.disk_read as f64 / 1024. / 1024.
                        ]),
                        text(format![
                            "{:.2} MB",
                            process_data.disk_written as f64 / 1024. / 1024.
                        ]),
                        button(text("Kill"))
                            .on_press(ProcessesPageMessage::KillProcess(process_data.pid))
                            .style(iced::theme::Button::Custom(Box::new(
                                styles::button::Primary {},
                            ))),
                    ));
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

        /* let content = column![];

        let container = container(content);
        container.into() */
    }
}
