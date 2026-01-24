// Magic commands module - IPython-like functionality
use std::collections::VecDeque;
use std::time::Duration;

use super::theme;

/// Session history entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub number: usize,
    pub input: String,
    pub output: Option<String>,
    pub duration: Option<Duration>,
}

/// Session state for magic commands
pub struct SessionState {
    /// History of all inputs and outputs
    pub history: Vec<HistoryEntry>,
    /// Current session number
    pub session_number: usize,
    /// Last N outputs for _ and __ style access
    pub last_outputs: VecDeque<String>,
    /// Show timing by default
    pub show_timing: bool,
    /// Max outputs to keep for _ access
    max_outputs: usize,
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionState {
    pub fn new() -> Self {
        SessionState {
            history: Vec::new(),
            session_number: 1,
            last_outputs: VecDeque::with_capacity(10),
            show_timing: false,
            max_outputs: 10,
        }
    }
    
    /// Add a new history entry
    pub fn add_entry(&mut self, input: String, output: Option<String>, duration: Option<Duration>) {
        let entry = HistoryEntry {
            number: self.session_number,
            input,
            output: output.clone(),
            duration,
        };
        self.history.push(entry);
        
        // Track last outputs
        if let Some(out) = output {
            if !out.trim().is_empty() {
                if self.last_outputs.len() >= self.max_outputs {
                    self.last_outputs.pop_back();
                }
                self.last_outputs.push_front(out);
            }
        }
        
        self.session_number += 1;
    }
    
    /// Get the last output (_)
    pub fn last_output(&self) -> Option<&String> {
        self.last_outputs.front()
    }
    
    /// Get the second-to-last output (__)
    pub fn prev_output(&self) -> Option<&String> {
        self.last_outputs.get(1)
    }
    
    /// Get output by index (___N)
    pub fn output_at(&self, idx: usize) -> Option<&String> {
        self.last_outputs.get(idx)
    }
    
    /// Clear session state
    pub fn reset(&mut self) {
        self.history.clear();
        self.last_outputs.clear();
        self.session_number = 1;
    }
}

/// Magic command result
pub enum MagicResult {
    /// Command produced output to display
    Output(String),
    /// Command was handled, no output needed
    Handled,
    /// Not a magic command
    NotMagic,
    /// Error occurred
    Error(String),
    /// Exit requested
    Exit,
    /// Show help
    Help,
}

/// Process a magic command (starts with %)
pub fn process_magic(cmd: &str, state: &mut SessionState, highlight: bool, theme_name: &str) -> MagicResult {
    let trimmed = cmd.trim();
    
    if !trimmed.starts_with('%') {
        return MagicResult::NotMagic;
    }
    
    let parts: Vec<&str> = trimmed[1..].split_whitespace().collect();
    if parts.is_empty() {
        return MagicResult::Error("Empty magic command".to_string());
    }
    
    let magic_name = parts[0].to_lowercase();
    let args = &parts[1..];
    
    match magic_name.as_str() {
        "help" | "?" => MagicResult::Help,
        
        "quit" | "exit" | "q" => MagicResult::Exit,
        
        "history" | "hist" | "h" => {
            let n: usize = args.first()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10);
            MagicResult::Output(format_history(&state.history, n))
        }
        
        "reset" | "clear" => {
            state.reset();
            MagicResult::Output("Session reset. History cleared.".to_string())
        }
        
        "time" | "timeit" => {
            state.show_timing = !state.show_timing;
            MagicResult::Output(format!(
                "Timing display: {}",
                if state.show_timing { "ON" } else { "OFF" }
            ))
        }
        
        "who" | "whos" => {
            // List all declared symbols from history
            let symbols = extract_symbols(&state.history);
            if symbols.is_empty() {
                MagicResult::Output("No symbols declared in this session.".to_string())
            } else {
                MagicResult::Output(format!("Declared symbols: {}", symbols.join(", ")))
            }
        }
        
        "last" | "_" => {
            match state.last_output() {
                Some(out) => MagicResult::Output(out.clone()),
                None => MagicResult::Output("No output history.".to_string()),
            }
        }
        
        "recall" | "r" => {
            let n: usize = args.first()
                .and_then(|s| s.parse().ok())
                .unwrap_or(state.session_number.saturating_sub(1));
            
            if let Some(entry) = state.history.iter().find(|e| e.number == n) {
                MagicResult::Output(format!("In [{}]:\n{}", n, entry.input))
            } else {
                MagicResult::Error(format!("No entry found for session {}", n))
            }
        }
        
        "theme" | "themes" => {
            if args.is_empty() {
                let themes = theme::list_themes();
                let current = if highlight { theme_name } else { "disabled" };
                MagicResult::Output(format!(
                    "Available themes: {}\nCurrent: {}",
                    themes.join(", "),
                    current
                ))
            } else {
                MagicResult::Output(format!(
                    "Theme switching at runtime not yet supported.\nUse --theme {} at startup.",
                    args[0]
                ))
            }
        }
        
        "info" | "about" => {
            MagicResult::Output(format!(
                "FORM REPL v{}\n\
                 Sessions: {}\n\
                 History entries: {}\n\
                 Timing display: {}",
                env!("CARGO_PKG_VERSION"),
                state.session_number - 1,
                state.history.len(),
                if state.show_timing { "ON" } else { "OFF" }
            ))
        }
        
        "lsmagic" | "magic" => {
            MagicResult::Output(
                "Available magic commands:\n\
                 %help, %?        - Show REPL help\n\
                 %quit, %exit, %q - Exit the REPL\n\
                 %history [N]     - Show last N history entries (default 10)\n\
                 %reset           - Clear session state and history\n\
                 %time            - Toggle timing display\n\
                 %who             - List declared symbols\n\
                 %last, %_        - Show last output\n\
                 %recall [N]      - Recall input from session N\n\
                 %theme           - List available themes\n\
                 %info            - Show session info\n\
                 %lsmagic         - List magic commands".to_string()
            )
        }
        
        _ => MagicResult::Error(format!(
            "Unknown magic command: %{}\nUse %lsmagic to see available commands.",
            magic_name
        )),
    }
}

