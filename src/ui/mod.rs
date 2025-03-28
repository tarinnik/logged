pub mod log_view;

use log_view::LogView;

#[derive(Default)]
pub enum View {
    #[default]
    Log,
    Settings,
}

#[derive(Default)]
pub struct Views {
    pub log: LogView,
}
