use crate::util::Formatted;
use anyhow::Context;
use std::{io::SeekFrom, path::PathBuf};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};

/// Stores the data for a log file
#[derive(Clone, Debug)]
pub struct LogData {
    pub path: PathBuf,
    pub contents: Vec<String>,
    pub position: u64,
}

impl LogData {
    pub fn new(path: PathBuf, data: String, position: u64) -> Self {
        let contents = data.trim().split('\n').map(String::from).collect();

        Self {
            path,
            contents,
            position,
        }
    }

    pub fn append(&mut self, mut other: LogData) {
        self.contents.append(&mut other.contents);
        self.position = other.position;
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
    println!("Yep re-reading the file buddy");
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

    println!("New data: {}", &new_data);

    Ok(LogData::new(path, new_data, new_position))
}
