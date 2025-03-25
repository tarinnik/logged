mod watcher;

use crate::watcher::WatcherEvent;
use iced::{widget::text, Element, Subscription, Theme};

fn main() -> iced::Result {
    iced::application("logged", App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
}

#[derive(Default)]
struct App {}

impl App {
    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<Message> {
        text("logged").into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(watcher::watch).map(Message::WatcherEvent)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    WatcherEvent(WatcherEvent),
}
