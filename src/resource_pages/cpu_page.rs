use iced::{widget::{column, container, text}, Command, Element};

use crate::{preferences::Preferences, CpuData};

use super::chart::{ResourceChart, ResourceChartMessage};


#[derive(Debug, Clone)]
pub enum CpuPageMessage {
    ResourceChartMessage(ResourceChartMessage),
}


#[derive(Debug)]
pub struct CpuPage {
    written_chart: ResourceChart,
    read_chart: ResourceChart,
}

impl CpuPage {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            written_chart: ResourceChart::new(preferences),
            read_chart: ResourceChart::new(preferences),
        }
    }

    pub fn update(&mut self, message: CpuPageMessage) -> Command<CpuPageMessage> {
        match message {
            _ => Command::none(),
        }
    }

    pub fn view(&self, preferences: &Preferences, data: &CpuData) -> Element<CpuPageMessage> {
        let content = column![text(String::from("cpu"))];
        let container = container(content);
        container.into()
    }
}