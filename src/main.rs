use std::{env, time::Duration};

use constants::{padding, DisplayState, PERCENT_PRECISION};
use iced::{
    advanced::{
        graphics::{futures::backend::default, text::cosmic_text::SwashImage},
        widget::Text,
        Application,
    },
    alignment, color, executor, font, theme,
    widget::{
        button, column, container, horizontal_space, keyed_column, progress_bar, row,
        scrollable::{self, Direction, Properties, RelativeOffset},
        shader::wgpu::{hal::empty::Resource, naga::proc},
        text, text_input, Column, Container, Row, Scrollable, Space,
    },
    window::{icon, Icon},
    Alignment, Color, Command, Element, Font, Length, Pixels, Renderer, Settings, Subscription,
    Theme,
};
use iced_aw::{
    core::icons,
    floating_element::{self, Anchor},
    spinner, split, FloatingElement, Spinner, NERD_FONT,
};
use resource_details::resource_details::{ResourceDetails, ResourceDetailsMessage};
use styles::container::{main_content, sidebar};
use sysinfo::{
    Cpu, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, ProcessRefreshKind, RefreshKind,
    System, UpdateKind,
};

mod constants;
mod resource_details;
mod resource_previews;
mod styles;

pub fn main() -> iced::Result {
    // env::set_var("RUST_BACKTRACE", "1");
    App::run(Settings::default())
}

pub enum CustomTheme {
    Light,
    Dark,
}

impl CustomTheme {
    pub fn from_system() -> Self {
        CustomTheme::Dark
    }
}

#[derive(Debug, Default, Clone)]
// Consider a vec format instead, due to performance in displaying graphs (presumably they take in an array of values)
// However a vec format will probably have to handle skipped ticks by reformatting the vector, which might ruin any performance benefits
pub struct ResourceHistoryTick {
    pub tick: u64,
    pub cpu_usage_percent: f32,
    pub cpu_cores_usage_percent: f32,
    pub gpu_usage_percent: f32,
    pub vram_usage: f32,
    pub memory_usage_percent: f32,
    pub disk_write: f32,
    pub disk_read: f32,
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SidebarItemParentMessage(usize, SidebarItemParentMessage),
    ResourceDetailsMessage(ResourceDetailsMessage),
    SetResourceDetails(ResourceType),
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
    main_content: ResourceDetails,
    tick_interval: u64,
    system_info: System,
    cpu_count: u32,
    disk_info: Disks,
    network_info: Networks,
    state: AppState,
    tick: u64,
    history: Vec<ResourceHistoryTick>,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    type Renderer = Renderer;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let system_info = System::new_all();
        let cpu_count = system_info.physical_core_count().unwrap_or(1) as u32/* system_info.cpus().len() as u32 */;

        (
            Self {
                tick_interval: 1000,
                state: AppState::Loading,
                preferences: Preferences::new(),
                system_info,
                cpu_count,
                tick: 0,
                history: Vec::new(),
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
                        SidebarItemParent::new(
                            ResourceType::Applications,
                            String::from("Applications"),
                        ),
                        SidebarItemParent::new(ResourceType::Processes, String::from("Processes")),
                        SidebarItemParent::new(ResourceType::Cpu, String::from("Cpu")),
                        SidebarItemParent::new(ResourceType::Memory, String::from("Memory")),
                        SidebarItemParent::new(ResourceType::Gpu, String::from("Gpu")),
                        SidebarItemParent::new(ResourceType::Disk, String::from("Disk")),
                        SidebarItemParent::new(ResourceType::Wifi, String::from("Wifi")),
                        SidebarItemParent::new(ResourceType::Ethernet, String::from("Ethernet")),
                    ];
                    self.state = AppState::Loaded;

                    self.main_content = ResourceDetails::new(ResourceType::Processes);
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

                match message {
                    Message::Tick => {
                        self.tick += 1;

                        // Change this to call specific to be more optimal

                        self.system_info
                            .refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
                        self.system_info.refresh_processes_specifics(
                            ProcessRefreshKind::new().with_user(UpdateKind::Always),
                        );
                        self.system_info
                            .refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
                        self.disk_info = Disks::new_with_refreshed_list();
                        self.network_info = Networks::new_with_refreshed_list();

                        println!("tick: {}", self.tick);
                        for element in self.sidebar_items.iter_mut() {
                            element.on_tick(&self.system_info, &self.disk_info, &self.network_info);
                        }

                        self.main_content
                            .on_tick(&mut self.system_info, self.cpu_count);
                    }
                    Message::ResourceDetailsMessage(resource_details_message) => {
                        let _ = self
                            .main_content
                            .update(resource_details_message)
                            .map(Message::ResourceDetailsMessage);
                    }
                    Message::SetResourceDetails(resource) => {
                        self.main_content.resource = resource.clone();
                    }
                    _ => {}
                }
            }
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
                let loading = column![/* spinner */];

