use std::{
    collections::{HashMap, VecDeque},
    env,
    ffi::OsString,
    time::Duration,
};

use battery::{
    units::{ElectricPotential, Energy, Power, ThermodynamicTemperature},
    Battery,
};
use constants::{padding, DisplayState, ICON, PERCENT_PRECISION};
use iced::{
    advanced::{
        graphics::{
            futures::backend::default, image::image_rs::ImageFormat, text::cosmic_text::SwashImage,
        },
        widget::Text,
    },
    alignment, color, executor, font,
    theme::{
        self,
        palette::{Extended, Secondary},
        Palette,
    },
    widget::{
        button, column, container, horizontal_space, keyed_column, progress_bar, row,
        scrollable::{self, Direction, Properties, RelativeOffset},
        shader::wgpu::{hal::empty::Resource, naga::proc},
        text, text_input, Column, Container, Image, Row, Scrollable, Space,
    },
    window::{icon, Icon},
    Alignment, Application, Color, Command, Element, Font, Length, Pixels, Renderer, Sandbox,
    Settings, Subscription, Theme,
};
use iced_aw::{
    floating_element::{self, Anchor},
    graphics::icons::{self, icon_to_char},
    native::Split,
    split, BootstrapIcon, FloatingElement, NerdIcon, Spinner, NERD_FONT,
};
use preferences::Preferences;
use resource_pages::{
    applications_page::{ApplicationsPage, ApplicationsPageMessage},
    cpu_page::{CpuPage, CpuPageMessage},
    disk_page::{DiskPage, DiskPageMessage},
    memory_page::{MemoryPage, MemoryPageMessage},
    resource_details::{ResourceDetails, ResourceDetailsMessage},
};

use resource_previews::{
    cpu_preview::{self, CpuPreview},
    disk_preview::DiskPreview,
    memory_preview::MemoryPreview,
    resource_preview::ResourcePreviewMessage,
};
use sidebar::sidebar_item::{SidebarItemParent, SidebarItemParentMessage};
use styles::container::{main_content, sidebar};
use sysinfo::{
    Cpu, CpuRefreshKind, Disk, DiskKind, Disks, MemoryRefreshKind, Networks, Pid,
    ProcessRefreshKind, RefreshKind, System, UpdateKind,
};
use types::resource_data::{DiskData, ResourceData};

use crate::constants::HISTORY_TICKS;

mod constants;
mod general_widgets;
mod preferences;
mod resource_pages;
mod resource_previews;
mod sidebar;
mod styles;
mod utils;
mod types;

pub fn main() -> iced::Result {
    // let image = Image::load_from_memory(ICON).unwrap();
    // let icon = iced::window::icon::from_rgba(ICON.as_bytes().to_vec(), ICON_HEIGHT, ICON_WIDTH).unwrap();
    //let icon = iced::window::icon::from_rgba(rgba, width, height)

    let icon = iced::window::icon::from_file_data(ICON, Some(ImageFormat::Png)).unwrap();

    let settings = Settings {
        window: iced::window::Settings {
            icon: Some(icon),
            transparent: true,
            // decorations: false,
            ..Default::default()
        },
        ..Default::default()
    };

    env::set_var("RUST_BACKTRACE", "1");
    App::run(settings)
}

pub enum CustomThemeChoice {
    Light,
    Dark,
}

impl CustomThemeChoice {
    pub fn from_system() -> Self {
        CustomThemeChoice::Dark
    }
}

#[derive(Debug, Default)]
pub struct ResourceHistory {
    // use the difference between the current tick and the last tick
    pub last_tick: i32,
    pub cpu: VecDeque<(i32, i32)>,
    pub logical_cores: Vec<VecDeque<(i32, i32)>>,
    pub ram: VecDeque<(i32, i32)>,
    pub swap: VecDeque<(i32, i32)>,
    pub disk_write: HashMap<String, VecDeque<(i32, i32)>>,
    pub disk_read: HashMap<String, VecDeque<(i32, i32)>>,
    pub gpu: VecDeque<(i32, i32)>,
    pub vram: VecDeque<(i32, i32)>,
    pub wifi: VecDeque<(i32, i32)>,
    pub ethernet: VecDeque<(i32, i32)>,
}

