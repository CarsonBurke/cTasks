use std::{
    collections::{HashMap, VecDeque},
    env,
    ffi::OsString,
    time::Duration,
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
use resource_details::resource_details::{ResourceDetails, ResourceDetailsMessage};
use resource_previews::{disk_preview::DiskPreview, resource_preview::ResourcePreviewMessage};
use sidebar::sidebar_item::{SidebarItemParent, SidebarItemParentMessage};
use styles::container::{main_content, sidebar};
use sysinfo::{
    Cpu, CpuRefreshKind, Disk, DiskKind, Disks, MemoryRefreshKind, Networks, ProcessRefreshKind, RefreshKind, System, UpdateKind
};

use crate::{
    constants::HISTORY_TICKS, resource_previews::resource_preview::ResourcePreviewDisplayState,
};

mod constants;
mod general_widgets;
mod preferences;
mod resource_details;
mod resource_previews;
mod sidebar;
mod styles;
mod utils;

pub fn main() -> iced::Result {
    // let image = Image::load_from_memory(ICON).unwrap();
    // let icon = iced::window::icon::from_rgba(ICON.as_bytes().to_vec(), ICON_HEIGHT, ICON_WIDTH).unwrap();
    //let icon = iced::window::icon::from_rgba(rgba, width, height)

    let icon = iced::window::icon::from_file_data(ICON, Some(ImageFormat::Png)).unwrap();

    let settings = Settings {
        window: iced::window::Settings {
            icon: Some(icon),
            transparent: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // env::set_var("RUST_BACKTRACE", "1");
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
    pub disk_write: Vec<VecDeque<(i32, i32)>>,
    pub disk_read: Vec<VecDeque<(i32, i32)>>,
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

#[derive(Debug)]
pub struct DiskData {
    pub read: u64,
    pub written: u64,
    pub is_removable: bool,
    pub kind: DiskKind,
    pub name: String,
    pub space_total: u64,
    pub space_used: u64,
}

impl Default for DiskData {
    fn default() -> Self {
        Self {
            name: String::new(),
            read: 0,
            written: 0,
            space_total: 0,
            space_used: 0,
            is_removable: false,
            kind: DiskKind::Unknown(0),
        }
    }
}

#[derive(Debug, Default)]
pub struct ResourcePreviews {
    pub disks: HashMap<String, DiskPreview>,
}

#[derive(Debug, Default)]
pub struct ResourcesDetails {
    // pub disks: HashMap<OsString, DiskPreview>,
}

#[derive(Debug, Default)]
pub struct ResourceData {
    pub disks: HashMap<String, DiskData>,
}

#[derive(Debug, Clone)]
enum AppMessage {
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SidebarItemParentMessage(usize, SidebarItemParentMessage),
    ResourceDetailsMessage(ResourceDetailsMessage),
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

pub type ActivePreview = (String, ResourceType);

#[derive(Debug, Default)]
struct App {
    sidebar_items: Vec<SidebarItemParent>,
    preferences: Preferences,
    main_content: ResourceDetails,
    tick_interval: u64,
    system_info: System,
    physical_cpu_count: u32,
    logical_cpu_count: u32,
    cpu_brand: String,
    cpu_frequency: u64,
    disk_info: Disks,
    network_info: Networks,
    cpu_usage_percent: f32,
    ram_usage_percent: f32,
    swap_usage_percent: f32,
    state: AppState,
    tick: i32,
    logical_cores_usage_percent: Vec<f32>,
    logical_cores_frequencies: Vec<u64>,
    resource_history: ResourceHistory,
    resource_data: ResourceData,
    previews: ResourcePreviews,
    resources_details: ResourcesDetails,
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
        let physical_cpu_count = system_info.physical_core_count().unwrap_or(1) as u32/* system_info.cpus().len() as u32 */;
        let logical_cpu_count = system_info.cpus().len() as u32;

        let cpu_brand = system_info.global_cpu_info().brand().to_string();

        let mut logical_cores_usage_percent = Vec::new();
        let mut logical_cores_frequencies = Vec::new();

        for _ in 0..logical_cpu_count {
            logical_cores_usage_percent.push(0.);
            logical_cores_frequencies.push(0);
        }

        let mut new_self = Self {
            tick_interval: 1000,
            state: AppState::Loading,
            preferences: Preferences::new(),
            system_info,
            physical_cpu_count,
            logical_cpu_count,
            tick: 0,
            cpu_brand,
            logical_cores_usage_percent,
            logical_cores_frequencies,
            resource_history: ResourceHistory::new(logical_cpu_count),
            ..Default::default()
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
                        SidebarItemParent::new(ResourceType::Cpu, String::from("Cpu")),
                        SidebarItemParent::new(ResourceType::Memory, String::from("Memory")),
                        SidebarItemParent::new(ResourceType::Gpu, String::from("Gpu")),
                        SidebarItemParent::new(ResourceType::Disk, String::from("Disk")),
                        SidebarItemParent::new(ResourceType::Wifi, String::from("Wifi")),
                        SidebarItemParent::new(ResourceType::Ethernet, String::from("Ethernet")),
                    ];
                    self.state = AppState::Loaded;

                    self.main_content =
                        ResourceDetails::new(&self.preferences, ResourceType::Processes);
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

                (|| {
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
                                    update_disk_data(disk_data, &disk_name, disk);
                                    continue;
                                };

                                let mut new_disk_data = DiskData::default();

                                update_disk_data(&mut new_disk_data, &disk_name, disk);

                                self.resource_data.disks.insert(disk_name, new_disk_data);
                            }

                            // Update and construct disk previews

                            for (_, disk_data) in &self.resource_data.disks {
                                if let Some(preview) = self.previews.disks.get_mut(&disk_data.name) {
                                    preview.on_tick(disk_data);
                                    continue;
                                };

                                let mut new_preview = DiskPreview::new();

                                new_preview.on_tick(disk_data);

                                self.previews.disks.insert(disk_data.name.clone(), new_preview);
                            }

                            // cpu usage

                            let cpus = self.system_info.cpus();
                            // Relative to the number of logical cores. So 200% means 2 cores fully used
                            let mut total_used: f32 = 0.;

                            for (index, cpu) in cpus.iter().enumerate() {
                                let cpu_usage = cpu.cpu_usage();
                                let frequency = cpu.frequency();

                                self.logical_cores_usage_percent[index] = cpu_usage;

                                self.logical_cores_frequencies[index] = frequency;

                                total_used += cpu_usage;
                            }

                            self.cpu_usage_percent = total_used / self.logical_cpu_count as f32;

                            let global_cpu_info = self.system_info.global_cpu_info();

                            self.cpu_frequency = global_cpu_info.frequency();

                            // ram

                            let total_used = self.system_info.used_memory();
                            let total_capacity = self.system_info.total_memory();

                            self.ram_usage_percent =
                                total_used as f32 / total_capacity as f32 * 100.;

                            // swap

                            let total_used = self.system_info.used_swap();
                            let total_capacity = self.system_info.total_swap();

                            self.swap_usage_percent =
                                total_used as f32 / total_capacity as f32 * 100.;

                            // cpu history

                            let tick_delta = self.tick - self.resource_history.last_tick;

                            for history_tick in &mut self.resource_history.cpu {
                                history_tick.0 -= tick_delta as i32;
                            }

                            self.resource_history
                                .cpu
                                .retain(|history_tick| history_tick.0 >= 0);

                            self.resource_history
                                .cpu
                                .push_back((HISTORY_TICKS as i32, self.cpu_usage_percent as i32));

                            // logical cores

                            // logical cores history

                            for (index, history) in
                                self.resource_history.logical_cores.iter_mut().enumerate()
                            {
                                for history_tick in history.iter_mut() {
                                    history_tick.0 -= tick_delta as i32;
                                }

                                history.retain(|history_tick| history_tick.0 >= 0);

                                history.push_back((
                                    HISTORY_TICKS as i32,
                                    self.logical_cores_usage_percent[index] as i32,
                                ));
                            }

                            // ram history

                            for history_tick in &mut self.resource_history.ram {
                                history_tick.0 -= tick_delta as i32;
                            }

                            self.resource_history
                                .ram
                                .retain(|history_tick| history_tick.0 >= 0);

                            self.resource_history
                                .ram
                                .push_back((HISTORY_TICKS as i32, self.ram_usage_percent as i32));

                            // swap history

                            for history_tick in &mut self.resource_history.swap {
                                history_tick.0 -= tick_delta as i32;
                            }

                            self.resource_history
                                .swap
                                .retain(|history_tick| history_tick.0 >= 0);

                            self.resource_history
                                .swap
                                .push_back((HISTORY_TICKS as i32, self.swap_usage_percent as i32));

                            //

                            println!("tick: {}", self.tick);
                            for element in self.sidebar_items.iter_mut() {
                                element.on_tick(
                                    &self.system_info,
                                    self.cpu_usage_percent,
                                    self.ram_usage_percent,
                                    &self.disk_info,
                                    &self.network_info,
                                );
                            }

                            self.main_content.on_tick(
                                &mut self.system_info,
                                self.cpu_usage_percent,
                                self.physical_cpu_count,
                                self.logical_cpu_count,
                                self.cpu_brand.clone(),
                                self.cpu_frequency,
                                &self.resource_history,
                                &self.logical_cores_usage_percent,
                                &self.logical_cores_frequencies,
                                &self.resource_data,
                                &self.preferences,
                            );
                        }
                        AppMessage::ResourceDetailsMessage(resource_details_message) => {
                            // println!("message: {:?}", resource_details_message);

                            let _ = self
                                .main_content
                                .update(resource_details_message)
                                .map(AppMessage::ResourceDetailsMessage);

                            match resource_details_message {
                                ResourceDetailsMessage::ToggleLogicalCores(_toggle_state) => {
                                    self.main_content.on_tick(
                                        &mut self.system_info,
                                        self.cpu_usage_percent,
                                        self.physical_cpu_count,
                                        self.logical_cpu_count,
                                        self.cpu_brand.clone(),
                                        self.cpu_frequency,
                                        &self.resource_history,
                                        &self.logical_cores_usage_percent,
                                        &self.logical_cores_frequencies,
                                        &self.resource_data,
                                        &self.preferences,
                                    );
                                }
                                _ => {}
                            }
                        }
                        AppMessage::SetResourceDetails(resource) => {
                            if resource == self.main_content.resource {
                                return;
                            }

                            self.main_content
                                .apply_resource_type(resource, &self.preferences);
                            self.main_content.on_tick(
                                &mut self.system_info,
                                self.cpu_usage_percent,
                                self.physical_cpu_count,
                                self.logical_cpu_count,
                                self.cpu_brand.clone(),
                                self.cpu_frequency,
                                &self.resource_history,
                                &self.logical_cores_usage_percent,
                                &self.logical_cores_frequencies,
                                &self.resource_data,
                                &self.preferences,
                            );
                        }
                        AppMessage::ResourcePreviewMessage(preview_message) => {
                            match preview_message {
                                ResourcePreviewMessage::ResourceDetailsFor(key, resource) => {
                                    // We also need a way to toggle this off, ideally not being super complicated
                                    // if let Some(preview) = self.previews.disks.get_mut(&key) {
                                    //     preview.display_state = ResourcePreviewDisplayState::Active;
                                    // };
                                    self.active_preview = (key, resource);

                                    self.main_content
                                        .apply_resource_type(resource, &self.preferences)
                                }
                            }
                        }
                        _ => {}
                    }
                })();
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

                    for (_, disk_preview) in &self.previews.disks {
                        children.push(
                            disk_preview
                                .view(&self.active_preview)
                                .map(|message| AppMessage::ResourcePreviewMessage(message)),
                        );
                    }

                    children
                });

                let sidebar = container(
                    column![sidebar_header, sidebar_content, sidebar_content_new].spacing(20),
                )
                /* .style(theme::Container::Box) */
                .style(sidebar())
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

                let main = container(
                    //                    Scrollable::new(
                    column![self
                        .main_content
                        .view()
                        .map(move |message| AppMessage::ResourceDetailsMessage(message))]
                    .spacing(10),
                    // )
                    // .direction(Direction::Vertical(Properties::default())),
                )
                .style(main_content())
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x();

                let left = sidebar;
                let right = column![/* header, */ main /* footer */,].width(Length::FillPortion(3));

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
        let theme_color = CustomThemeChoice::from_system();

        let theme = match theme_color {
            CustomThemeChoice::Dark => iced::Theme::custom(
                String::from("Custom"),
                iced::theme::Palette {
                    success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                    danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                    text: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.),
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
                    text: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.),
                    // primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                    primary: Color::from_rgb(0.21, 0.52, 0.89),
                    background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                },
            ),
        };

        theme
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

pub fn update_disk_data(disk_data: &mut DiskData, disk_name: &String, disk: &Disk) {

    disk_data.name = disk_name.clone();
    disk_data.space_total = disk.total_space();
    disk_data.space_used = disk_data.space_total - disk.available_space();
    disk_data.read = 0;
    disk_data.written = 0;
    disk_data.kind = disk.kind();
    disk_data.is_removable = disk.is_removable();
}