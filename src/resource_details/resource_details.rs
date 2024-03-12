use iced::{
    advanced::graphics::futures::backend::default, widget::shader::wgpu::hal::empty::Resource,
    Element,
};

use crate::ResourceType;

use super::memory_detail::{MemoryDetails, MemoryDetailsMessage};

#[derive(Debug, Clone)]
pub enum ResourceDetailsMessage {
    MemoryDetailsMessage(MemoryDetailsMessage),
}

pub type ResourceDetailsElements = MemoryDetails;

#[derive(Debug, Default)]
pub struct ResourceDetails {
    resource: ResourceType,
    resource_details: ResourceDetailsElements,
}

impl ResourceDetails {
    pub fn new(resource: ResourceType) -> Self {
        let resource_details = match resource {
            ResourceType::Applications => MemoryDetails::new(),
            ResourceType::Processes => MemoryDetails::new(),
            ResourceType::Memory => MemoryDetails::new(),
            ResourceType::Cpu => MemoryDetails::new(),
            ResourceType::Gpu => MemoryDetails::new(),
            ResourceType::Disk => MemoryDetails::new(),
            ResourceType::Wifi => MemoryDetails::new(),
            ResourceType::Ethernet => MemoryDetails::new(),
        };

        Self {
            resource,
            resource_details,
        }
    }

    pub fn on_tick(&mut self) {
        self.resource_details.on_tick();
    }

    pub fn view(&self) -> Element<ResourceDetailsMessage> {
        match &self.resource {
            ResourceType::Applications => self
                .resource_details
                .view()
                .map(move |message| ResourceDetailsMessage::MemoryDetailsMessage(message)),
            ResourceType::Processes => self
                .resource_details
                .view()
                .map(move |message| ResourceDetailsMessage::MemoryDetailsMessage(message)),
            ResourceType::Memory => self
                .resource_details
                .view()
                .map(move |message| ResourceDetailsMessage::MemoryDetailsMessage(message)),
            ResourceType::Cpu => self
                .resource_details
                .view()
                .map(move |message| ResourceDetailsMessage::MemoryDetailsMessage(message)),
            ResourceType::Gpu => self
                .resource_details
                .view()
                .map(move |message| ResourceDetailsMessage::MemoryDetailsMessage(message)),
            ResourceType::Disk => self
                .resource_details
                .view()
                .map(move |message| ResourceDetailsMessage::MemoryDetailsMessage(message)),
            ResourceType::Wifi => self
                .resource_details
                .view()
                .map(move |message| ResourceDetailsMessage::MemoryDetailsMessage(message)),
            ResourceType::Ethernet => self
                .resource_details
                .view()
                .map(move |message| ResourceDetailsMessage::MemoryDetailsMessage(message)),
        }
    }
}
