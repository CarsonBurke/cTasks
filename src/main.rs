use iced::{
    executor, theme,
    widget::{column, container, horizontal_space, row, scrollable, text, Space},
    Alignment, Application, Command, Element, Font, Length, Settings, Theme,
};
pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { ..Self::default() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("C Tasks")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                println!("ticked, {:?}", message);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let sidebar_header = row![
            text(String::from("Prefs")),
            // horizontal_space(),
            text(String::from("C Tasks")),
            // horizontal_space(),
            text(String::from("toggle"))
        ].spacing(10);

        let sidebar_content = column![
            text(String::from("applications")),
            text(String::from("processes")),
            text(String::from("cpu")),
            text(String::from("memory")),
            text(String::from("gpu 1")),
            text(String::from("gpu 2")),
            text(String::from("disk")),
            text(String::from("wifi")),
            text(String::from("Ethernet if exists")),
        ];

        let sidebar = container(column![
            sidebar_header,
            sidebar_content,
        ].spacing(20))
        .style(theme::Container::Box)
        .height(Length::Fill)
        .padding(20)
        .width(Length::Shrink);

        let header = container(
            row![
                horizontal_space(),
                text(String::from("Switcher"))
                    .size(20)
                    .font(Font::MONOSPACE),
                horizontal_space(),
                text(String::from("- x")),
            ],
        )
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Appearance::default().with_border(palette.background.strong.color, 1)
        })
        .padding(20);

        let footer = container(row![text(String::from("Footer Text"))])
            .style(theme::Container::Box)
            .center_y()
            .center_x()
            .width(Length::Fill)
            .padding(20);

        let main = container(scrollable(
            column![text("hello".to_string())].spacing(10),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20);
    
        let left = sidebar;
        let right = column![header, main, footer]
        .width(Length::FillPortion(3));

        row![left, right].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
