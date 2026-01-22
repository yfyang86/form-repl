mod modules;

use std::env;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use modules::term::{self, ERASE_SCREEN, ERASE_TO_END, RESET, BOLD};
use modules::theme::get_theme;
use modules::input::{self, InputBuffer, KeyEvent};
use modules::form::{self, find_form_executable};

struct Config {
    highlight: bool,
    theme_name: String,
    verbose: bool,
}

fn print_help(highlight: bool, theme: &modules::theme::Theme) {
    let p = if highlight { &format!("{}{}", theme.prompt, BOLD) } else { "" };
    let r = if highlight { RESET } else { "" };

    println!("{0}FORM REPL{1} - Multi-line input mode", p, r);
    println!("  - Type statements, press Enter to continue on next line");
    println!("  - Press Enter on empty line to submit");
    println!("  - Use {0}Up/Down{1} arrows to navigate between buffer lines", p, r);
    println!("  - Use {0}Backspace{1} to edit current line", p, r);
    println!("  - Use {0}Ctrl+L{1} to clear the screen", p, r);
    println!("  - Type {0}.help{1} for commands, {0}.quit{1} to exit", p, r);
    println!();
    println!("Available commands:");
    println!("  {0}.help{1}   - Show this help", p, r);
    println!("  {0}.quit{1}   - Exit the REPL", p, r);
    println!("  {0}.clear{1}  - Clear the input buffer", p, r);
    println!();
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut config = Config {
        highlight: false,
        theme_name: "default".to_string(),
        verbose: false,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--highlight" | "-h" => config.highlight = true,
            "--theme" | "-t" => {
                if i + 1 < args.len() {
                    config.theme_name = args[i + 1].clone();
                    i += 1;
                }
            }
            "--verbose" | "-v" => config.verbose = true,
            "--help" | "-H" => {
                println!("FORM REPL - Interactive FORM environment");
                println!();
                println!("Usage: form-repl [OPTIONS]");
                println!();
                println!("Options:");
                println!("  --highlight, -h    Enable syntax highlighting");
                println!("  --theme, -t NAME   Set color theme (default, solarized-dark, monokai, dracula)");
                println!("  --verbose, -v      Enable verbose debug output");
                println!("  --help, -H         Show this help message");
                println!();
                println!("Key bindings:");
                println!("  Enter              - Continue to next line");
                println!("  Enter (empty line) - Submit to FORM");
                println!("  Up/Down            - Navigate between buffer lines");
                println!("  Backspace          - Delete character on current line");
                println!("  Ctrl+L             - Clear screen");
                println!("  Ctrl+C             - Cancel current input");
                println!("  Ctrl+D             - Submit (if buffer not empty)");
                std::process::exit(0);
            }
            _ => {}
        }
        i += 1;
    }

    config
}

fn is_repl_command(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    // Only treat as REPL command if it starts with '.' and has no args
    // But exclude '.end' which is a FORM directive
    if trimmed.starts_with('.') && !trimmed.contains(' ') && !trimmed.contains('\t')
        && trimmed != ".end" {
        Some(trimmed)
    } else {
        None
    }
}

// Print prompt for current buffer line with session number
fn print_buffer_prompt(buffer: &InputBuffer, session: usize) {
    let is_first_line = buffer.current_index == 0;
    let prompt = if is_first_line {
        format!("[{}] > ", session)
    } else {
        format!("... > ")
    };
    let line = &buffer.lines[buffer.current_index];
    print!("{}{}", prompt, line);
    let _ = std::io::stdout().flush();
}

// Reprompt on current line (erases to end of line first)
fn reprompt_buffer(buffer: &mut InputBuffer, session: usize) {
    print!("{}", ERASE_TO_END);
    print_buffer_prompt(buffer, session);
}

// Horizontal line separator
const SEPARATOR_WITH_NEWLINE: &str = "\r\n────────────────────────────────────────\r\n";

