use iced::{
    alignment, theme,
    widget::{button, column, container, progress_bar, row, text},
    Element, Length,
};
use iced_aw::BootstrapIcon;
use sysinfo::{Disk, DiskKind};

use crate::{
    constants::{custom_theme, font_sizes, padding},
    general_widgets::icons::{battery_icon, bootstrap_icon},
    preferences::Preferences,
    styles,
    types::resource_data::{BatteryData, CpuData},
    utils::format_bytes,
    ActivePreview, ResourceType,
};

use super::{
    preview_widgets::{preview_header, preview_metrics},
    resource_preview::{ResourcePreview, ResourcePreviewDisplayState, ResourcePreviewMessage},
};

#[derive(Debug)]
pub struct BatteryPreview {
    pub resource: ResourceType,
    pub display_state: ResourcePreviewDisplayState,
}

impl Default for BatteryPreview {
    fn default() -> Self {
        Self {
            resource: ResourceType::Battery,
            display_state: ResourcePreviewDisplayState::Shown,
        }
    }
}

impl BatteryPreview {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn view(
        &self,
        preferences: &Preferences,
        active_preview: &ActivePreview,
        data: &BatteryData,
    ) -> Element<ResourcePreviewMessage> {
        let content = column![
            row![
                preview_header(
                    battery_icon(data.state),
                    text("Battery").size(font_sizes::H2)
                ),
                text(format!(/* "{:.1}%",  */"{:.0?}%", data.state_of_charge * 100.))
                    .style(theme::Text::Color(custom_theme::GREY_TEXT))
                    .size(font_sizes::P),
            ]
            .spacing(padding::PORTION)
            .align_items(iced::Alignment::Center),
            progress_bar(0.0..=100., (data.state_of_charge * 100.).into())
                .height(5)
                .width(Length::Fill)
                .style(|_: &_| styles::progress_bar::primary_background_5())
        ]
        .spacing(padding::PORTION)
        .padding(padding::PORTION);

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
