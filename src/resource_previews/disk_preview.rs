use std::ffi::OsString;

use iced::{
    theme,
    widget::{button, column, container, progress_bar, row, text},
    Element, Length,
};
use iced_aw::BootstrapIcon;
use sysinfo::{Disk, DiskKind};

use crate::{
    constants::{custom_theme, font_sizes, padding},
    general_widgets::icons::bootstrap_icon,
    resource_details::resource_details::ResourceDetailsMessage,
    styles,
    utils::format_bytes,
    ResourceType,
};

use super::resource_preview::{ResourcePreview, ResourcePreviewMessage};

#[derive(Debug)]
pub struct DiskPreview {
    pub resource: ResourceType,
    pub disk_name: OsString,
    pub disk_size: u64,
    pub disk_read: u64,
    pub disk_written: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub disk_kind: DiskKind,
}

impl Default for DiskPreview {
    fn default() -> Self {
        Self {
            resource: ResourceType::Disk,
            disk_kind: DiskKind::Unknown(0),
            disk_name: OsString::new(),
            disk_size: 0,
            disk_used: 0,
            disk_read: 0,
            disk_written: 0,
            disk_total: 0,
        }
    }
}

impl DiskPreview {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn on_tick(&mut self, disk: &Disk) {

        self.resource = ResourceType::Disk;
        self.disk_name = disk.name().into();
        self.disk_size = disk.total_space();
        self.disk_used = self.disk_size - disk.available_space();
        self.disk_read = 0;
        self.disk_written = 0;
        self.disk_kind = disk.kind();
    }

    pub fn view(&self) -> Element<ResourcePreviewMessage> {
        let content = column![
            row![
                bootstrap_icon(BootstrapIcon::Hdd).size(font_sizes::H2),
                text(format!(
                    "{} {}",
                    format_bytes(self.disk_size),
                    self.disk_kind
                ))
                .size(font_sizes::H2),
            ]
            .spacing(padding::PORTION),
            row![
                row![
                    bootstrap_icon(BootstrapIcon::Eye)
                        .style(theme::Text::Color(custom_theme::GREY_TEXT))
                        .size(font_sizes::H4),
                    text(format_bytes(self.disk_read))
                        .style(theme::Text::Color(custom_theme::GREY_TEXT))
                        .size(font_sizes::H4)
                ]
                .spacing(padding::PORTION),
                row![
                    bootstrap_icon(BootstrapIcon::Pen)
                        .style(theme::Text::Color(custom_theme::GREY_TEXT))
                        .size(font_sizes::H4),
                    text(format_bytes(self.disk_written))
                        .style(theme::Text::Color(custom_theme::GREY_TEXT))
                        .size(font_sizes::H4)
                ]
                .spacing(padding::PORTION)
            ]
            .spacing(padding::MAIN),
            progress_bar(0.0..=100.0, self.disk_used as f32 / self.disk_total as f32)
                .height(5)
                .width(Length::Fill)
                .style(|_: &_| styles::progress_bar::primary_background_5())
        ]
        .spacing(padding::PORTION);

        let button = button(content)
            .on_press(ResourcePreviewMessage::ResourceDetailsFor(
                self.disk_name.clone(),
                self.resource,
            ))
            .style(iced::theme::Button::Custom(Box::new(
                styles::button::Background3Blended {},
            )));

        button.into()
    }
}