                container(loading)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            AppState::Loaded => {
                /*let floating_content = container(
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
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center); */

                let sidebar_header = row![
                    // text(icons::Bootstrap::List.to_string())
                    //     .font(iced_aw::BOOTSTRAP_FONT),
                    // text(String::from("C Tasks")),
                    // text(icons::Bootstrap::LayoutSidebar.to_string())
                    //     .font(iced_aw::BOOTSTRAP_FONT),
                    horizontal_space(),
                    text(String::from("C Tasks")),
                    horizontal_space(),
                    text(icons::Bootstrap::List.to_string()).font(Font {
                        family: font::Family::Name("bootstrap-icons"),
                        ..Default::default()
                    }),
                ]
                .spacing(10);

                let sidebar_content: Element<_> =
                    {
                        keyed_column(self.sidebar_items.iter().enumerate().map(|(i, element)| {
                            (
                                i,
                                button(element.view(i).map(move |message| {
                                    Message::SidebarItemParentMessage(i, message)
                                }))
                                .on_press(Message::SetResourceDetails(element.resource.clone()))
                                .width(Length::Fill)
                                .style(styles::button::button_appearance)
                                /* .style(MyButtonStyleSheet) */
                                .into(),
                            )
                        }))
                        .spacing(10)
                        .into()
                    };

                // let content: Element<_> = keyed_column().into();

                let sidebar = container(column![sidebar_header, sidebar_content,].spacing(20))
                    /* .style(theme::Container::Box) */
                    .style(sidebar)
                    .height(Length::Fill)
                    .padding(padding::MAIN)
                    .width(Length::Shrink)
                    .max_width(200);

                // let header = container(row![
                //     horizontal_space(),
                //     text(String::from("Switcher"))
                //         .size(20)
                //         .font(Font::MONOSPACE),
                //     horizontal_space(),
                //     text(icons::Bootstrap::Dash.to_string())
                //         .font(iced_aw::BOOTSTRAP_FONT),
                //     text(icons::Bootstrap::X.to_string())
                //         .font(iced_aw::BOOTSTRAP_FONT),
                // ])
                // .width(Length::Fill)
                // .style(|theme: &Theme| {
                //     let palette = theme.extended_palette();

                //     container::Appearance::default().with_border(palette.background.strong.color, 1)
                // })
                // .padding(padding::MAIN);

                //     let footer = container(
                //     row![text(String::from("Footer Text"))]
                // )
                //     .style(theme::Container::Box)
                //     .center_y()
                //     .center_x()
                //     .width(Length::Fill)
                //     .padding(padding::MAIN);

                let main = container(Scrollable::new(
                    column![self
                        .main_content
                        .view()
                        .map(move |message| Message::ResourceDetailsMessage(message))]
                    .spacing(10),
                ))
                .style(main_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(padding::MAIN);

                let left = sidebar;
                let right = column![/* header, */ main /* footer */,].width(Length::FillPortion(3));

                let container = container(
                    row![left, right], /* FloatingElement::new(row![left, right], floating_content)
                                       .anchor(Anchor::SouthEast)
                                       .hide(false) */
                );
                container.into()
            }
        }
    }

    fn theme(&self) -> Theme {
        let theme_color = CustomTheme::from_system();

        let theme = match theme_color {
            CustomTheme::Dark => iced::Theme::custom(
                String::from("Custom"),
                iced::theme::Palette {
                    success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                    danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                    text: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.),
                    primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                    background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                },
            ),
            CustomTheme::Light => iced::Theme::custom(
                String::from("Custom"),
                iced::theme::Palette {
                    success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                    danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                    text: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.),
                    primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                    background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                },
            ),
        };

        theme
    }
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Clone, Default)]
pub struct ResourcePreviewMetrics {
    memory_usage: u64,
    swap_usage: u64,
    network_usage: u64,
    disk_written: u64,
    disk_read: u64,
    cpu_usage: f32,
    gpu_usage: f32,
}

#[derive(Debug, Default)]
enum SidebarItemParentDisplayState {
    #[default]
    Shown,
    Hidden,
}

