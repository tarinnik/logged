use async_channel::{bounded, Receiver, Sender};
use iced::{
    futures::{channel::mpsc::Sender as FuturesSender, SinkExt, Stream},
    stream,
};
use notify::{recommended_watcher, Event, RecommendedWatcher};
use std::path::PathBuf;

pub fn watch() -> impl Stream<Item = WatcherEvent> {
    stream::channel(100, |mut output| async move {
        let mut state = State::Inactive;

        loop {
            match state {
                State::Inactive => match start_watcher(&mut output).await {
                    Ok(s) => state = s,
                    Err(e) => {
                        let _ = output.send(WatcherEvent::WatcherInactive(e)).await;
                    }
                },
                State::Active(ref watcher) => {}
            }
        }
    })
}

async fn start_watcher(output: &mut FuturesSender<WatcherEvent>) -> anyhow::Result<State> {
    let (event_sender, event_receiver) = bounded::<notify::Result<Event>>(1000);
    let (command_sender, command_receiver) = bounded::<WatcherCommand>(50);
    let watcher = recommended_watcher(move |e| {
        let _ = event_sender.send_blocking(e);
    })?;

    output
        .send(WatcherEvent::WatcherActive(command_sender))
        .await?;

    Ok(State::Active(Watcher {
        event_receiver,
        command_receiver,
        watcher,
    }))
}

struct Watcher {
    event_receiver: Receiver<notify::Result<Event>>,
    command_receiver: Receiver<WatcherCommand>,
    watcher: RecommendedWatcher,
}

#[derive(Clone, Debug)]
pub enum WatcherEvent {
    WatcherInactive(anyhow::Error),
    WatcherActive(Sender<WatcherCommand>),
    NewLog,
}

pub enum WatcherCommand {
    Watch(PathBuf),
    Unwatch(PathBuf),
}

enum State {
    Inactive,
    Active(Watcher),
}
