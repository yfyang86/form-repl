// Theme definitions for syntax highlighting

/// Color theme for REPL output
/// Contains ANSI escape codes for different output types
pub struct Theme {
    pub prompt: String,
    pub output: String,
    pub error: String,
}

impl Default for Theme {
    /// Default theme with no colors (monochrome)
    fn default() -> Self {
        Theme {
            prompt: String::new(),
            output: String::new(),
            error: String::new(),
        }
    }
}

impl Theme {

    /// Solarized Dark theme
    pub fn solarized_dark() -> Self {
        Theme {
            prompt: String::from("\x1b[34m"),  // Blue
            output: String::from("\x1b[32m"), // Green
            error: String::from("\x1b[31m"),  // Red
        }
    }

    /// Monokai theme
    pub fn monokai() -> Self {
        Theme {
            prompt: String::from("\x1b[38;5;208m"), // Orange
            output: String::from("\x1b[38;5;142m"), // Light green
            error: String::from("\x1b[38;5;168m"), // Pink
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        Theme {
            prompt: String::from("\x1b[38;5;140m"), // Purple
            output: String::from("\x1b[38;5;84m"), // Green
            error: String::from("\x1b[38;5;210m"), // Pink
        }
    }
}

/// Gets a theme by name.
///
/// # Arguments
///
/// * `name` - Theme name (case-insensitive): "default", "solarized-dark", "monokai", or "dracula"
///
/// # Returns
///
/// The requested theme, or default theme if name is unrecognized.
pub fn get_theme(name: &str) -> Theme {
    match name.to_lowercase().as_str() {
        "solarized-dark" | "solarized" | "solarized_dark" => Theme::solarized_dark(),
        "monokai" => Theme::monokai(),
        "dracula" => Theme::dracula(),
        _ => Theme::default(),
    }
}
