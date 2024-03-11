use std::time::Duration;

use constants::{padding, DisplayState};
use iced::{
    advanced::{
        graphics::{futures::backend::default, text::cosmic_text::SwashImage},
        widget::Text,
    },
    alignment, executor, font, theme,
    widget::{
        column, container, horizontal_space, keyed_column, progress_bar, row, scrollable,
        shader::wgpu::hal::empty::Resource, text, text_input, Column, Row, Space,
    },
    window::{icon, Icon},
    Alignment, Application, Command, Element, Font, Length, Pixels, Renderer, Sandbox, Settings,
    Subscription, Theme,
};
use iced_aw::{
    floating_element::{self, Anchor},
    graphics::icons::{self, icon_to_char},
    native::Split,
    split, BootstrapIcon, FloatingElement, NerdIcon, Spinner, NERD_FONT,
};

mod constants;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SidebarItemParentMessage(usize, SidebarItemParentMessage),
    Tick,
}

#[derive(Debug, Default)]
enum AppState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug, Default)]
struct App {
    sidebar_items: Vec<SidebarItemParent>,
    preferences: Preferences,
    tick_interval: u64,
    state: AppState,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                tick_interval: 1000,
                state: AppState::Loading,
                preferences: Preferences::new(),
                ..Default::default()
            },
            Command::batch(vec![
                // font::load(iced_aw::NERD_FONT_BYTES).map(Message::FontLoaded),
                // Command::perform(load(), Message::Loaded),
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("C Tasks")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self.state {
            AppState::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    println!("loading success");

                    self.sidebar_items = vec![
                        SidebarItemParent::new(ResourceType::Applications, String::from("Hello")),
                        SidebarItemParent::new(ResourceType::Processes, String::from("Processes")),
                        SidebarItemParent::new(ResourceType::Memory, String::from("Goodbye")),
                        SidebarItemParent::new(ResourceType::Cpu, String::from("Cpu")),
                        SidebarItemParent::new(ResourceType::Gpu, String::from("Gpu")),
                        SidebarItemParent::new(ResourceType::Disk, String::from("Disk")),
                        SidebarItemParent::new(ResourceType::Wifi, String::from("Wifi")),
                        SidebarItemParent::new(ResourceType::Ethernet, String::from("Ethernet")),
                    ];
                    self.state = AppState::Loaded;
                }
                Message::Loaded(Err(_)) => {
                    println!("loading failure");
                }
                _ => {}
            },
            AppState::Loaded => {
                println!("loaded");

                // state.sidebar_items = vec![
                //     SidebarItemParent::new(String::from("Hello")),
                //     SidebarItemParent::new(String::from("Goodbye")),
                // ];
            }
        }

        match message {
            Message::Tick => {
                println!("tick");
                for element in self.sidebar_items.iter_mut() {
                    element.on_tick();
                }
            }
            _ => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(self.tick_interval))
            .map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        match self.state {
            AppState::Loading => {
                let spinner = Spinner::new();
                let loading = column![spinner];

                container(loading)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            AppState::Loaded => {
                let floating_content = container(
                    column![
                        text(String::from("Preferences")),
                        text_input("tick interval", "value")
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                )
                .width(Length::FillPortion(2))
                .center_x()
                .center_y()
                .style(theme::Container::Box)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center);

                let sidebar_header = row![
                    // text(iced_aw::graphics::icons::BootstrapIcon::List.to_string())
                    //     .font(iced_aw::BOOTSTRAP_FONT),
                    // text(String::from("C Tasks")),
                    // text(iced_aw::graphics::icons::BootstrapIcon::LayoutSidebar.to_string())
                    //     .font(iced_aw::BOOTSTRAP_FONT),
                    horizontal_space(),
                    text(String::from("C Tasks")),
                    horizontal_space(),
                    text(iced_aw::graphics::icons::BootstrapIcon::List.to_string())
                        .font(iced_aw::BOOTSTRAP_FONT),
                ]
                .spacing(10);

                let sidebar_content: Element<_> = {
                    keyed_column(self.sidebar_items.iter().enumerate().map(|(i, element)| {
                        (
                            i,
                            element
                                .view(i)
                                .map(move |message| Message::SidebarItemParentMessage(i, message)),
                        )
                    }))
                    .spacing(10)
                    .into()
                };

                // let content: Element<_> = keyed_column().into();

                let sidebar = container(column![sidebar_header, sidebar_content,].spacing(20))
                    .style(theme::Container::Box)
                    .height(Length::Fill)
                    .padding(padding::MAIN)
                    .width(Length::Shrink)
                    .max_width(200);

                let header = container(row![
                    horizontal_space(),
                    text(String::from("Switcher"))
                        .size(20)
                        .font(Font::MONOSPACE),
                    horizontal_space(),
                    text(iced_aw::graphics::icons::BootstrapIcon::Dash.to_string())
                        .font(iced_aw::BOOTSTRAP_FONT),
                    text(iced_aw::graphics::icons::BootstrapIcon::X.to_string())
                        .font(iced_aw::BOOTSTRAP_FONT),
                ])
                .width(Length::Fill)
                .style(|theme: &Theme| {
                    let palette = theme.extended_palette();

                    container::Appearance::default().with_border(palette.background.strong.color, 1)
                })
                .padding(padding::MAIN);

                let footer = container(row![text(String::from("Footer Text"))])
                    .style(theme::Container::Box)
                    .center_y()
                    .center_x()
                    .width(Length::Fill)
                    .padding(padding::MAIN);

                let main = container(scrollable(column![text("hello".to_string())].spacing(10)))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(padding::MAIN);

                let left = sidebar;
                let right = column![header, main, footer].width(Length::FillPortion(3));

                let container = container(
                    FloatingElement::new(row![left, right], floating_content)
                        .anchor(Anchor::SouthEast)
                        .hide(false),
                );
                container.into()
            }
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, Default)]
enum ResourceType {
    #[default]
    Applications,
    Processes,
    Cpu,
    Memory,
    Gpu,
    Disk,
    Wifi,
    Ethernet,
}

