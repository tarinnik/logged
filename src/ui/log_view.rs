use crate::{
    file::{read_file, read_new_data, LogData},
    util::Formatted,
    watcher::{WatcherCommand, WatcherEvent},
    Message,
};
use iced::{
    futures::channel::mpsc::Sender,
    widget::{
        button,
        button::{Catalog, Status},
        column, container,
        container::Style,
        row, text,
    },
    Color, Element, Length, Task, Theme,
};
use notify::{Event, EventKind};
use rfd::{AsyncFileDialog, FileHandle};
use std::path::PathBuf;

#[derive(Default)]
pub struct LogView {
    watcher_sender: Option<Sender<WatcherCommand>>,
    data: Vec<LogData>,
    selected_tab: Option<PathBuf>,
}

impl LogView {
    pub fn view(&self) -> Element<Message> {
        let mut tabs = row![];
        let mut logs = column![];

        if let Some(selected_tab) = &self.selected_tab {
            for tab_data in &self.data {
                tabs = tabs.push(tab_button(&tab_data.path));
                if *selected_tab == tab_data.path {
                    for line in &tab_data.contents {
                        logs = logs.push(text(line));
                    }
                }
            }
        }

        tabs = tabs.push(button("+").on_press(Message::LogViewMessage(LogViewMessage::PickFile)));

        column![tabs, container(logs).padding(10)].into()
    }

    pub fn update(&mut self, message: LogViewMessage) -> Task<Message> {
        match message {
            LogViewMessage::WatcherEvent(event) => self.handle_watcher_event(event),
            LogViewMessage::PickFile => Task::perform(pick_file(), |x| {
                Message::LogViewMessage(LogViewMessage::FilePicked(x))
            }),
            LogViewMessage::FilePicked(file) => {
                if let Some(file) = file {
                    self.selected_tab = Some(file.path().to_owned());
                    Task::perform(read_file(file.path().to_owned()), |x| {
                        let result = x.map_err(|e| e.format());
                        Message::LogViewMessage(LogViewMessage::FileRead(result))
                    })
                } else {
                    Task::none()
                }
            }
            LogViewMessage::FileRead(data) => match data {
                Ok(data) => {
                    if let Some(sender) = &mut self.watcher_sender {
                        let _ = sender.try_send(WatcherCommand::Watch(data.path.clone()));
                    }
                    self.data.push(data);
                    Task::none()
                }
                Err(err) => {
                    self.display_error(err);
                    Task::none()
                }
            },
            LogViewMessage::FileUpdated(update) => {
                match update {
                    Ok(update) => {
                        for data in self.data.iter_mut() {
                            if data.path == update.path {
                                data.append(update);
                                break;
                            }
                        }
                    }
                    Err(err) => self.display_error(err),
                };
                Task::none()
            }
            LogViewMessage::ChangeTab(tab) => {
                self.selected_tab = Some(tab);
                Task::none()
            }
            LogViewMessage::CloseTab(tab) => {
                let index = self.data.iter().position(|x| x.path == tab);
                if let Some(i) = index {
                    self.data.remove(i);
                    self.selected_tab = self.data.first().map(|x| x.path.clone());
                }
                Task::none()
            }
        }
    }

    fn handle_watcher_event(&mut self, event: WatcherEvent) -> Task<Message> {
        match event {
            WatcherEvent::WatcherActive(sender) => {
                self.watcher_sender = Some(sender);
                Task::none()
            }
            WatcherEvent::WatcherInactive(_) => {
                self.watcher_sender = None;
                Task::none()
            }
            WatcherEvent::NewLog(event) => match event {
                Ok(event) => self.handle_file_event(event),
                Err(err) => {
                    self.display_error(err);
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn handle_file_event(&mut self, event: Event) -> Task<Message> {
        match event.kind {
            EventKind::Modify(_) => {
                for data in &self.data {
                    if event.paths.contains(&data.path) {
                        return Task::perform(
                            read_new_data(data.path.clone(), data.position),
                            |x| {
                                let result = x.map_err(|e| e.format());
                                Message::LogViewMessage(LogViewMessage::FileUpdated(result))
                            },
                        );
                    }
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn display_error(&self, error: String) {
        eprintln!("{}", error);
    }
}

async fn pick_file() -> Option<FileHandle> {
    AsyncFileDialog::new().pick_file().await
}

#[derive(Clone, Debug)]
pub enum LogViewMessage {
    PickFile,
    FilePicked(Option<FileHandle>),
    WatcherEvent(WatcherEvent),
    FileRead(Result<LogData, String>),
    FileUpdated(Result<LogData, String>),
    ChangeTab(PathBuf),
    CloseTab(PathBuf),
}

fn tab_button(path: &PathBuf) -> Element<Message> {
    let file_name = path
        .as_path()
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or(String::from("N/A"));

    let background = tab_button_style;

    let close_button = button("x")
        .on_press(Message::LogViewMessage(LogViewMessage::CloseTab(
            path.clone(),
        )))
        .style(close_button_style)
        .padding(0);

    button(
        container(row![
            container(text(file_name)).align_left(Length::Fill),
            container(close_button).align_right(Length::Fill),
        ])
        .style(background)
        .padding(10)
        .width(150),
    )
    .padding(0)
    .on_press(Message::LogViewMessage(LogViewMessage::ChangeTab(
        path.clone(),
    )))
    .into()
}

fn tab_button_style(_theme: &Theme) -> Style {
    Style::default().background(Color::from_rgb8(15, 15, 15))
}

fn close_button_style(_theme: &Theme, _status: Status) -> iced::widget::button::Style {
    let mut style = iced::widget::button::Style::default();
    style.text_color = Color::WHITE;
    style
}