fn main() {
    let config = parse_args();
    let theme = get_theme(&config.theme_name);

    // Set verbose flag
    unsafe { term::VERBOSE = config.verbose; }

    let form_path = match find_form_executable() {
        Some(p) => p,
        None => {
            eprintln!("Error: Could not find FORM executable");
            eprintln!("Make sure 'sources/form' exists or FORM is in your PATH");
            std::process::exit(1);
        }
    };

    let p = if config.highlight { &format!("{}{}", theme.prompt, BOLD) } else { "" };
    let r = if config.highlight { RESET } else { "" };

    println!("{0}FORM REPL{1} - Multi-line input mode", p, r);
    println!("  - Type statements, press Enter to continue on next line");
    println!("  - Press Enter on empty line to submit");
    println!("  - Use Up/Down arrows to navigate between buffer lines, Ctrl+L to clear screen");
    if config.highlight {
        println!("  - Theme: {}", config.theme_name);
    }
    if config.verbose {
        println!("  - Verbose mode enabled");
    }
    println!();

    // Enable raw terminal mode (disables echo, enables raw input)
    let _raw_guard = match term::enable_raw_mode() {
        Ok(guard) => guard,
        Err(e) => {
            eprintln!("Warning: Could not enable raw mode: {}. Input may not display properly.", e);
            // Continue anyway - it may still work in some terminals
            std::process::exit(1);
        }
    };

    let running = Arc::new(AtomicBool::new(true));
    let r_clone = running.clone();

    ctrlc::set_handler(move || {
        r_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    let error_style = if config.highlight { &theme.error } else { "" };
    let reset_str = if config.highlight { RESET } else { "" };

    let mut buffer = InputBuffer::new();
    let mut is_first_line = true;
    let mut session_count = 1;

    // Print initial separator
    if config.highlight {
        print!("{}{}{}", theme.prompt, SEPARATOR_WITH_NEWLINE, reset_str);
    } else {
        print!("{}", SEPARATOR_WITH_NEWLINE);
    }
    let _ = std::io::stdout().flush();

    while running.load(Ordering::SeqCst) {
        // Ensure we have a line to edit
        buffer.ensure_current_line();

        // Print prompt with session number
        print_buffer_prompt(&buffer, session_count);

        // Read and process key events
        loop {
            match input::read_key_event() {
                KeyEvent::Enter => {
                    // User pressed Enter
                    let current_line = &buffer.lines[buffer.current_index];
                    let current_line_empty = current_line.is_empty();
                    let trimmed = current_line.trim();

                    // Check if current line is ".end" - this always triggers submission
                    if trimmed == ".end" {
                        vprintln!("[verbose] Submitting buffer with {} lines (found .end)", buffer.lines.len());
                        buffer.is_submitting = true;
                        break;
                    }

                    if current_line_empty && buffer.has_content() {
                        // Empty line with prior content = submit
                        vprintln!("[verbose] Submitting buffer with {} lines", buffer.lines.len());
                        buffer.is_submitting = true;
                        break;
                    } else if current_line_empty {
                        // Buffer completely empty, show hint
                        println!("Use '.quit' to exit, or press Enter on a line to submit");
                        reprompt_buffer(&mut buffer, session_count);
                        continue;
                    } else {
                        // Current line has content = continue to next line
                        // Print CR+LF to move to next line (needed in raw mode)
                        print!("\r\n");
                        let _ = std::io::stdout().flush();
                        buffer.current_index += 1;
                        buffer.ensure_current_line();
                        is_first_line = false;
                        vprintln!("[verbose] Continuing to line {}", buffer.current_index + 1);
                        break;
                    }
                }

                KeyEvent::CtrlC => {
                    buffer.clear();
                    println!("^C");
                    break;
                }

                KeyEvent::CtrlD => {
                    if buffer.has_content() {
                        buffer.is_submitting = true;
                        break;
                    } else {
                        println!("\nUse '.quit' to exit");
                        return;
                    }
                }

                KeyEvent::CtrlL => {
                    print!("{}{}", ERASE_SCREEN, reset_str);
                    let _ = std::io::stdout().flush();
                    // Reprompt on current line
                    reprompt_buffer(&mut buffer, session_count);
                }

                KeyEvent::UpArrow => {
                    if buffer.current_index > 0 {
                        buffer.current_index -= 1;
                        reprompt_buffer(&mut buffer, session_count);
                    }
                }

                KeyEvent::DownArrow => {
                    if buffer.current_index + 1 < buffer.lines.len().max(1) {
                        buffer.current_index += 1;
                        buffer.ensure_current_line();
                        reprompt_buffer(&mut buffer, session_count);
                    }
                }

                KeyEvent::Backspace => {
                    if !buffer.lines[buffer.current_index].is_empty() {
                        buffer.lines[buffer.current_index].pop();
                        term::handle_backspace();
                    }
                }

                KeyEvent::Char(ch) => {
                    buffer.lines[buffer.current_index].push(ch);
                    term::print_char(ch);
                }

                KeyEvent::Escape(_seq) => {
                    // Ignore unknown escape sequences
                    print!("{}", term::ERASE_TO_END);
                    let _ = std::io::stdout().flush();
                }

                KeyEvent::None => {
                    // Read error or unknown, continue
                }
            }
        }

        let running_now = running.load(Ordering::SeqCst);
        if !running_now {
            break;
        }

        // Process submission
        if buffer.is_submitting {
            vprintln!("[verbose] Processing submission, lines={}", buffer.lines.len());
            let input = buffer.lines.join("\n");

            // Check for REPL commands (only on first line)
            if is_first_line {
                if let Some(cmd) = is_repl_command(&input) {
                    match cmd {
                        ".quit" | ".exit" => {
                            break;
                        }
                        ".help" => {
                            print_help(config.highlight, &theme);
                            buffer.clear();
                            is_first_line = true;
                            continue;
                        }
                        ".clear" => {
                            buffer.clear();
                            is_first_line = true;
                            continue;
                        }
                        _ => {
                            println!("Unknown command: {}", cmd);
                            buffer.clear();
                            is_first_line = true;
                            continue;
                        }
                    }
                }
            }

            // Execute FORM command
            vprintln!("[verbose] Running FORM: input={} bytes, lines={}", input.len(), buffer.lines.len());
            match form::run_form(&input, &form_path, config.verbose) {
                Ok(output) => {
                    let formatted = form::format_output(&output);
                    if !formatted.trim().is_empty() {
                        // Print newline first to ensure FORM output starts on new line
                        print!("\r\n");
                        let _ = std::io::stdout().flush();
                        if config.highlight {
                            print!("{}{}{}", theme.output, formatted, reset_str);
                        } else {
                            print!("{}", formatted);
                        }
                        // Use \r\n for proper newline in raw mode
                        print!("\r\n");
                        let _ = std::io::stdout().flush();
                    } else {
                        print!("\r\n");
                        let _ = std::io::stdout().flush();
                    }
                }
                Err(e) => {
                    eprintln!("{}{}Error: {}{}", error_style, BOLD, e, reset_str);
                }
            }

            buffer.clear();
            is_first_line = true;
            session_count += 1;

            // Print separator for next session
            if config.highlight {
                print!("{}{}{}", theme.prompt, SEPARATOR_WITH_NEWLINE, reset_str);
            } else {
                print!("{}", SEPARATOR_WITH_NEWLINE);
            }
            let _ = std::io::stdout().flush();
        }
    }

    println!("Goodbye!");
}
