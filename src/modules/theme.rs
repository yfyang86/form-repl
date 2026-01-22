// Theme definitions for syntax highlighting

pub const FORM_PROMPT: &str = "form > ";
pub const CONTINUATION_PROMPT: &str = " ...> ";

// Horizontal line separator
pub const SEPARATOR: &str = "────────────────────────────────────────";

// Session separator with line break
pub const SESSION_SEPARATOR: &str = "\r\n────────────────────────────────────────\r\n";

pub struct Theme {
    pub prompt: String,
    pub continuation: String,
    pub output: String,
    pub error: String,
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            prompt: String::from(""),
            continuation: String::from(""),
            output: String::from(""),
            error: String::from(""),
        }
    }

    pub fn solarized_dark() -> Self {
        Theme {
            prompt: String::from("\x1b[34m"),  // Blue
            continuation: String::from("\x1b[36m"), // Cyan
            output: String::from("\x1b[32m"), // Green
            error: String::from("\x1b[31m"),  // Red
        }
    }

    pub fn monokai() -> Self {
        Theme {
            prompt: String::from("\x1b[38;5;208m"), // Orange
            continuation: String::from("\x1b[38;5;136m"), // Yellow
            output: String::from("\x1b[38;5;142m"), // Light green
            error: String::from("\x1b[38;5;168m"), // Pink
        }
    }

    pub fn dracula() -> Self {
        Theme {
            prompt: String::from("\x1b[38;5;140m"), // Purple
            continuation: String::from("\x1b[38;5;139m"), // Dark purple
            output: String::from("\x1b[38;5;84m"), // Green
            error: String::from("\x1b[38;5;210m"), // Pink
        }
    }
}

pub fn get_theme(name: &str) -> Theme {
    match name.to_lowercase().as_str() {
        "solarized-dark" | "solarized" | "solarized_dark" => Theme::solarized_dark(),
        "monokai" => Theme::monokai(),
        "dracula" => Theme::dracula(),
        _ => Theme::default(),
    }
}
