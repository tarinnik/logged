use crate::Message;

use iced::{widget::button, Element, Task};
use rfd::{AsyncFileDialog, FileHandle};

#[derive(Default)]
pub struct LogView {}

impl LogView {
    pub fn view(&self) -> Element<Message> {
        button("+")
            .on_press(Message::LogViewMessage(LogViewMessage::PickFile))
            .into()
    }

    pub fn update(&self, message: LogViewMessage) -> Task<Message> {
        match message {
            LogViewMessage::PickFile => Task::perform(pick_file(), |x| {
                Message::LogViewMessage(LogViewMessage::FilePicked(x))
            }),
            _ => Task::none(),
        }
    }
}

async fn pick_file() -> Option<FileHandle> {
    AsyncFileDialog::new().set_directory("/").pick_file().await
}

#[derive(Clone, Debug)]
pub enum LogViewMessage {
    PickFile,
    FilePicked(Option<FileHandle>),
}
