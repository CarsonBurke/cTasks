use iced::{widget::{column, container, text}, Command, Element};

use crate::{preferences::{self, Preferences}, ApplicationData};

use super::chart::{ResourceChart, ResourceChartMessage};

#[derive(Debug, Clone)]
pub enum ApplicationsPageMessage {
    ResourceChartMessage(ResourceChartMessage),
}

#[derive(Debug)]
pub struct ApplicationsPage {
    written_chart: ResourceChart,
    read_chart: ResourceChart,
}

impl ApplicationsPage {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            written_chart: ResourceChart::new(preferences),
            read_chart: ResourceChart::new(preferences),
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
