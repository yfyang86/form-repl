// Configuration module for FORM REPL settings
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub settings: Settings,
    pub history: HistoryConfig,
}

/// General settings
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Enable syntax highlighting
    pub highlight: bool,
    /// Theme name
    pub theme: String,
    /// Show timing information
    pub show_timing: bool,
    /// Verbose debug output
    pub verbose: bool,
    /// Auto-add .end to submissions
    pub auto_end: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            highlight: true,
            theme: "default".to_string(),
            show_timing: false,
            verbose: false,
            auto_end: true,
        }
    }
}

/// History configuration
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HistoryConfig {
    /// Path to history file (supports ~ expansion)
    pub file: String,
    /// Maximum history entries to keep
    pub max_entries: usize,
    /// Save history on exit
    pub save_on_exit: bool,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        HistoryConfig {
            file: "~/.form_repl_history".to_string(),
            max_entries: 1000,
            save_on_exit: true,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Self {
        // Try to load from multiple locations
        let config_paths = [
            // Current directory
            PathBuf::from(".form_replrc"),
            PathBuf::from(".form_repl.toml"),
            // Home directory
            dirs::home_dir()
                .map(|h| h.join(".form_replrc"))
                .unwrap_or_default(),
            dirs::home_dir()
                .map(|h| h.join(".config/form-repl/config.toml"))
                .unwrap_or_default(),
        ];
        
        for path in &config_paths {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    match toml::from_str(&content) {
                        Ok(config) => {
                            return config;
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to parse config at {}: {}", 
                                path.display(), e);
                        }
                    }
                }
            }
        }
        
        // Return default config if no file found
        Config::default()
    }
    
    /// Get the expanded history file path
    pub fn history_path(&self) -> PathBuf {
        expand_path(&self.history.file)
    }
}

/// Expand ~ in paths to home directory
pub fn expand_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[1..].trim_start_matches('/'));
        }
    }
    PathBuf::from(path)
}

/// Generate a sample configuration file content
pub fn sample_config() -> &'static str {
    r#"# FORM REPL Configuration File
# Place this file at ~/.form_replrc or ./.form_replrc

[settings]
# Enable syntax highlighting (default: true)
highlight = true

# Color theme: default, solarized-dark, monokai, dracula, nord, gruvbox, one-dark
theme = "dracula"

# Show timing information after each execution
show_timing = false

# Verbose debug output
verbose = false

# Automatically add .end to submissions
auto_end = true

[history]
# History file location (supports ~ for home directory)
file = "~/.form_repl_history"

# Maximum history entries to keep
max_entries = 1000

# Save history when exiting
save_on_exit = true
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.settings.highlight);
        assert_eq!(config.settings.theme, "default");
        assert_eq!(config.history.max_entries, 1000);
    }
    
    #[test]
    fn test_expand_path() {
        let path = expand_path("~/.form_repl_history");
        assert!(!path.to_string_lossy().contains('~'));
    }
    
    #[test]
    fn test_parse_config() {
        let config_str = r#"
[settings]
highlight = false
theme = "monokai"
"#;
        let config: Config = toml::from_str(config_str).unwrap();
        assert!(!config.settings.highlight);
        assert_eq!(config.settings.theme, "monokai");
    }
}
