use std::ffi::OsString;

use iced::Element;

use crate::{ActivePreview, ResourceType};

#[derive(Debug, Clone)]
pub enum ResourcePreviewMessage {
    ResourceDetailsFor(ActivePreview),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePreviewDisplayState {
    Shown,
    Hidden,
    Active
}

pub trait ResourcePreview<Message, OnTickParams> {
    fn new() -> Self;
    // need a way to pass data
    fn on_tick(&mut self, params: OnTickParams);
    fn view(&self) -> Element<Message>;
}