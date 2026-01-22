mod modules;

use std::env;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rustyline::Editor;

use modules::theme::get_theme;
use modules::form::{self, find_form_executable};

struct Config {
    highlight: bool,
    theme_name: String,
    verbose: bool,
}

fn print_help(highlight: bool, theme: &modules::theme::Theme) {
    let p = if highlight { &format!("{}{}", theme.prompt, "\x1b[1m") } else { "" };
    let r = if highlight { "\x1b[0m" } else { "" };

    println!("{0}FORM REPL{1} - Multi-line input mode", p, r);
    println!("  - Type statements, press Enter to continue on next line");
    println!("  - Press Enter on empty line to submit");
    println!("  - Use Up/Down arrows for command history");
    println!("  - Use Ctrl+C to cancel current input");
    println!("  - Type .help for commands, .quit to exit");
    println!();
    println!("Available commands:");
    println!("  {0}.help{1}   - Show this help", p, r);
    println!("  {0}.quit{1}   - Exit the REPL", p, r);
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
                println!("  Enter              - Continue to next line (or submit if empty)");
                println!("  Up/Down            - Command history");
                println!("  Ctrl+C             - Cancel current input");
                println!("  Ctrl+D             - Submit (if buffer not empty) or exit (if empty)");
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
    if trimmed.starts_with('.') && !trimmed.contains(' ') && !trimmed.contains('\t')
        && trimmed != ".end" {
        Some(trimmed)
    } else {
        None
    }
}

// Horizontal line separator
const SEPARATOR: &str = "────────────────────────────────────────";