/// Format history for display
fn format_history(history: &[HistoryEntry], n: usize) -> String {
    let start = history.len().saturating_sub(n);
    let mut output = String::new();
    
    for entry in history.iter().skip(start) {
        output.push_str(&format!("In [{}]: {}\n", entry.number, 
            entry.input.lines().next().unwrap_or("")));
        
        // Show truncated input if multi-line
        if entry.input.lines().count() > 1 {
            output.push_str("        ...\n");
        }
        
        if let Some(ref out) = entry.output {
            let first_line = out.lines().next().unwrap_or("");
            if !first_line.trim().is_empty() {
                output.push_str(&format!("Out[{}]: {}\n", entry.number, first_line));
                if out.lines().count() > 1 {
                    output.push_str("        ...\n");
                }
            }
        }
        
        if let Some(dur) = entry.duration {
            output.push_str(&format!("        ({:.3}s)\n", dur.as_secs_f64()));
        }
        output.push('\n');
    }
    
    output
}

/// Extract declared symbols from session history
fn extract_symbols(history: &[HistoryEntry]) -> Vec<String> {
    use regex::Regex;
    use std::collections::HashSet;
    use std::sync::LazyLock;
    
    static SYMBOL_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)\b(?:Symbol|Symbols)\s+([^;]+);").unwrap()
    });
    
    let mut symbols = HashSet::new();
    
    for entry in history {
        for cap in SYMBOL_RE.captures_iter(&entry.input) {
            if let Some(m) = cap.get(1) {
                for sym in m.as_str().split(',') {
                    let clean = sym.trim()
                        .split('(').next().unwrap_or("")
                        .trim();
                    if !clean.is_empty() && clean.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) {
                        symbols.insert(clean.to_string());
                    }
                }
            }
        }
    }
    
    let mut result: Vec<_> = symbols.into_iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_state() {
        let mut state = SessionState::new();
        state.add_entry("test".to_string(), Some("output".to_string()), None);
        assert_eq!(state.session_number, 2);
        assert_eq!(state.last_output(), Some(&"output".to_string()));
    }
    
    #[test]
    fn test_magic_help() {
        let mut state = SessionState::new();
        match process_magic("%help", &mut state, false, "default") {
            MagicResult::Help => {}
            _ => panic!("Expected Help result"),
        }
    }
    
    #[test]
    fn test_magic_not_magic() {
        let mut state = SessionState::new();
        match process_magic("Symbol x;", &mut state, false, "default") {
            MagicResult::NotMagic => {}
            _ => panic!("Expected NotMagic result"),
        }
    }
}