impl ResourceHistory {
    fn new(logical_cores_count: u32) -> Self {
        let mut logical_cores = vec![];

        // initialize history queue for each logical core
        for _ in 0..logical_cores_count {
            logical_cores.push(VecDeque::new());
        }

        Self {
            logical_cores,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct ResourcePreviews {
    pub cpu: CpuPreview,
    pub memory: MemoryPreview,
    pub disks: HashMap<String, DiskPreview>,
}

#[derive(Debug)]

pub enum ResourcePage {
    Cpu(CpuPage),
    Disk(DiskPage),
    Memory(MemoryPage),
    Applications(ApplicationsPage),
}

#[derive(Debug)]
pub struct ResourcePages {
    pub disks: HashMap<String, DiskPage>,
}

impl ResourcePages {
    fn new() -> Self {
        Self {
            disks: HashMap::new(),
        }
    }
}

//

#[derive(Debug, Clone)]
pub enum ResourcePageMessage {
    DiskPageMessage(DiskPageMessage),
    CpuPageMessage(CpuPageMessage),
    MemoryPageMessage(MemoryPageMessage),
    ApplicationsPageMessage(ApplicationsPageMessage),
}

#[derive(Debug, Clone)]
enum AppMessage {
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SidebarItemParentMessage(usize, SidebarItemParentMessage),
    ResourceDetailsMessage(ResourceDetailsMessage),
    ResourcePageMessage(ResourcePageMessage),
    SetResourceDetails(ResourceType),
    ResourcePreviewMessage(ResourcePreviewMessage),
    Tick,
}

#[derive(Debug, Default)]
enum AppState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug, Clone)]
pub struct ActivePreview {
    pub resource: ResourceType,
    // A identifying name if there are multiple devices of the resource type
    pub name: Option<String>,
}

#[derive(Debug)]
struct App {
    sidebar_items: Vec<SidebarItemParent>,
    preferences: Preferences,
    resource_page: ResourcePage,
    tick_interval: u64,
    system_info: System,
    physical_core_count: u32,
    logical_core_count: u32,
    cpu_brand: String,
    cpu_frequency: u64,
    disk_info: Disks,
    network_info: Networks,
    state: AppState,
    tick: i32,
    logical_cores_usage_percent: Vec<f32>,
    logical_cores_frequencies: Vec<u64>,
    resource_history: ResourceHistory,
    resource_data: ResourceData,
    previews: ResourcePreviews,
    #[deprecated]
    resources_details: ResourcePages,
    active_preview: ActivePreview,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AppMessage>) {
        let system_info = System::new_all();
        let physical_core_count = system_info.physical_core_count().unwrap_or(1) as u32/* system_info.cpus().len() as u32 */;
        let logical_core_count = system_info.cpus().len() as u32;

        let cpu_brand = system_info.global_cpu_info().brand().to_string();

        let mut logical_cores_usage_percent = Vec::new();
        let mut logical_cores_frequencies = Vec::new();

        for _ in 0..logical_core_count {
            logical_cores_usage_percent.push(0.);
            logical_cores_frequencies.push(0);
        }

        let preferences = Preferences::new();

        let mut new_self = Self {
            tick_interval: 1000,
            state: AppState::Loading,
            preferences,
            system_info,
            physical_core_count,
            logical_core_count,
            cpu_brand,
            tick: 0,
            logical_cores_usage_percent,
            logical_cores_frequencies,
            resource_history: ResourceHistory::new(logical_core_count),
            sidebar_items: Vec::new(),
            resource_page: ResourcePage::Cpu(CpuPage::new(&preferences)),
            active_preview: ActivePreview {
                resource: ResourceType::default(),
                name: None,
            },
            resource_data: ResourceData::new(),
            previews: ResourcePreviews::default(),
            resources_details: ResourcePages::new(),
            disk_info: Disks::new(),
            network_info: Networks::new(),
            cpu_frequency: 0,
        };

        // new_self.previews.disks.insert("disk 1".to_string(), DiskPreview::new());

        let command = Command::batch(vec![
            // font::load(iced_aw::NERD_FONT_BYTES).map(Message::FontLoaded),
            // Command::perform(load(), Message::Loaded),
            font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(AppMessage::FontLoaded),
            Command::perform(load(), AppMessage::Loaded),
        ]);

        (new_self, command)
    }

