use iced::{self, executor, widget::{column, container, text}, Application, Command, Element, Settings, Theme};
pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    
}

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
        (
            Self {
                ..Self::default()
            },
            Command::none(),
        )
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

        let content = column![
            text(String::from("Hello World")),
        ];

        container(content).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}