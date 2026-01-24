// Theme definitions for syntax highlighting
use serde::Deserialize;

/// Color theme for REPL output and syntax highlighting
/// Contains ANSI escape codes for different token types
#[derive(Clone, Debug)]
pub struct Theme {
    // UI colors
    pub prompt_in: String,      // Input prompt color
    pub prompt_out: String,     // Output prompt color
    pub prompt_cont: String,    // Continuation prompt color
    pub separator: String,      // Separator line color
    pub error: String,          // Error message color
    pub timing: String,         // Timing info color
    pub output_label: String,   // Output label (e.g., "E =")
    
    // Syntax highlighting colors
    pub keyword: String,        // Keywords (id, repeat, if, etc.)
    pub declaration: String,    // Declarations (Symbol, Local, etc.)
    pub function: String,       // Built-in functions
    pub preprocessor: String,   // Preprocessor (#, .end, etc.)
    pub number: String,         // Numbers
    pub operator: String,       // Operators
    pub comment: String,        // Comments
    pub string: String,         // String literals
    pub identifier: String,     // User identifiers
}

impl Default for Theme {
    /// Default theme - subtle colors
    fn default() -> Self {
        Theme {
            prompt_in: String::from("\x1b[38;5;39m"),    // Bright blue
            prompt_out: String::from("\x1b[38;5;208m"),  // Orange
            prompt_cont: String::from("\x1b[38;5;242m"), // Gray
            separator: String::from("\x1b[38;5;240m"),   // Dark gray
            error: String::from("\x1b[38;5;196m"),       // Red
            timing: String::from("\x1b[38;5;242m"),      // Gray
            output_label: String::from("\x1b[38;5;81m"), // Cyan
            
            keyword: String::from("\x1b[38;5;207m"),     // Magenta
            declaration: String::from("\x1b[38;5;39m"),  // Blue
            function: String::from("\x1b[38;5;221m"),    // Yellow
            preprocessor: String::from("\x1b[38;5;208m"),// Orange
            number: String::from("\x1b[38;5;147m"),      // Light purple
            operator: String::from("\x1b[38;5;251m"),    // Light gray
            comment: String::from("\x1b[38;5;242m\x1b[3m"), // Gray italic
            string: String::from("\x1b[38;5;113m"),      // Green
            identifier: String::new(),                   // No color (default)
        }
    }
}

impl Theme {
    /// No colors (plain text)
    pub fn none() -> Self {
        Theme {
            prompt_in: String::new(),
            prompt_out: String::new(),
            prompt_cont: String::new(),
            separator: String::new(),
            error: String::new(),
            timing: String::new(),
            output_label: String::new(),
            
            keyword: String::new(),
            declaration: String::new(),
            function: String::new(),
            preprocessor: String::new(),
            number: String::new(),
            operator: String::new(),
            comment: String::new(),
            string: String::new(),
            identifier: String::new(),
        }
    }

    /// Solarized Dark theme
    pub fn solarized_dark() -> Self {
        Theme {
            prompt_in: String::from("\x1b[38;5;33m"),    // Blue
            prompt_out: String::from("\x1b[38;5;136m"),  // Yellow
            prompt_cont: String::from("\x1b[38;5;240m"), // Base01
            separator: String::from("\x1b[38;5;239m"),   // Base02
            error: String::from("\x1b[38;5;160m"),       // Red
            timing: String::from("\x1b[38;5;240m"),      // Base01
            output_label: String::from("\x1b[38;5;37m"), // Cyan
            
            keyword: String::from("\x1b[38;5;125m"),     // Magenta
            declaration: String::from("\x1b[38;5;33m"),  // Blue
            function: String::from("\x1b[38;5;166m"),    // Orange
            preprocessor: String::from("\x1b[38;5;136m"),// Yellow
            number: String::from("\x1b[38;5;37m"),       // Cyan
            operator: String::from("\x1b[38;5;245m"),    // Base0
            comment: String::from("\x1b[38;5;240m\x1b[3m"), // Base01 italic
            string: String::from("\x1b[38;5;64m"),       // Green
            identifier: String::new(),
        }
    }

