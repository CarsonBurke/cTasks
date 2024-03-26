use std::collections::HashMap;

use iced::{Element, Length};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use super::resource_details::ResourceDetailsMessage;

struct ResourceChartMessage {}

#[derive(Debug, Default)]
pub struct ResourceChart {
    /// Tick, data point => x, y
    data: HashMap<u32, f32>,
}

impl Chart<ResourceDetailsMessage> for ResourceChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        //build your chart here, please refer to plotters for more details

        let mut chart = builder
            .x_label_area_size(0)
            .y_label_area_size(28)
            .margin(20).build_cartesian_2d(0..100, 0..1).expect("failed to build chart");
    }
}

impl ResourceChart {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn view(&self) -> Element<ResourceDetailsMessage> {
        ChartWidget::new(self)
            .width(Length::Fixed(200.))
            .height(Length::Fixed(200.))
            .into()
    }
}