#[derive(Debug, Default)]
pub struct SidebarItemParent {
    header: String,
    usage_percent: Option<f32>,
    resource: ResourceType,
    metric: Option<String>,
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
            usage_percent: None,
            metric: None,
            ..Default::default()
        }
    }

    fn on_tick(&mut self, system_info: &System, disk_info: &Disks, network_info: &Networks) {
        let (usage_percent, metric): (Option<f32>, Option<String>) = match self.resource {
            ResourceType::Applications => (None, None),
            ResourceType::Processes => {
                let processes = system_info.processes();

                (None, Some(processes.len().to_string()))
            }
            ResourceType::Cpu => {
                let cpus = system_info.cpus();
                // Relative to the number of cores. So 200% means 2 cores fully used
                let mut total_used: f32 = 0.;
                let mut cpu_count: u32 = 0;

                for cpu in cpus {
                    total_used += cpu.cpu_usage();
                    cpu_count += 1;
                }

                // Make it relative to total core usage
                let usage_percent = total_used / cpu_count as f32;
                self.usage_percent = Some(usage_percent);
                self.metric = Some(format!("{:.1}%", total_used));

                (Some(usage_percent), Some(format!("{:.1}%", usage_percent)))
            }
            ResourceType::Memory => {
                let total_used = system_info.used_memory();
                let total_capacity = system_info.total_memory();

                let usage_percent = total_used as f32 / total_capacity as f32 * 100.;

                (Some(usage_percent), Some(format!("{:.1}%", usage_percent)))
            }
            ResourceType::Gpu => (None, None),
            ResourceType::Disk => {
                let mut total_read = 0;
                let mut total_written = 0;

                /* for disk in disk_info {
                    disk.
                    total_read += disk.name().
                }

                system_info. */

                for (pid, process) in system_info.processes() {
                    let disk_usage = process.disk_usage();

                    total_read += disk_usage.read_bytes;
                    total_written += disk_usage.written_bytes;
                }

                (
                    None,
                    Some(format!("{:.1}% {:.1}%", total_read, total_written)),
                )
            }
            ResourceType::Wifi => {
                let mut total_received = 0;
                let mut total_transmitted = 0;

                for (interface_name, data) in network_info {
                    total_received += data.received();
                    total_transmitted += data.transmitted();
                }

                (
                    None,
                    Some(format!("{:.1}% {:.1}%", total_received, total_transmitted)),
                )
            }
            ResourceType::Ethernet => (None, None),
        };

        self.usage_percent = usage_percent;
        self.metric = metric;
    }

    fn view(&self, i: usize) -> Element<SidebarItemParentMessage> {
        match self.resource {
            ResourceType::Applications => String::from(icons::Bootstrap::WindowStack),
            ResourceType::Processes => String::from(icons::Bootstrap::PersonWorkspace),
            ResourceType::Cpu => String::from(icons::Bootstrap::Cpu),
            ResourceType::Memory => String::from(icons::Bootstrap::Memory),
            ResourceType::Gpu => String::from(icons::Bootstrap::GpuCard),
            ResourceType::Disk => String::from(icons::Bootstrap::Hdd),
            ResourceType::Wifi => String::from(icons::Bootstrap::Wifi),
            ResourceType::Ethernet => String::from(icons::Bootstrap::DiagramTwo),
        };

        let icon_text = match self.resource {
            ResourceType::Applications => String::from(icons::Bootstrap::WindowStack),
            ResourceType::Processes => String::from(icons::Bootstrap::PersonWorkspace),
            ResourceType::Cpu => String::from(icons::Bootstrap::Cpu),
            ResourceType::Memory => String::from(icons::Bootstrap::Memory),
            ResourceType::Gpu => String::from(icons::Bootstrap::GpuCard),
            ResourceType::Disk => String::from(icons::Bootstrap::Hdd),
            ResourceType::Wifi => String::from(icons::Bootstrap::Wifi),
            ResourceType::Ethernet => String::from(icons::Bootstrap::DiagramTwo),
        };

        let preview_state = {
            if let Some(usage_percent) = self.usage_percent {
                row![progress_bar::<Theme>(0.0..=100.0, usage_percent)
                    .height(5)
                    .width(Length::Fill)]
            } else {
                row![/* text(String::from("No bar")) */]
            }
        };

        let container = container(
            column![
                row![
                    text(icon_text).font(Font {
                        family: font::Family::Name("bootstrap-icons"),
                        ..Default::default()
                    }),
                    text(self.header.clone()),
                    text(self.metric.clone().unwrap_or("".to_string())).size(10),
                    // {
                    //     if let Some(metric) = &self.metric {
                    //         return text(metric).size(10).into();
                    //     }

                    //     text("".to_string()).size(10)
                    // }
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                preview_state
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
