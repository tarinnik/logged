use crate::{
    file::{read_file, read_new_data, LogData},
    util::Formatted,
    watcher::{WatcherCommand, WatcherEvent},
    Message,
};
use iced::{futures::channel::mpsc::Sender, widget::button, Element, Task};
use notify::{Event, EventKind};
use rfd::{AsyncFileDialog, FileHandle};

#[derive(Default)]
pub struct LogView {
    watcher_sender: Option<Sender<WatcherCommand>>,
    data: Vec<LogData>,
}

impl LogView {
    pub fn view(&self) -> Element<Message> {
        button("+")
            .on_press(Message::LogViewMessage(LogViewMessage::PickFile))
            .into()
    }

    pub fn update(&mut self, message: LogViewMessage) -> Task<Message> {
        match message {
            LogViewMessage::WatcherEvent(event) => self.handle_watcher_event(event),
            LogViewMessage::PickFile => Task::perform(pick_file(), |x| {
                Message::LogViewMessage(LogViewMessage::FilePicked(x))
            }),
            LogViewMessage::FilePicked(file) => {
                if let Some(file) = file {
                    Task::perform(read_file(file.path().to_owned()), |x| {
                        let result = x.map_err(|e| e.format());
                        Message::LogViewMessage(LogViewMessage::FileRead(result))
                    })
                } else {
                    Task::none()
                }
            }
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

            _ => Task::none(),
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

    fn display_error(&self, _error: String) {}
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
}
