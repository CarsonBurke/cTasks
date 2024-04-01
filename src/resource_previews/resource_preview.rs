use iced::Element;

use super::disk_preview::DiskPreviewMessage;

#[derive(Debug, Clone)]
pub enum ResourcePreviewMessage {
    DiskPreviewMessage(DiskPreviewMessage),
}

pub trait ResourcePreview<Message, OnTickParams> {
    fn new() -> Self;
    // need a way to pass data
    fn on_tick(&mut self, params: OnTickParams);
    fn view(&self) -> Element<Message>;
}