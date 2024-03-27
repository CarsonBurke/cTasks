use std::collections::{HashMap, VecDeque};

use iced::{Element, Length};
use plotters::{
    series::AreaSeries,
    style::{Color, FontTransform, HSLColor, IntoFont, ShapeStyle},
};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use super::resource_details::ResourceDetailsMessage;

struct ResourceChartMessage {}

#[derive(Debug, Default)]
pub struct ResourceChart {
    // 31 ticks of data
    pub data_points: VecDeque<(i32, i32)>,
}

impl Chart<ResourceDetailsMessage> for ResourceChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        //build your chart here, please refer to plotters for more details

        let mut chart = builder
            .x_label_area_size(0)
            .y_label_area_size(28)
            .margin(20)
            .build_cartesian_2d(0..30, 0..100)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .bold_line_style(plotters::style::colors::full_palette::GREY_600)
            .light_line_style(plotters::style::colors::full_palette::GREY_800)
            .axis_style(
                ShapeStyle::from(plotters::style::colors::full_palette::GREY_500).stroke_width(1),
            )
            .y_max_light_lines(2)
            .y_labels(6)
            .y_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&plotters::style::colors::WHITE)
                    .transform(FontTransform::Rotate90),
            )
            .x_max_light_lines(30)
            .x_labels(3)
            // .y_label_formatter(&&|y| format!("{}%", y))
            .draw()
            .expect("failed to draw chart mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data_points.iter().map(|x| (x.0, x.1)),
                    // self.data.iter(),
                    0,
                    HSLColor {
                        0: 240. / 360.,
                        1: 0.7,
                        2: 0.651,
                    },
                )
                .border_style(ShapeStyle::from(plotters::style::colors::BLUE).stroke_width(2)),
            )
            .expect("failed to draw chart data");
    }
}

impl ResourceChart {
    pub fn new() -> Self {
        let mut data_points = VecDeque::new();
        // data_points.push_back((0, 0));
        // data_points.push_back((1, 5));
        // data_points.push_back((2, 25));
        // data_points.push_back((3, 30));
        // data_points.push_back((4, 10));
        // data_points.push_back((5, 5));
        // data_points.push_back((6, 12));
        // data_points.push_back((7, 15));
        // data_points.push_back((8, 20));
        // data_points.push_back((9, 10));
        // data_points.push_back((10, 5));
        // data_points.push_back((11, 16));
        // data_points.push_back((12, 10));
        // data_points.push_back((13, 18));
        // data_points.push_back((14, 22));
        // data_points.push_back((15, 28));
        // data_points.push_back((16, 20));
        // data_points.push_back((17, 14));
        // data_points.push_back((18, 12));
        // data_points.push_back((19, 26));
        // data_points.push_back((20, 24));
        // data_points.push_back((21, 20));
        // data_points.push_back((22, 18));
        // data_points.push_back((23, 26));
        // data_points.push_back((24, 14));
        // data_points.push_back((25, 22));
        // data_points.push_back((26, 30));
        // data_points.push_back((27, 10));
        // data_points.push_back((28, 28));
        // data_points.push_back((29, 16));
        // data_points.push_back((30, 16));

        Self { data_points }
    }

    pub fn set_data() {

    }

    pub fn view(&self) -> Element<ResourceDetailsMessage> {
        // 3:2 ratio
        ChartWidget::new(self)
            // .width(Length::Fixed(600.))
            .height(Length::Fixed(300.))
            .into()
    }
}
