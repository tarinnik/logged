pub trait FormattedError {
    fn format(&self) -> String;
}

impl FormattedError for anyhow::Error {
    fn format(&self) -> String {
        format!("{:#}", self)
    }
}
