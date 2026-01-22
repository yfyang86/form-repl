// Input handling and buffer management
use std::io::Write;

use crate::term::{read_char, read_escape_sequence, ERASE_TO_END};

pub struct InputBuffer {
    pub lines: Vec<String>,
    pub current_index: usize,
    pub is_submitting: bool,
}

impl InputBuffer {
    pub fn new() -> Self {
        InputBuffer {
            lines: Vec::new(),
            current_index: 0,
            is_submitting: false,
        }
    }

    pub fn ensure_current_line(&mut self) {
        if self.current_index >= self.lines.len() {
            self.lines.push(String::new());
        }
    }

    pub fn get_prompt(&self, highlight: bool, theme_prompt: &str) -> String {
        if self.current_index == 0 {
            if highlight { format!("{}{}", theme_prompt, super::theme::FORM_PROMPT) }
            else { super::theme::FORM_PROMPT.to_string() }
        } else {
            if highlight { format!("{}{}", theme_prompt, super::theme::CONTINUATION_PROMPT) }
            else { super::theme::CONTINUATION_PROMPT.to_string() }
        }
    }

    pub fn print_prompt(&self, highlight: bool, theme_prompt: &str) {
        let prompt = self.get_prompt(highlight, theme_prompt);
        let line = &self.lines[self.current_index];

        if highlight {
            print!("{}{}{}", theme_prompt, prompt, line);
        } else {
            print!("{}{}", prompt, line);
        }
        let _ = std::io::stdout().flush();
    }

    pub fn reprompt(&mut self, highlight: bool, theme_prompt: &str) {
        print!("{}", ERASE_TO_END);
        self.print_prompt(highlight, theme_prompt);
    }

    pub fn has_content(&self) -> bool {
        self.lines.iter().any(|l| !l.is_empty())
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.current_index = 0;
        self.is_submitting = false;
    }

    pub fn submit(&mut self) {
        self.is_submitting = true;
    }
}

// Key event types
pub enum KeyEvent {
    Enter,
    CtrlC,
    CtrlD,
    CtrlL,
    UpArrow,
    DownArrow,
    Backspace,
    Char(char),
    Escape(String),
    None,
}

pub fn read_key_event() -> KeyEvent {
    match read_char() {
        Ok(b) => {
            let ch = b as char;

            if ch == '\n' || ch == '\r' {
                KeyEvent::Enter
            } else if ch == '\x04' {
                KeyEvent::CtrlD
            } else if ch == '\x03' {
                KeyEvent::CtrlC
            } else if ch == '\x0c' {
                KeyEvent::CtrlL
            } else if ch == '\x1b' {
                let seq = read_escape_sequence();

                match seq.as_str() {
                    "[A" => KeyEvent::UpArrow,
                    "[B" => KeyEvent::DownArrow,
                    _ => KeyEvent::Escape(seq),
                }
            } else if ch == '\x7f' || ch == '\x08' {
                KeyEvent::Backspace
            } else if !ch.is_ascii_control() {
                KeyEvent::Char(ch)
            } else {
                KeyEvent::None
            }
        }
        Err(_) => {
            KeyEvent::None
        }
    }
}
