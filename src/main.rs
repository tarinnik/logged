mod file;
mod ui;
mod util;
mod watcher;

use crate::ui::View;
use iced::{widget::text, Element, Subscription, Task, Theme};
use ui::{log_view::LogViewMessage, Views};

fn main() -> iced::Result {
    iced::application("logged", App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
}

#[derive(Default)]
struct App {
    view: View,
    views: Views,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LogViewMessage(log_message) => self.views.log.update(log_message),
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.view {
            View::Log => self.views.log.view(),
            View::Settings => text("settings").into(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(watcher::watch)
            .map(|e| Message::LogViewMessage(LogViewMessage::WatcherEvent(e)))
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    LogViewMessage(LogViewMessage),
    SettingsViewMessage,
}
