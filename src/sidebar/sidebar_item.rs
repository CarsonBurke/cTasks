use iced::{
    widget::{column, container, progress_bar, row, text},
    Alignment, Element, Length,
};
use iced_aw::BootstrapIcon;
use sysinfo::{Disks, Networks, System};

use crate::{styles, utils::format_bytes, ResourceType};

#[derive(Debug, Default)]
pub struct SidebarItemParent {
    header: String,
    usage_percent: Option<f32>,
    pub resource: ResourceType,
    metric: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SidebarItemParentMessage {
    Tick,
}

impl SidebarItemParent {
    pub fn new(resource: ResourceType, header: String) -> Self {
        Self {
            resource,
            header,
            usage_percent: None,
            metric: None,
            ..Default::default()
        }
    }

    pub fn on_tick(
        &mut self,
        system_info: &System,
        cpu_usage_percent: f32,
        memory_usage_percent: f32,
        disk_info: &Disks,
        network_info: &Networks,
    ) {
        let (usage_percent, metric): (Option<f32>, Option<String>) = match self.resource {
            ResourceType::Applications => (None, None),
            ResourceType::Processes => {

                let processes = system_info.processes();

                (None, Some(processes.len().to_string()))
            }
            ResourceType::Cpu => {
                let usage_percent = cpu_usage_percent;
                self.usage_percent = Some(usage_percent);
                self.metric = Some(format!("{:.1}%", usage_percent));

                (Some(usage_percent), Some(format!("{:.1}%", usage_percent)))
            }
            ResourceType::Memory => {
                let usage_percent = memory_usage_percent;

                (Some(usage_percent), Some(format!("{:.1}%", usage_percent)))
            }
            ResourceType::Gpu => (None, None),
            ResourceType::Disk => {
                let mut total_read = 0;
                let mut total_written = 0;

                /* for disk in disk_info {
                    disk.
                    total_read += disk.name().
                }

                system_info. */

                // for disk in disk_info {
                //     total_read += disk.total_space();
                //     total_written += disk.available_space();
                // }

                for (pid, process) in system_info.processes() {
                    let disk_usage = process.disk_usage();

                    total_read += disk_usage.read_bytes;
                    total_written += disk_usage.written_bytes;
                }

                (
                    None,
                    Some(format!("{} {}", format_bytes(total_read), format_bytes(total_written))),
                )
            }
            ResourceType::Wifi => {
                let mut total_received = 0;
                let mut total_transmitted = 0;

                for (interface_name, data) in network_info {
                    total_received += data.received();
                    total_transmitted += data.transmitted();
                }

                (
                    None,
                    Some(format!("{:.1} {:.1}", total_received, total_transmitted)),
                )
            }
            ResourceType::Ethernet => (None, None),
        };

        self.usage_percent = usage_percent;
        self.metric = metric;
    }

    pub fn view(&self, i: usize) -> Element<SidebarItemParentMessage> {
        match self.resource {
            ResourceType::Applications => String::from(BootstrapIcon::WindowStack),
            ResourceType::Processes => String::from(BootstrapIcon::PersonWorkspace),
            ResourceType::Cpu => String::from(BootstrapIcon::Cpu),
            ResourceType::Memory => String::from(BootstrapIcon::Memory),
            ResourceType::Gpu => String::from(BootstrapIcon::GpuCard),
            ResourceType::Disk => String::from(BootstrapIcon::Hdd),
            ResourceType::Wifi => String::from(BootstrapIcon::Wifi),
            ResourceType::Ethernet => String::from(BootstrapIcon::DiagramTwo),
        };

        let icon_text = match self.resource {
            ResourceType::Applications => String::from(BootstrapIcon::WindowStack),
            ResourceType::Processes => String::from(BootstrapIcon::PersonWorkspace),
            ResourceType::Cpu => String::from(BootstrapIcon::Cpu),
            ResourceType::Memory => String::from(BootstrapIcon::Memory),
            ResourceType::Gpu => String::from(BootstrapIcon::GpuCard),
            ResourceType::Disk => String::from(BootstrapIcon::Hdd),
            ResourceType::Wifi => String::from(BootstrapIcon::Wifi),
            ResourceType::Ethernet => String::from(BootstrapIcon::DiagramTwo),
        };

        let preview_state = {
            if let Some(usage_percent) = self.usage_percent {
                row![progress_bar(0.0..=100.0, usage_percent)
                    .height(5)
                    .width(Length::Fill)
                    .style(|_: &_| styles::progress_bar::primary_background_5())]
            } else {
                row![/* text(String::from("No bar")) */]
            }
        };

        let container = container(
            column![
                row![
                    text(icon_text).font(iced_aw::BOOTSTRAP_FONT),
                    text(self.header.clone()),
                    text(self.metric.clone().unwrap_or("".to_string())).size(10),
                    // {
                    //     if let Some(metric) = &self.metric {
                    //         return text(metric).size(10).into();
                    //     }

                    //     text("".to_string()).size(10)
                    // }
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                preview_state
            ]
            .spacing(5),
        );
        container.into()
    }
}