#[derive(Debug, Default)]
enum SidebarItemParentState {
    #[default]
    Shown,
    Hidden,
}

#[derive(Debug, Default)]
pub struct SidebarItemParent {
    header: String,
    metric: String,
    usage_percent: f32,
    state: SidebarItemParentState,
    resource: ResourceType,
}

#[derive(Debug, Clone)]
pub enum SidebarItemParentMessage {
    Tick,
}

impl SidebarItemParent {
    fn new(resource: ResourceType, header: String) -> Self {
        Self {
            resource,
            header,
            usage_percent: 50.0,
            ..Default::default()
        }
    }

    fn on_tick(&mut self) {
        self.usage_percent = rand::random::<f32>() * 100.;
    }

    fn view(&self, i: usize) -> Element<SidebarItemParentMessage> {
        let icon_text = match self.resource {
            ResourceType::Applications => String::from(BootstrapIcon::WindowStack),
            ResourceType::Processes => String::from(BootstrapIcon::PersonWorkspace),
            ResourceType::Cpu => String::from(BootstrapIcon::Cpu),
            ResourceType::Memory => String::from(BootstrapIcon::Memory),
            ResourceType::Gpu => String::from(BootstrapIcon::GpuCard),
            ResourceType::Disk => String::from(BootstrapIcon::Hdd),
            ResourceType::Wifi => String::from(BootstrapIcon::Wifi),
            ResourceType::Ethernet => String::from(BootstrapIcon::DiagramTwo),
        };

        let container = container(
            column![
                row![
                    text(icon_text).font(iced_aw::BOOTSTRAP_FONT),
                    text(self.header.clone()),
                    text(String::from("metric")).size(10)
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                row![progress_bar(0.0..=100.0, self.usage_percent)
                    .height(5)
                    .width(Length::Fill)]
            ]
            .spacing(5),
        );
        container.into()
    }
}

#[derive(Debug, Default)]
pub struct Preferences {
    display_state: DisplayState,
}

pub enum PreferencesMessage {
    DisplayState(DisplayState),
}

impl Preferences {
    fn new() -> Self {
        Self {
            display_state: DisplayState::Shown,
            ..Default::default()
        }
    }

    // fn view(&self) -> Element<PreferencesMessage> {
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

pub struct ResourceGraph;

impl ResourceGraph {}
