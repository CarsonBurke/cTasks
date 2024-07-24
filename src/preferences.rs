use crate::{constants::DisplayState, resource_pages::resource_details::SortDirection};

#[derive(Debug, Copy, Clone)]
pub enum ByteBase {
    Decimal,
    Binary,
}

#[derive(Debug, Copy, Clone)]
pub struct Preferences {
    display_state: DisplayState,
    percent_precision: u8,
    history_ticks: i32,
    pub chart_y_axis_labels: bool,
    pub chart_y_axis_major_grid_lines: usize,
    pub byte_base: ByteBase,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            percent_precision: 1,
            history_ticks: 30,
            display_state: DisplayState::Shown,
            chart_y_axis_labels: false,
            chart_y_axis_major_grid_lines: 6,
            byte_base: ByteBase::Binary,
        }
    }
}

pub enum PreferencesMessage {
    DisplayState(DisplayState),
}

impl Preferences {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: PreferencesMessage) {
        match message {
            PreferencesMessage::DisplayState(display_state) => {
                self.display_state = display_state;
            }
        }
    }

    // fn view(&self) -> Element<PreferencesMessage> {
    // if self.display_state == DisplayState::Hidden {
    //     return Column::new().into();
    // }
    // let content = column![
    //     text(String::from("Preferences")),
    //     text_input("tick interval".to_string(), "value".to_string())
    // ]
    // .spacing(10);

    // let container = container(
    //     FloatingElement::new(content)
    //         .anchor(Anchor::SouthEast)
    //         .offset(20.0)
    //         .hide(false),
    // );
    // container.into()
    // }
}
