use iced::Color;

#[derive(Clone, Debug)]
pub struct Filters {
    pub levels: Vec<LogLevel>,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            levels: vec![
                LogLevel::new(
                    "Verbose".into(),
                    "VERBOSE TRACE".into(),
                    Color::from_rgb8(2, 51, 0),
                    Color::from_rgb8(154, 249, 149),
                ),
                LogLevel::new(
                    "Debug".into(),
                    "DEBUG".into(),
                    Color::from_rgb8(0, 32, 66),
                    Color::from_rgb8(141, 195, 252),
                ),
                LogLevel::new(
                    "Info".into(),
                    "INFO INFORMATION".into(),
                    Color::WHITE,
                    Color::TRANSPARENT,
                ),
                LogLevel::new(
                    "Warn".into(),
                    "WARN WARNING".into(),
                    Color::from_rgb8(79, 72, 0),
                    Color::from_rgb8(249, 238, 109),
                ),
                LogLevel::new(
                    "Error".into(),
                    "ERROR".into(),
                    Color::from_rgb8(51, 1, 1),
                    Color::from_rgb8(252, 116, 116),
                ),
            ],
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogLevel {
    pub name: String,
    pub pattern: String,
    pub enabled: bool,
    pub foreground: Color,
    pub background: Color,
}

impl LogLevel {
    pub fn new(name: String, pattern: String, foreground: Color, background: Color) -> Self {
        Self {
            name,
            pattern,
            enabled: true,
            foreground,
            background,
        }
    }
}
