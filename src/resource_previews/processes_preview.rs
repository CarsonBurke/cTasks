use iced::{
    alignment, theme,
    widget::{button, column, container, progress_bar, row, text},
    Element, Length,
};
use iced_aw::BootstrapIcon;
use sysinfo::{Disk, DiskKind};

use crate::{
    constants::{custom_theme, font_sizes, padding},
    general_widgets::icons::bootstrap_icon,
    preferences::Preferences,
    styles,
    types::resource_data::{ApplicationData, ApplicationsData, CpuData},
    utils::format_bytes,
    ActivePreview, ResourceType,
};

use super::{
    preview_widgets::{preview_header, preview_metrics},
    resource_preview::{ResourcePreview, ResourcePreviewDisplayState, ResourcePreviewMessage},
};

#[derive(Debug)]
pub struct ProcessesPreview {
    pub resource: ResourceType,
    pub display_state: ResourcePreviewDisplayState,
}

impl Default for ProcessesPreview {
    fn default() -> Self {
        Self {
            resource: ResourceType::Processes,
            display_state: ResourcePreviewDisplayState::Shown,
        }
    }
}

impl ProcessesPreview {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn view(
        &self,
        preferences: &Preferences,
        active_preview: &ActivePreview,
    ) -> Element<ResourcePreviewMessage> {
        let content = column![row![preview_header(
            bootstrap_icon(BootstrapIcon::PersonWorkspace),
            text("Processes").size(font_sizes::H2)
        ),]
        .spacing(padding::PORTION)
        .align_items(iced::Alignment::Center),]
        .spacing(padding::PORTION)
        .padding(padding::PORTION)
        .width(Length::Fill);

        let button = button(content)
            .on_press(ResourcePreviewMessage::ResourcePageFor(ActivePreview {
                resource: self.resource,
                name: None,
            }))
            .style(iced::theme::Button::Custom(Box::new(
                styles::button::Background3Blended {
                    display_as_pressed: active_preview.resource == self.resource,
                },
            )));

        button.into()
    }
}
