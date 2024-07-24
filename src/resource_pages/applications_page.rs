use iced::{widget::{column, container, text}, Command, Element};

use crate::{preferences::{self, Preferences}, types::resource_data::ApplicationData};

use super::chart::{ResourceChart, ResourceChartMessage};

#[derive(Debug, Clone)]
pub enum ApplicationsPageMessage {
    ResourceChartMessage(ResourceChartMessage),
}

#[derive(Debug)]
pub struct ApplicationsPage {
    
}

impl ApplicationsPage {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            
        }
    }

    pub fn update(&mut self, message: ApplicationsPageMessage) -> Command<ApplicationsPageMessage> {
        match message {
            _ => Command::none(),
        }
    }

    pub fn view(&self, preferences: &Preferences, data: &ApplicationData) -> Element<ApplicationsPageMessage> {
        let content = column![text(String::from("apps"))];

        let container = container(content);
        container.into()
    }
}
