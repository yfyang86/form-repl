// Terminal utilities
use std::sync::atomic::{AtomicBool, Ordering};

/// Thread-safe verbose flag using AtomicBool
/// This prevents data races and follows Rust's safety guarantees
pub static VERBOSE: AtomicBool = AtomicBool::new(false);

/// Check if verbose mode is enabled
#[inline]
pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

/// Set verbose mode
#[inline]
pub fn set_verbose(enabled: bool) {
    VERBOSE.store(enabled, Ordering::Relaxed);
}

/// Print message only if verbose mode is enabled
#[inline]
pub fn verbose_println(msg: &str) {
    if is_verbose() {
        eprintln!("[verbose] {}", msg);
    }
}

/// Macro for conditional verbose printing with formatting
#[macro_export]
macro_rules! vprintln {
    () => {
        if $crate::modules::term::is_verbose() {
            eprintln!();
        }
    };
    ($($arg:tt)*) => {
        if $crate::modules::term::is_verbose() {
            eprintln!("[verbose] {}", format!($($arg)*));
        }
    };
}

/// ANSI escape code utilities
pub mod ansi {
    /// Reset all attributes
    pub const RESET: &str = "\x1b[0m";
    /// Bold text
    pub const BOLD: &str = "\x1b[1m";
    /// Dim text
    pub const DIM: &str = "\x1b[2m";
    /// Italic text
    pub const ITALIC: &str = "\x1b[3m";
    /// Underline text
    pub const UNDERLINE: &str = "\x1b[4m";
    
    /// Clear the current line
    pub const CLEAR_LINE: &str = "\x1b[2K";
    /// Move cursor to beginning of line
    pub const LINE_START: &str = "\r";
    /// Move cursor up one line
    pub const CURSOR_UP: &str = "\x1b[A";
    
    /// Check if stdout is a terminal
    pub fn is_tty() -> bool {
        use std::io::IsTerminal;
        std::io::stdout().is_terminal()
    }
    
    /// Get terminal width (returns 80 as default if unable to determine)
    pub fn terminal_width() -> usize {
        // Try to get terminal size using a simple method
        // In a real implementation, you might use the `terminal_size` crate
        80
    }
}

/// Format duration for display
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs_f64();
    if secs < 0.001 {
        format!("{:.2}µs", secs * 1_000_000.0)
    } else if secs < 1.0 {
        format!("{:.2}ms", secs * 1000.0)
    } else if secs < 60.0 {
        format!("{:.2}s", secs)
    } else {
        let mins = (secs / 60.0).floor() as u64;
        let remaining_secs = secs - (mins as f64 * 60.0);
        format!("{}m {:.1}s", mins, remaining_secs)
    }
}

/// Horizontal separator line
pub fn separator(width: usize, colored: bool, color: &str) -> String {
    let line: String = "─".repeat(width);
    if colored && !color.is_empty() {
        format!("{}{}{}", color, line, ansi::RESET)
    } else {
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_format_duration() {
        assert!(format_duration(Duration::from_micros(500)).contains("µs"));
        assert!(format_duration(Duration::from_millis(500)).contains("ms"));
        assert!(format_duration(Duration::from_secs(30)).contains("s"));
        assert!(format_duration(Duration::from_secs(90)).contains("m"));
    }
    
    #[test]
    fn test_separator() {
        let sep = separator(10, false, "");
        assert_eq!(sep.chars().count(), 10);
    }
}
