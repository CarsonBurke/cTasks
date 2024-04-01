use iced::{
    theme,
    widget::{button, column, container, progress_bar, row, text},
    Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{custom_theme, font_sizes, padding},
    general_widgets::icons::bootstrap_icon,
    resource_details::resource_details::ResourceDetailsMessage,
    styles,
    utils::format_bytes,
    ResourceType,
};

use super::resource_preview::{ResourcePreview, ResourcePreviewMessage};

pub struct DiskPreviewOnTickParams {}

#[derive(Debug, Default)]
pub struct DiskPreview {
    resource: ResourceType,
    disk_id: usize,
    disk_name: String,
    disk_size: u64,
    disk_read: u64,
    disk_written: u64,
    disk_used: u64,
    disk_total: u64,
}

impl DiskPreview {
    pub fn new() -> Self {
        Self {
            resource: ResourceType::Disk,
            ..Default::default()
        }
    }

    fn on_tick(&mut self, params: DiskPreviewOnTickParams) {}

    pub fn view(&self) -> Element<ResourcePreviewMessage> {
        let content = column![
            row![
                bootstrap_icon(BootstrapIcon::Hdd).size(font_sizes::H2),
                text(format!(
                    "{} {}",
                    format_bytes(self.disk_size),
                    self.disk_name
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
                self.disk_id,
                self.resource,
            ))
            .style(iced::theme::Button::Custom(Box::new(
                styles::button::Background3Blended {},
            )));

        button.into()
    }
}