    fn title(&self) -> String {
        String::from("C Tasks")
    }

    fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        match self.state {
            AppState::Loading => match message {
                AppMessage::Loaded(Ok(state)) => {
                    println!("loading success");

                    self.sidebar_items = vec![
                        SidebarItemParent::new(
                            ResourceType::Applications,
                            String::from("Applications"),
                        ),
                        SidebarItemParent::new(ResourceType::Processes, String::from("Processes")),
                        SidebarItemParent::new(ResourceType::Memory, String::from("Memory")),
                        SidebarItemParent::new(ResourceType::Gpu, String::from("Gpu")),
                        SidebarItemParent::new(ResourceType::Wifi, String::from("Wifi")),
                        SidebarItemParent::new(ResourceType::Ethernet, String::from("Ethernet")),
                    ];
                    self.state = AppState::Loaded;
                }
                AppMessage::Loaded(Err(_)) => {
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
                    AppMessage::Tick => {
                        self.resource_history.last_tick = self.tick;
                        self.tick += 1;

                        // Change this to call specific to be more optimal

                        self.system_info.refresh_cpu_specifics(
                            CpuRefreshKind::new().with_cpu_usage().with_frequency(),
                        );
                        self.system_info.refresh_processes_specifics(
                            ProcessRefreshKind::new().with_user(UpdateKind::Always),
                        );
                        self.system_info
                            .refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
                        self.disk_info = Disks::new_with_refreshed_list();
                        self.network_info = Networks::new_with_refreshed_list();

                        // Update and construct disk data

                        for disk in &self.disk_info {
                            let disk_name = disk.name().to_str().unwrap_or("default").to_string();

                            if let Some(disk_data) = self.resource_data.disks.get_mut(&disk_name) {
                                disk_data.update(&disk_name, disk);
                                continue;
                            };

                            let mut new_disk_data = DiskData::new();

                            new_disk_data.update(&disk_name, disk);

                            self.resource_data.disks.insert(disk_name, new_disk_data);
                        }

                        // Update and construct disk previews and details

                        for (_, disk_data) in &self.resource_data.disks {
                            if self.previews.disks.get_mut(&disk_data.name).is_none() {
                                let new_preview = DiskPreview::new();

                                self.previews
                                    .disks
                                    .insert(disk_data.name.clone(), new_preview);
                            }
                        }

                        // cpu
                        self.resource_data
                            .cpu
                            .update(self.system_info.cpus(), self.logical_core_count);

                        // ram
                        self.resource_data.memory.update(&self.system_info);

                        // cpu history

                        let tick_delta = self.tick - self.resource_history.last_tick;

                        for history_tick in &mut self.resource_history.cpu {
                            history_tick.0 -= tick_delta;
                        }

                        self.resource_history
                            .cpu
                            .retain(|history_tick| history_tick.0 >= 0);

                        self.resource_history.cpu.push_back((
                            HISTORY_TICKS as i32,
                            self.resource_data.cpu.cpu_usage_percent as i32,
                        ));

                        // logical cores

                        // logical cores history

                        for (index, history) in
                            self.resource_history.logical_cores.iter_mut().enumerate()
                        {
                            for history_tick in history.iter_mut() {
                                history_tick.0 -= tick_delta;
                            }

                            history.retain(|history_tick| history_tick.0 >= 0);

                            history.push_back((
                                HISTORY_TICKS as i32,
                                self.logical_cores_usage_percent[index] as i32,
                            ));
                        }

                        // ram history

                        for history_tick in &mut self.resource_history.ram {
                            history_tick.0 -= tick_delta;
                        }

                        self.resource_history
                            .ram
                            .retain(|history_tick| history_tick.0 >= 0);

                        self.resource_history.ram.push_back((
                            HISTORY_TICKS as i32,
                            self.resource_data.memory.ram_usage_percent as i32,
                        ));

                        // swap history

                        for history_tick in &mut self.resource_history.swap {
                            history_tick.0 -= tick_delta;
                        }

                        self.resource_history
                            .swap
                            .retain(|history_tick| history_tick.0 >= 0);

                        self.resource_history.swap.push_back((
                            HISTORY_TICKS as i32,
                            self.resource_data.memory.swap_usage_percent as i32,
                        ));

                        // disk history

                        for (disk_name, disk_data) in &self.resource_data.disks {
                            // written

                            let written_history = self
                                .resource_history
                                .disk_write
                                .entry(disk_name.clone())
                                .or_insert(VecDeque::new());

                            for history_tick in &mut *written_history {
                                history_tick.0 -= tick_delta;
                            }

                            written_history.retain(|history_tick| history_tick.0 >= 0);

                            written_history
                                .push_back((HISTORY_TICKS as i32, disk_data.written as i32));

                            // read

                            let read_history = self
                                .resource_history
                                .disk_read
                                .entry(disk_name.clone())
                                .or_insert(VecDeque::new());

                            for history_tick in &mut *read_history {
                                history_tick.0 -= tick_delta;
                            }

                            read_history.retain(|history_tick| history_tick.0 >= 0);

                            read_history.push_back((HISTORY_TICKS as i32, disk_data.read as i32));
                        }

                        // resource page

                        update_resource_page(self);

                        //

                        println!("tick: {}", self.tick);
                    }
                    AppMessage::ResourcePageMessage(resource_page_message) => {
                        // maybe this is good reason to split each page into its own message, since they each may have unique properties

                        match resource_page_message {
                            ResourcePageMessage::DiskPageMessage(disk_page_message) => {}
                            ResourcePageMessage::CpuPageMessage(cpu_page_message) => {}
                            ResourcePageMessage::MemoryPageMessage(memory_page_message) => {}
                            ResourcePageMessage::ApplicationsPageMessage(
                                applications_page_message,
                            ) => {}
                            _ => {}
                        }
                    }
                    AppMessage::ResourceDetailsMessage(resource_details_message) => {
                        // println!("message: {:?}", resource_details_message);

                        /* let _ = self
                            .main_content
                            .update(resource_details_message)
                            .map(AppMessage::ResourceDetailsMessage);

                        match resource_details_message {
                            ResourceDetailsMessage::ToggleLogicalCores(_toggle_state) => {
                                self.main_content.on_tick(
                                    &mut self.system_info,
                                    self.cpu_usage_percent,
                                    self.physical_cpu_count,
                                    self.logical_core_count,
                                    self.cpu_brand.clone(),
                                    self.cpu_frequency,
                                    &self.resource_history,
                                    &self.logical_cores_usage_percent,
                                    &self.logical_cores_frequencies,
                                    &self.resource_data,
                                    &self.preferences,
                                    &self.active_preview,
                                );
                            }
                            _ => {}
                        } */
                    }
                    AppMessage::SetResourceDetails(resource) => {
                        /* if resource == self.main_content.resource {
                            return;
                        }

                        self.main_content
                            .apply_resource_type(resource, &self.preferences);
                        self.main_content.on_tick(
                            &mut self.system_info,
                            self.cpu_usage_percent,
                            self.physical_cpu_count,
                            self.logical_core_count,
                            self.cpu_brand.clone(),
                            self.cpu_frequency,
                            &self.resource_history,
                            &self.logical_cores_usage_percent,
                            &self.logical_cores_frequencies,
                            &self.resource_data,
                            &self.preferences,
                            &self.active_preview,
                        ); */
                    }
                    AppMessage::ResourcePreviewMessage(preview_message) => {
                        match preview_message {
                            ResourcePreviewMessage::ResourcePageFor(active_preview) => {
                                // We also need a way to toggle this off, ideally not being super complicated
                                // if let Some(preview) = self.previews.disks.get_mut(&key) {
                                //     preview.display_state = ResourcePreviewDisplayState::Active;
                                // };

                                let active_preview = ActivePreview {
                                    resource: active_preview.resource,
                                    name: active_preview.name,
                                };

                                change_resource_page(self, &active_preview);

                                self.active_preview = active_preview;

                                update_resource_page(self);

                                // change resource page to match preview

                                /* self.main_content
                                .apply_resource_type(active_preview.resource, &self.preferences) */
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        iced::time::every(std::time::Duration::from_millis(self.tick_interval))
            .map(|_| AppMessage::Tick)
    }

    fn view(&self) -> Element<AppMessage> {
        match self.state {
            AppState::Loading => {
                let spinner = Spinner::new();
                let loading = column![text(String::from("Loading App")), spinner];

                container(loading)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            AppState::Loaded => {
                let floating_content = container(row![]); /* container(
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
                                                          .align_y(alignment::Vertical::Center); */

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
                            button(element.view(i).map(move |message| {
                                AppMessage::SidebarItemParentMessage(i, message)
                            }))
                            .style(theme::Button::Text)
                            .on_press(AppMessage::SetResourceDetails(element.resource))
                            .width(Length::Fill)
                            /* .style(styles::button::button_appearance(&self.theme())) */
                            .into(),
                        )
                    }))
                    .spacing(10)
                    .into()
                };

                let sidebar_content_new = Column::with_children({
                    let mut children = Vec::new();

                    children.push(
                        self.previews
                            .cpu
                            .view(
                                &self.preferences,
                                &self.active_preview,
                                &self.resource_data.cpu,
                            )
                            .map(AppMessage::ResourcePreviewMessage),
                    );

                    children.push(
                        self.previews
                            .memory
                            .view(
                                &self.preferences,
                                &self.active_preview,
                                &self.resource_data.memory,
                            )
                            .map(AppMessage::ResourcePreviewMessage),
                    );

                    for (disk_name, disk_preview) in &self.previews.disks {
                        let disk_data = self.resource_data.disks.get(disk_name).unwrap();

                        children.push(
                            disk_preview
                                .view(&self.preferences, &self.active_preview, disk_data)
                                .map(AppMessage::ResourcePreviewMessage),
                        );
                    }

                    children
                })
                .spacing(padding::PORTION);

                let sidebar = container(
                    iced::widget::scrollable(
                        column![sidebar_header, sidebar_content, sidebar_content_new]
                            .spacing(20)
                            .padding(padding::MAIN),
                    )
                    .style(iced::theme::Scrollable::Custom(Box::new(
                        styles::scrollable::Background3 {},
                    ))),
                )
                .height(Length::Fill)
                .width(Length::Shrink)
                .max_width(200)
                .style(styles::container::sidebar());

                // let header = container(row![
                //     horizontal_space(),
                //     text(String::from("Switcher"))
                //         .size(20)
                //         .font(Font::MONOSPACE),
                //     horizontal_space(),
                //     text(iced_aw::graphics::icons::BootstrapIcon::Dash.to_string())
                //         .font(iced_aw::BOOTSTRAP_FONT),
                //     text(iced_aw::graphics::icons::BootstrapIcon::X.to_string())
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

                let main_new = container({
                    let page: Element<_> = match &self.resource_page {
                        ResourcePage::Cpu(cpu_page) => cpu_page
                            .view(
                                &self.preferences,
                                &self.resource_data.cpu,
                                self.physical_core_count,
                                self.logical_core_count,
                                self.cpu_brand.clone(),
                            )
                            .map(move |message| {
                                AppMessage::ResourcePageMessage(
                                    ResourcePageMessage::CpuPageMessage(message),
                                )
                            }),
                        ResourcePage::Memory(memory_page) => memory_page
                            .view(&self.preferences, &self.resource_data.memory)
                            .map(move |message| {
                                AppMessage::ResourcePageMessage(
                                    ResourcePageMessage::MemoryPageMessage(message),
                                )
                            }),
                        ResourcePage::Disk(disk_page) => {
                            let active_preview_name = &self.active_preview.name.as_ref().unwrap();

                            let Some(data) =
                                self.resource_data.disks.get(active_preview_name.as_str())
                            else {
                                return text(format!(
                                    "Error: failed to access data for disk {}",
                                    active_preview_name.as_str()
                                ))
                                .into();
                            };

                            disk_page.view(&self.preferences, data).map(move |message| {
                                AppMessage::ResourcePageMessage(
                                    ResourcePageMessage::DiskPageMessage(message),
                                )
                            })
                        }
                        _ => text(String::from("Error: failed to match resource")).into(),
                    };

                    /* match self.active_preview.resource {
                        ResourceType::Disk => {

                            let active_preview_name = &self.active_preview.name.as_ref().unwrap();

                            let Some(data) = self.resource_data.disks.get(active_preview_name.as_str()) else {
                                return text(format!("Error: failed to access data for disk {}", active_preview_name.as_str())).into()
                            };

                            self.resource_page
                                .view(&self.preferences, data)
                                .map(move |message| {
                                    AppMessage::ResourcePageMessage(
                                        ResourcePageMessage::DiskPageMessage(message),
                                    )
                                })
                                .into()
                        }
                        _ => text(String::from("Error: failed to match resource")).into(),
                    }; */

                    page
                })
                .style(main_content());

                let left = sidebar;
                let right =
                    column![/* header, */ main_new /* footer */].width(Length::FillPortion(3));

                let container = container(
                    FloatingElement::new(row![left, right], floating_content)
                        .anchor(Anchor::SouthEast)
                        .hide(false),
                )
                .style(styles::container::main_content());
                container.into()
            }
        }
    }

    fn theme(&self) -> Theme {
        let theme_color = CustomThemeChoice::from_system();

        match theme_color {
            CustomThemeChoice::Dark => iced::Theme::custom(
                String::from("Custom"),
                iced::theme::Palette {
                    success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                    danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                    text: Color::from_rgb(1., 1., 1.),
                    // primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                    primary: Color::from_rgb(0.21, 0.52, 0.89),
                    background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                },
            ),
            CustomThemeChoice::Light => iced::Theme::custom(
                String::from("Custom"),
                iced::theme::Palette {
                    success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                    danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                    text: Color::from_rgb(1., 1., 1.),
                    // primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                    primary: Color::from_rgb(0.21, 0.52, 0.89),
                    background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                },
            ),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub enum ResourceType {
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

fn change_resource_page(app: &mut App, active_preview: &ActivePreview) {
    match active_preview.resource {
        ResourceType::Cpu => {
            app.resource_page = ResourcePage::Cpu(CpuPage::new(&app.preferences));
        }
        ResourceType::Disk => {
            app.resource_page = ResourcePage::Disk(DiskPage::new(&app.preferences));
        }
        ResourceType::Memory => {
            app.resource_page = ResourcePage::Memory(MemoryPage::new(&app.preferences));
        }
        _ => {}
    }
}

fn update_resource_page(app: &mut App) {
    match &mut app.resource_page {
        ResourcePage::Cpu(cpu_page) => {
            cpu_page.update_history(&app.resource_history);
        }
        ResourcePage::Memory(memory_page) => {
            memory_page.update_history(&app.resource_history);
        }
        ResourcePage::Disk(disk_page) => {
            for disk in &app.disk_info {
                let disk_name = disk.name().to_str().unwrap_or("default").to_string();

                let disk_data = app.resource_data.disks.get_mut(&disk_name).unwrap();

                disk_data.update_in_depth(&disk_name, disk);

                disk_page.update_history(&app.active_preview, &app.resource_history)
            }
        }
        _ => {}
    }
}