fn main() {
    let config = parse_args();
    let theme = get_theme(&config.theme_name);

    let form_path = match find_form_executable() {
        Some(p) => p,
        None => {
            eprintln!("Error: Could not find FORM executable");
            eprintln!("Make sure 'sources/form' exists or FORM is in your PATH");
            std::process::exit(1);
        }
    };

    let p = if config.highlight { &format!("{}{}", theme.prompt, "\x1b[1m") } else { "" };
    let r = if config.highlight { "\x1b[0m" } else { "" };

    println!("{0}FORM REPL{1} - Multi-line input mode", p, r);
    println!("  - Type statements, press Enter to continue on next line");
    println!("  - Press Enter on empty line to submit");
    println!("  - Use Up/Down arrows for command history");
    if config.highlight {
        println!("  - Theme: {}", config.theme_name);
    }
    if config.verbose {
        println!("  - Verbose mode enabled");
        // Set the global verbose flag using atomic store operation
        modules::term::VERBOSE.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    println!();

    // Initialize rustyline editor with explicit type for rustyline v14
    // Using () for the Helper type (no custom helper) and default FileHistory
    let mut rl: Editor<(), rustyline::history::FileHistory> = match Editor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Failed to initialize editor: {:?}", e);
            std::process::exit(1);
        }
    };

    // Print initial separator
    if config.highlight {
        print!("{}{}\r\n", theme.prompt, SEPARATOR);
    } else {
        print!("{}\r\n", SEPARATOR);
    }
    let _ = std::io::stdout().flush();

    // Set up Ctrl+C handler to gracefully exit the REPL
    // Note: This sets a flag to exit the main loop, but doesn't interrupt ongoing FORM execution
    // Ctrl+C during input will cancel the current input buffer (handled by rustyline)
    let running = Arc::new(AtomicBool::new(true));
    let r_clone = running.clone();

    ctrlc::set_handler(move || {
        r_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    let error_style = if config.highlight { &theme.error } else { "" };
    let reset_str = if config.highlight { "\x1b[0m" } else { "" };
    let bold_str = if config.highlight { "\x1b[1m" } else { "" };

    let mut session_count = 1;

    while running.load(Ordering::SeqCst) {
        // Build prompt
        let prompt = format!("[{}] > ", session_count);
        let mut prompt_with_color = if config.highlight {
            format!("{}{}{}", theme.prompt, bold_str, prompt)
        } else {
            prompt.clone()
        };

        // Read input with multi-line support
        let mut full_input = String::new();
        let mut is_first_line = true;
        let mut cancelled = false;

        loop {
            match rl.readline_with_initial(&prompt_with_color, ("", "")) {
                Ok(line) => {
                    let trimmed = line.trim();

                    if trimmed == ".end" {
                        // .end on its own line submits
                        full_input.push_str(&line);
                        break;
                    }

                    if line.is_empty() {
                        // Empty line
                        if full_input.is_empty() {
                            // Completely empty - just continue
                            if is_first_line {
                                println!("Use '.quit' to exit, or type code and press Enter to continue");
                                continue;
                            } else {
                                // Submit on empty continuation line
                                break;
                            }
                        } else {
                            // Non-empty buffer + empty line = submit
                            break;
                        }
                    }

                    // Non-empty line
                    if is_first_line {
                        // Check for REPL commands
                        if let Some(cmd) = is_repl_command(&line) {
                            match cmd {
                                ".quit" | ".exit" => {
                                    println!("Goodbye!");
                                    return;
                                }
                                ".help" => {
                                    print_help(config.highlight, &theme);
                                    cancelled = true;
                                    break;
                                }
                                ".clear" => {
                                    cancelled = true;
                                    break;
                                }
                                _ => {
                                    println!("Unknown command: {}", cmd);
                                    cancelled = true;
                                    break;
                                }
                            }
                        }
                    }

                    // Add line to input
                    if !full_input.is_empty() {
                        full_input.push('\n');
                    }
                    full_input.push_str(&line);
                    is_first_line = false;

                    // Continue to next line
                    // Reassign instead of clear + push_str for better efficiency
                    prompt_with_color = if config.highlight {
                        format!("{}{}... > ", theme.prompt, bold_str)
                    } else {
                        "... > ".to_string()
                    };
                }
                Err(rustyline::error::ReadlineError::Interrupted) => {
                    // Ctrl+C - cancel current input
                    println!("^C");
                    cancelled = true;
                    break;
                }
                Err(rustyline::error::ReadlineError::Eof) => {
                    // Ctrl+D
                    if full_input.is_empty() {
                        println!("\nUse '.quit' to exit");
                        return;
                    } else {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Readline error: {:?}", e);
                    cancelled = true;
                    break;
                }
            }
        }

        if cancelled {
            // Reset for next input
            session_count += 1;
            if config.highlight {
                print!("{}{}\r\n", theme.prompt, SEPARATOR);
            } else {
                print!("{}\r\n", SEPARATOR);
            }
            let _ = std::io::stdout().flush();
            continue;
        }

        if full_input.is_empty() {
            continue;
        }

        // Add to history (without .end)
        let hist_line: String = full_input.lines()
            .filter(|l| l.trim() != ".end")
            .collect::<Vec<_>>()
            .join("\n");
        if !hist_line.is_empty() {
            let _ = rl.add_history_entry(hist_line);
        }

        // Ensure .end at the end
        if !full_input.trim_end().ends_with(".end") {
            full_input.push_str("\n.end");
        }

        // Execute FORM command
        if config.verbose {
            eprintln!("[verbose] Running FORM: input={} bytes", full_input.len());
        }

        match form::run_form(&full_input, &form_path, config.verbose) {
            Ok(output) => {
                let formatted = form::format_output(&output);
                if !formatted.trim().is_empty() {
                    print!("\r\n");
                    if config.highlight {
                        print!("{}{}{}", theme.output, formatted, reset_str);
                    } else {
                        print!("{}", formatted);
                    }
                    print!("\r\n");
                }
            }
            Err(e) => {
                eprintln!("{}{}Error: {}{}", error_style, bold_str, e, reset_str);
            }
        }

        session_count += 1;

        // Print separator for next session
        if config.highlight {
            print!("{}{}\r\n", theme.prompt, SEPARATOR);
        } else {
            print!("{}\r\n", SEPARATOR);
        }
        let _ = std::io::stdout().flush();
    }

    println!("Goodbye!");
}
