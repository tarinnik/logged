use std::path::PathBuf;

pub trait Formatted {
    fn format(&self) -> String;
}

impl Formatted for anyhow::Error {
    fn format(&self) -> String {
        format!("{:#}", self)
    }
}

impl Formatted for PathBuf {
    fn format(&self) -> String {
        self.as_path().to_string_lossy().into()
    }
}
