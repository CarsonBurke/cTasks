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
    preferences::Preferences,
    resource_details::resource_details::ResourceDetailsMessage,
    styles,
    utils::format_bytes,
    ActivePreview, DiskData, ResourceType,
};

use super::{
    preview_widgets::{preview_header, preview_metrics},
    resource_preview::{ResourcePreview, ResourcePreviewDisplayState, ResourcePreviewMessage},
};

#[derive(Debug)]
pub struct DiskPreview {
    pub resource: ResourceType,
    pub disk_name: String,
    pub disk_size: u64,
    pub disk_read: u64,
    pub disk_written: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub disk_kind: DiskKind,
    pub display_state: ResourcePreviewDisplayState,
}

impl Default for DiskPreview {
    fn default() -> Self {
        Self {
            resource: ResourceType::Disk,
            disk_kind: DiskKind::Unknown(0),
            disk_name: String::new(),
            disk_size: 0,
            disk_used: 0,
            disk_read: 0,
            disk_written: 0,
            disk_total: 0,
            display_state: ResourcePreviewDisplayState::Shown,
        }
    }
}

impl DiskPreview {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn on_tick(&mut self, disk_data: &DiskData) {
        self.disk_name = disk_data.name.clone()/* .to_str().unwrap_or("no name").to_string() */;
        self.disk_size = disk_data.space_total;
        self.disk_used = disk_data.space_used;
        self.disk_read = disk_data.read;
        self.disk_written = disk_data.written;
        self.disk_kind = disk_data.kind;
    }

    pub fn view(
        &self,
        preferences: &Preferences,
        active_preview: &ActivePreview,
    ) -> Element<ResourcePreviewMessage> {
        let content = column![
            preview_header(
                bootstrap_icon(BootstrapIcon::Hdd),
                text(format!(
                    "{} {}",
                    format_bytes(preferences, self.disk_size as f32),
                    self.disk_kind
                ))
            ),
            preview_metrics(vec![
                (
                    bootstrap_icon(BootstrapIcon::Eye),
                    text(format_bytes(preferences, self.disk_read as f32)),
                ),
                (
                    bootstrap_icon(BootstrapIcon::Pen),
                    text(format_bytes(preferences, self.disk_written as f32)),
                )
            ]),
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
                styles::button::Background3Blended {
                    display_as_pressed: active_preview.0 == self.disk_name
                        && active_preview.1 == self.resource,
                },
            )));

        button.into()
    }
}
