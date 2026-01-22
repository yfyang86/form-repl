// Theme definitions for syntax highlighting

pub struct Theme {
    pub prompt: String,
    pub output: String,
    pub error: String,
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            prompt: String::from(""),
            output: String::from(""),
            error: String::from(""),
        }
    }

    pub fn solarized_dark() -> Self {
        Theme {
            prompt: String::from("\x1b[34m"),  // Blue
            output: String::from("\x1b[32m"), // Green
            error: String::from("\x1b[31m"),  // Red
        }
    }

    pub fn monokai() -> Self {
        Theme {
            prompt: String::from("\x1b[38;5;208m"), // Orange
            output: String::from("\x1b[38;5;142m"), // Light green
            error: String::from("\x1b[38;5;168m"), // Pink
        }
    }

    pub fn dracula() -> Self {
        Theme {
            prompt: String::from("\x1b[38;5;140m"), // Purple
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
