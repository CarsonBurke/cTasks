use iced::{widget::{column, container, text}, Element};

#[derive(Debug, Clone)]
pub enum ApplicationsDetailsMessage {}

#[derive(Debug, Default)]
pub struct ApplicationsDetails {}

impl ApplicationsDetails {
    pub fn new() -> Self {
        Self {}
    }

    pub fn on_tick(&mut self) {

    }

    pub fn view(&self) -> Element<ApplicationsDetailsMessage> {
        let content = column![text(String::from("apps"))];

        let container = container(content);
        container.into()
    }
}