    /// Monokai theme
    pub fn monokai() -> Self {
        Theme {
            prompt_in: String::from("\x1b[38;5;81m"),    // Cyan
            prompt_out: String::from("\x1b[38;5;208m"),  // Orange
            prompt_cont: String::from("\x1b[38;5;242m"), // Gray
            separator: String::from("\x1b[38;5;239m"),   // Dark gray
            error: String::from("\x1b[38;5;197m"),       // Pink-red
            timing: String::from("\x1b[38;5;242m"),      // Gray
            output_label: String::from("\x1b[38;5;81m"), // Cyan
            
            keyword: String::from("\x1b[38;5;197m"),     // Pink
            declaration: String::from("\x1b[38;5;81m"),  // Cyan
            function: String::from("\x1b[38;5;148m"),    // Green
            preprocessor: String::from("\x1b[38;5;208m"),// Orange
            number: String::from("\x1b[38;5;141m"),      // Purple
            operator: String::from("\x1b[38;5;197m"),    // Pink
            comment: String::from("\x1b[38;5;242m\x1b[3m"), // Gray italic
            string: String::from("\x1b[38;5;186m"),      // Yellow
            identifier: String::from("\x1b[38;5;231m"), // White
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        Theme {
            prompt_in: String::from("\x1b[38;5;141m"),   // Purple
            prompt_out: String::from("\x1b[38;5;84m"),   // Green
            prompt_cont: String::from("\x1b[38;5;61m"),  // Comment purple
            separator: String::from("\x1b[38;5;61m"),    // Comment
            error: String::from("\x1b[38;5;210m"),       // Red
            timing: String::from("\x1b[38;5;61m"),       // Comment
            output_label: String::from("\x1b[38;5;117m"),// Cyan
            
            keyword: String::from("\x1b[38;5;212m"),     // Pink
            declaration: String::from("\x1b[38;5;117m"), // Cyan
            function: String::from("\x1b[38;5;84m"),     // Green
            preprocessor: String::from("\x1b[38;5;215m"),// Orange
            number: String::from("\x1b[38;5;141m"),      // Purple
            operator: String::from("\x1b[38;5;212m"),    // Pink
            comment: String::from("\x1b[38;5;61m\x1b[3m"), // Comment italic
            string: String::from("\x1b[38;5;228m"),      // Yellow
            identifier: String::from("\x1b[38;5;231m"), // Foreground
        }
    }
    
    /// Nord theme
    pub fn nord() -> Self {
        Theme {
            prompt_in: String::from("\x1b[38;5;110m"),   // Nord9 (blue)
            prompt_out: String::from("\x1b[38;5;180m"), // Nord13 (yellow)
            prompt_cont: String::from("\x1b[38;5;60m"), // Nord3
            separator: String::from("\x1b[38;5;60m"),   // Nord3
            error: String::from("\x1b[38;5;167m"),      // Nord11 (red)
            timing: String::from("\x1b[38;5;60m"),      // Nord3
            output_label: String::from("\x1b[38;5;109m"),// Nord8 (cyan)
            
            keyword: String::from("\x1b[38;5;139m"),    // Nord15 (purple)
            declaration: String::from("\x1b[38;5;110m"),// Nord9 (blue)
            function: String::from("\x1b[38;5;109m"),   // Nord8 (cyan)
            preprocessor: String::from("\x1b[38;5;180m"),// Nord13 (yellow)
            number: String::from("\x1b[38;5;139m"),     // Nord15 (purple)
            operator: String::from("\x1b[38;5;109m"),   // Nord8
            comment: String::from("\x1b[38;5;60m\x1b[3m"), // Nord3 italic
            string: String::from("\x1b[38;5;150m"),     // Nord14 (green)
            identifier: String::from("\x1b[38;5;254m"),// Nord6 (white)
        }
    }

    /// Gruvbox Dark theme
    pub fn gruvbox() -> Self {
        Theme {
            prompt_in: String::from("\x1b[38;5;109m"),   // Blue
            prompt_out: String::from("\x1b[38;5;214m"),  // Orange
            prompt_cont: String::from("\x1b[38;5;245m"), // Gray
            separator: String::from("\x1b[38;5;239m"),   // Dark gray
            error: String::from("\x1b[38;5;167m"),       // Red
            timing: String::from("\x1b[38;5;245m"),      // Gray
            output_label: String::from("\x1b[38;5;108m"),// Aqua
            
            keyword: String::from("\x1b[38;5;167m"),     // Red
            declaration: String::from("\x1b[38;5;214m"), // Orange
            function: String::from("\x1b[38;5;142m"),    // Green
            preprocessor: String::from("\x1b[38;5;175m"),// Purple
            number: String::from("\x1b[38;5;175m"),      // Purple
            operator: String::from("\x1b[38;5;223m"),    // Light
            comment: String::from("\x1b[38;5;245m\x1b[3m"), // Gray italic
            string: String::from("\x1b[38;5;142m"),      // Green
            identifier: String::from("\x1b[38;5;223m"), // Light
        }
    }
    
    /// One Dark theme (Atom-inspired)
    pub fn one_dark() -> Self {
        Theme {
            prompt_in: String::from("\x1b[38;5;39m"),    // Blue
            prompt_out: String::from("\x1b[38;5;209m"),  // Orange
            prompt_cont: String::from("\x1b[38;5;241m"), // Comment
            separator: String::from("\x1b[38;5;238m"),   // Gutter
            error: String::from("\x1b[38;5;204m"),       // Red
            timing: String::from("\x1b[38;5;241m"),      // Comment
            output_label: String::from("\x1b[38;5;38m"), // Cyan
            
            keyword: String::from("\x1b[38;5;176m"),     // Purple
            declaration: String::from("\x1b[38;5;39m"),  // Blue
            function: String::from("\x1b[38;5;38m"),     // Cyan
            preprocessor: String::from("\x1b[38;5;209m"),// Orange
            number: String::from("\x1b[38;5;209m"),      // Orange
            operator: String::from("\x1b[38;5;176m"),    // Purple
            comment: String::from("\x1b[38;5;241m\x1b[3m"), // Gray italic
            string: String::from("\x1b[38;5;113m"),      // Green
            identifier: String::from("\x1b[38;5;204m"), // Red (for contrast)
        }
    }
}

/// Configuration for theme from TOML file
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ThemeConfig {
    pub name: String,
}

/// Gets a theme by name.
///
/// # Arguments
///
/// * `name` - Theme name (case-insensitive)
///
/// # Returns
///
/// The requested theme, or default theme if name is unrecognized.
pub fn get_theme(name: &str) -> Theme {
    match name.to_lowercase().as_str() {
        "none" | "plain" | "no-color" => Theme::none(),
        "solarized-dark" | "solarized" | "solarized_dark" => Theme::solarized_dark(),
        "monokai" => Theme::monokai(),
        "dracula" => Theme::dracula(),
        "nord" => Theme::nord(),
        "gruvbox" | "gruvbox-dark" => Theme::gruvbox(),
        "one-dark" | "one_dark" | "onedark" | "atom" => Theme::one_dark(),
        _ => Theme::default(),
    }
}

/// List all available themes
pub fn list_themes() -> Vec<&'static str> {
    vec![
        "default",
        "none",
        "solarized-dark",
        "monokai",
        "dracula",
        "nord",
        "gruvbox",
        "one-dark",
    ]
}
