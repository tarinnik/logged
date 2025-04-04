use crate::{
    filter::{Filters, LogLevel},
    util::Formatted,
};
use anyhow::Context;
use iced::Color;
use std::{io::SeekFrom, path::PathBuf};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};

/// Stores the data for a log file
#[derive(Clone, Debug)]
pub struct LogData {
    pub path: PathBuf,
    pub contents: Vec<LogLine>,
    pub position: u64,
    pub filters: Filters,
}

impl LogData {
    pub fn new(path: PathBuf, data: String, position: u64) -> Self {
        let contents = data.trim().split('\n').map(LogLine::new).collect();

        Self {
            path,
            contents,
            position,
            filters: Filters::default(),
        }
    }

    pub fn append(&mut self, mut other: LogData) {
        other.filters = self.filters.clone();
        other.filter_all();
        self.contents.append(&mut other.contents);
        self.position = other.position;
    }

    /// Applies the filters to all the logs
    pub fn filter_all(&mut self) {
        // If a line doesn't match a level filter, use the last one
        let mut last_filter: Option<&LogLevel> = None;

        'log: for log in &mut self.contents {
            for filter in &self.filters.levels {
                for pattern in filter.pattern.split(" ") {
                    if log.text.contains(pattern) {
                        log.apply_filter(filter);
                        last_filter = Some(filter);
                        continue 'log;
                    }
                }

                // Log doesn't have a level, use the last one
                if let Some(last_filter) = last_filter {
                    log.apply_filter(last_filter);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogLine {
    pub text: String,
    pub foreground: Color,
    pub background: Color,
    pub visible: bool,
}

impl LogLine {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            foreground: Color::WHITE,
            background: Color::TRANSPARENT,
            visible: true,
        }
    }

    pub fn apply_filter(&mut self, filter: &LogLevel) {
        self.visible = filter.enabled;
        self.background = filter.background;
        self.foreground = filter.foreground;
    }
}

/// Read a new log file
pub async fn read_file(path: PathBuf) -> anyhow::Result<LogData> {
    let mut file = File::open(&path)
        .await
        .with_context(|| format!("Unable to open file {}", path.format()))?;

    let mut data = String::new();
    file.read_to_string(&mut data)
        .await
        .with_context(|| format!("Unable to read file {}", path.format()))?;

    let position = file
        .stream_position()
        .await
        .with_context(|| format!("Unable to get stream position of {}", path.format()))?;

    Ok(LogData::new(path.clone(), data, position))
}

/// Read the new data from a log file
pub async fn read_new_data(path: PathBuf, position: u64) -> anyhow::Result<LogData> {
    let mut file = File::open(&path)
        .await
        .with_context(|| format!("Unable to open file {}", path.format()))?;

    file.seek(SeekFrom::Start(position))
        .await
        .with_context(|| {
            format!(
                "Unable to seek to position {} on file {}",
                position,
                &path.format()
            )
        })?;

    let mut new_data = String::new();
    file.read_to_string(&mut new_data)
        .await
        .with_context(|| format!("Unable to read file {}", path.format()))?;

    let new_position = file
        .stream_position()
        .await
        .with_context(|| format!("Unable to get stream position of {}", path.format()))?;

    Ok(LogData::new(path, new_data, new_position))
}
