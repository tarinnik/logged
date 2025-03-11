use iced::{futures::Stream, stream};

pub fn watch() -> impl Stream<Item = Event> {
    stream::channel(100, |mut output| async move {
        let mut state = State::Inactive;

        loop {
            match state {
                State::Inactive => {}
                State::Active(ref watcher) => {}
            }
        }
    })
}

struct Watcher {}

#[derive(Clone, Debug)]
pub enum Event {
    WatcherInactive,
    WatcherActive,
    NewLog,
}

enum State {
    Inactive,
    Active(Watcher),
}
