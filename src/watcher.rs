use crate::util::Formatted;
use anyhow::Context;
use iced::{
    futures::{
        channel::mpsc::{channel, Receiver, Sender},
        select, SinkExt, Stream, StreamExt,
    },
    stream,
};
use notify::{recommended_watcher, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

pub fn watch() -> impl Stream<Item = WatcherEvent> {
    stream::channel(100, |mut output| async move {
        let mut state = State::Inactive;

        loop {
            match state {
                State::Inactive => match start_watcher(&mut output).await {
                    Ok(s) => state = s,
                    Err(e) => {
                        let _ = output.send(WatcherEvent::WatcherInactive(e.format())).await;
                    }
                },
                State::Active(ref mut watcher) => {
                    select! {
                        event = watcher.event_receiver.next() => {
                            if let Some(event) = event {
                                let _ = output.send(WatcherEvent::NewLog(event.context("Error getting event").map_err(|e|e.format()))).await;
                            }
                        }
                        command = watcher.command_receiver.next() => {
                            if let Some(command) = command {
                                watcher.handle_command(command, &mut output).await;
                            }
                        }
                    }
                }
            }
        }
    })
}

async fn start_watcher(output: &mut Sender<WatcherEvent>) -> anyhow::Result<State> {
    let (mut event_sender, event_receiver) = channel::<notify::Result<Event>>(1000);
    let (command_sender, command_receiver) = channel::<WatcherCommand>(50);
    let watcher = recommended_watcher(move |e| {
        println!("NEW EVENT: {:?}", &e);
        let _ = event_sender.try_send(e);
    })?;

    output
        .send(WatcherEvent::WatcherActive(command_sender))
        .await?;

    Ok(State::Active(FileWatcher {
        event_receiver,
        command_receiver,
        watcher,
    }))
}

struct FileWatcher {
    event_receiver: Receiver<notify::Result<Event>>,
    command_receiver: Receiver<WatcherCommand>,
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    async fn handle_command(&mut self, command: WatcherCommand, sender: &mut Sender<WatcherEvent>) {
        let result = match command {
            WatcherCommand::Watch(path) => self.watcher.watch(&path, RecursiveMode::NonRecursive),
            WatcherCommand::Unwatch(path) => self.watcher.unwatch(&path),
        }
        .context("Unable to watch file")
        .map_err(|e| e.format());

        let _ = sender.send(WatcherEvent::WatchResult(result)).await;
    }
}

#[derive(Clone, Debug)]
pub enum WatcherEvent {
    WatcherInactive(String),
    WatcherActive(Sender<WatcherCommand>),
    NewLog(Result<Event, String>),
    WatchResult(Result<(), String>),
}

pub enum WatcherCommand {
    Watch(PathBuf),
    Unwatch(PathBuf),
}

enum State {
    Inactive,
    Active(FileWatcher),
}
