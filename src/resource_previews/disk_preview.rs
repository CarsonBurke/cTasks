use std::ffi::OsString;

use iced::{
    theme,
    widget::{button, column, container, progress_bar, row, text},
    Element, Length,
};
use iced_aw::BootstrapIcon;
use sysinfo::{Disk, DiskKind};

use crate::{
    constants::padding, general_widgets::icons::bootstrap_icon, preferences::Preferences, styles,
    utils::format_bytes, ActivePreview, DiskData, ResourceType,
};

use super::{
    preview_widgets::{preview_header, preview_metrics},
    resource_preview::{ResourcePreview, ResourcePreviewDisplayState, ResourcePreviewMessage},
};

#[derive(Debug)]
pub struct DiskPreview {
    pub resource: ResourceType,
    pub display_state: ResourcePreviewDisplayState,
}

impl Default for DiskPreview {
    fn default() -> Self {
        Self {
            resource: ResourceType::Disk,
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

    pub fn view(
        &self,
        preferences: &Preferences,
        active_preview: &ActivePreview,
        data: &DiskData,
    ) -> Element<ResourcePreviewMessage> {
        let content = column![
            preview_header(
                bootstrap_icon(BootstrapIcon::Hdd),
                text(format!(
                    "{} {}",
                    format_bytes(preferences, data.space_total as f32),
                    data.kind
                ))
            ),
            preview_metrics(vec![
                (
                    bootstrap_icon(BootstrapIcon::Eye),
                    text(format_bytes(preferences, data.read as f32)),
                ),
                (
                    bootstrap_icon(BootstrapIcon::Pen),
                    text(format_bytes(preferences, data.written as f32)),
                )
            ]),
            progress_bar(0.0..=1., data.space_used as f32 / data.space_total as f32)
                .height(5)
                .width(Length::Fill)
                .style(|_: &_| styles::progress_bar::primary_background_5())
        ]
        .spacing(padding::PORTION).padding(padding::PORTION);

        let button = button(content)
            .on_press(ResourcePreviewMessage::ResourceDetailsFor(ActivePreview {
                resource: self.resource,
                name: Some(data.name.clone()),
            }))
            .style(iced::theme::Button::Custom(Box::new(
                styles::button::Background3Blended {
                    display_as_pressed: active_preview.name.as_ref() == Some(&data.name)
                        && active_preview.resource == self.resource,
                },
            )));

        button.into()
    }
}

