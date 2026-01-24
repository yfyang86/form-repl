mod modules;

use std::env;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;

use modules::config::Config;
use modules::form::{self, find_form_executable};
use modules::highlight;
use modules::magic::{self, MagicResult, SessionState};
use modules::term::{self, ansi};
use modules::theme::{self, Theme};

/// Runtime configuration from CLI arguments
struct CliConfig {
    highlight: bool,
    theme_name: String,
    verbose: bool,
    show_help: bool,
    show_version: bool,
    show_sample_config: bool,
}

/// Print the help message
fn print_help(theme: &Theme, highlight: bool) {
    let reset = ansi::RESET;
    let bold = ansi::BOLD;
    let h = if highlight { theme.prompt_in.as_str() } else { "" };
    let r = if highlight { reset } else { "" };

    println!();
    println!("{}{}FORM REPL{} - Interactive FORM environment with IPython-like UX", h, bold, r);
    println!();
    println!("{}Input modes:{}", bold, reset);
    println!("  • Type FORM code, press Enter to continue on next line");
    println!("  • Press Enter on empty line (or type .end) to submit");
    println!("  • Use Up/Down arrows for command history");
    println!("  • Press Ctrl+C to cancel current input");
    println!("  • Press Ctrl+D to exit (or submit if buffer not empty)");
    println!();
    println!("{}REPL commands:{}", bold, reset);
    println!("  {}{}help{}, {}.quit{}   - Show help / Exit", h, ".", r, h, r);
    println!("  {}.clear{}          - Clear current input buffer", h, r);
    println!();
    println!("{}Magic commands:{}", bold, reset);
    println!("  {}%history [N]{}    - Show last N history entries", h, r);
    println!("  {}%time{}           - Toggle timing display", h, r);
    println!("  {}%who{}            - List declared symbols", h, r);
    println!("  {}%reset{}          - Clear session state", h, r);
    println!("  {}%lsmagic{}        - List all magic commands", h, r);
    println!();
}

/// Print version information
fn print_version() {
    println!("FORM REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("A modern interactive environment for FORM");
}

/// Parse command line arguments
fn parse_args() -> CliConfig {
    let args: Vec<String> = env::args().collect();
    let mut config = CliConfig {
        highlight: false,
        theme_name: "default".to_string(),
        verbose: false,
        show_help: false,
        show_version: false,
        show_sample_config: false,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            // Fixed: -h is now for help (standard convention)
            "--help" | "-h" => config.show_help = true,
            "--version" | "-V" => config.show_version = true,
            
            // Highlighting uses -H or --highlight
            "--highlight" | "-H" => config.highlight = true,
            "--no-highlight" => config.highlight = false,
            
            "--theme" | "-t" => {
                if i + 1 < args.len() {
                    config.theme_name = args[i + 1].clone();
                    config.highlight = true; // Auto-enable highlighting with theme
                    i += 1;
                } else {
                    eprintln!("Error: --theme requires a theme name");
                    eprintln!("Available themes: {}", theme::list_themes().join(", "));
                    std::process::exit(1);
                }
            }
            
            "--verbose" | "-v" => config.verbose = true,
            
            "--sample-config" => config.show_sample_config = true,
            
            "--list-themes" => {
                println!("Available themes:");
                for t in theme::list_themes() {
                    println!("  • {}", t);
                }
                std::process::exit(0);
            }
            
            arg if arg.starts_with('-') => {
                eprintln!("Unknown option: {}", arg);
                eprintln!("Use --help for usage information");
                std::process::exit(1);
            }
            
            _ => {}
        }
        i += 1;
    }

    config
}

/// Check if input is a REPL command (starts with . but not .end)
fn is_repl_command(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.starts_with('.')
        && !trimmed.contains(' ')
        && !trimmed.contains('\t')
        && trimmed != ".end"
    {
        Some(trimmed)
    } else {
        None
    }
}

/// Format the input prompt (IPython style)
fn format_in_prompt(n: usize, theme: &Theme, highlight: bool) -> String {
    if highlight {
        format!(
            "{}{}In [{}]:{} ",
            theme.prompt_in,
            ansi::BOLD,
            n,
            ansi::RESET
        )
    } else {
        format!("In [{}]: ", n)
    }
}

/// Format the continuation prompt
fn format_cont_prompt(n: usize, theme: &Theme, highlight: bool) -> String {
    let spaces = format!("{}", n).len();
    let padding = " ".repeat(spaces + 5); // "In [" + n + "]"
    
    if highlight {
        format!(
            "{}{}...:{} ",
            theme.prompt_cont,
            padding,
            ansi::RESET
        )
    } else {
        format!("{}...: ", padding)
    }
}

/// Format the output prompt
fn format_out_prompt(n: usize, theme: &Theme, highlight: bool) -> String {
    if highlight {
        format!(
            "{}{}Out[{}]:{} ",
            theme.prompt_out,
            ansi::BOLD,
            n,
            ansi::RESET
        )
    } else {
        format!("Out[{}]: ", n)
    }
}

/// Print separator line
fn print_separator(theme: &Theme, highlight: bool) {
    let width = 60;
    if highlight {
        println!("{}", term::separator(width, true, &theme.separator));
    } else {
        println!("{}", "─".repeat(width));
    }
}

/// Read multi-line input from the user
fn read_multiline_input(
    rl: &mut Editor<(), FileHistory>,
    session_num: usize,
    theme: &Theme,
    highlight: bool,
) -> Result<Option<String>, String> {
    let mut full_input = String::new();
    let mut is_first_line = true;

    loop {
        let prompt = if is_first_line {
            format_in_prompt(session_num, theme, highlight)
        } else {
            format_cont_prompt(session_num, theme, highlight)
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                // .end submits
                if trimmed == ".end" {
                    if !full_input.is_empty() {
                        full_input.push('\n');
                    }
                    full_input.push_str(".end");
                    return Ok(Some(full_input));
                }

                // Empty line handling
                if line.is_empty() {
                    if full_input.is_empty() {
                        if is_first_line {
                            // Completely empty - show hint
                            println!(
                                "{}Type FORM code, .help for help, or .quit to exit{}",
                                if highlight { &theme.prompt_cont } else { "" },
                                if highlight { ansi::RESET } else { "" }
                            );
                            continue;
                        }
                    }
                    // Non-empty buffer + empty line = submit
                    return Ok(Some(full_input));
                }

                // Check for REPL commands on first line
                if is_first_line {
                    if let Some(cmd) = is_repl_command(&line) {
                        return Err(format!("CMD:{}", cmd));
                    }
                    
                    // Check for magic commands
                    if trimmed.starts_with('%') {
                        return Err(format!("MAGIC:{}", trimmed));
                    }
                }

                // Add line to input
                if !full_input.is_empty() {
                    full_input.push('\n');
                }
                full_input.push_str(&line);
                is_first_line = false;
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C - cancel current input
                println!("^C");
                return Ok(None);
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                if full_input.is_empty() {
                    return Err("EXIT".to_string());
                } else {
                    return Ok(Some(full_input));
                }
            }
            Err(e) => {
                return Err(format!("Readline error: {:?}", e));
            }
        }
    }
}

fn main() {
    let cli_config = parse_args();
    
    // Handle special flags
    if cli_config.show_version {
        print_version();
        return;
    }
    
    if cli_config.show_sample_config {
        print!("{}", modules::config::sample_config());
        return;
    }
    
    // Load file config (can be overridden by CLI)
    let file_config = Config::load();
    
    // Merge configs: CLI takes precedence
    let highlight = cli_config.highlight || file_config.settings.highlight;
    let theme_name = if cli_config.theme_name != "default" {
        cli_config.theme_name.clone()
    } else {
        file_config.settings.theme.clone()
    };
    let verbose = cli_config.verbose || file_config.settings.verbose;
    
    let theme = theme::get_theme(&theme_name);
    
    if cli_config.show_help {
        print_help(&theme, highlight);
        println!("{}Usage:{} form-repl [OPTIONS]", ansi::BOLD, ansi::RESET);
        println!();
        println!("{}Options:{}", ansi::BOLD, ansi::RESET);
        println!("  -h, --help          Show this help message");
        println!("  -V, --version       Show version information");
        println!("  -H, --highlight     Enable syntax highlighting");
        println!("  -t, --theme NAME    Set color theme");
        println!("  -v, --verbose       Enable verbose debug output");
        println!("  --list-themes       List available themes");
        println!("  --sample-config     Print sample configuration file");
        println!();
        return;
    }

    // Find FORM executable
    let form_path: PathBuf = match find_form_executable() {
        Some(p) => p,
        None => {
            let error_prefix = if highlight {
                format!("{}{}", theme.error, ansi::BOLD)
            } else {
                String::new()
            };
            let error_suffix = if highlight { ansi::RESET } else { "" };
            eprintln!("{}Error:{} Could not find FORM executable", error_prefix, error_suffix);
            eprintln!("Make sure 'form' is in your PATH or set FORM_PATH environment variable");
            std::process::exit(1);
        }
    };

    // Set verbose mode
    if verbose {
        term::set_verbose(true);
        term::verbose_println(&format!("Using FORM at: {}", form_path.display()));
        term::verbose_println(&format!("Theme: {}", theme_name));
    }

    // Initialize session state
    let mut state = SessionState::new();
    state.show_timing = file_config.settings.show_timing;

    // Initialize rustyline
    let mut rl: Editor<(), FileHistory> = match Editor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Failed to initialize editor: {:?}", e);
            std::process::exit(1);
        }
    };

    // Load history
    let history_path = file_config.history_path();
    let _ = rl.load_history(&history_path);

    // Print welcome banner
    println!();
    if highlight {
        println!(
            "{}{}FORM REPL{} v{} — Type {}%help{} for help, {}.quit{} to exit",
            theme.prompt_in,
            ansi::BOLD,
            ansi::RESET,
            env!("CARGO_PKG_VERSION"),
            theme.prompt_out,
            ansi::RESET,
            theme.prompt_out,
            ansi::RESET
        );
        if verbose {
            println!("{}  Theme: {} | Verbose mode{}", theme.prompt_cont, theme_name, ansi::RESET);
        }
    } else {
        println!(
            "FORM REPL v{} — Type %help for help, .quit to exit",
            env!("CARGO_PKG_VERSION")
        );
    }
    println!();

    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r_clone = running.clone();
    ctrlc::set_handler(move || {
        r_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    // Main REPL loop
    while running.load(Ordering::SeqCst) {
        // Read input
        let input = match read_multiline_input(&mut rl, state.session_number, &theme, highlight) {
            Ok(Some(input)) => input,
            Ok(None) => {
                // Cancelled input
                print_separator(&theme, highlight);
                continue;
            }
            Err(msg) if msg == "EXIT" => {
                println!();
                break;
            }
            Err(msg) if msg.starts_with("CMD:") => {
                let cmd = &msg[4..];
                match cmd {
                    ".quit" | ".exit" | ".q" => {
                        break;
                    }
                    ".help" => {
                        print_help(&theme, highlight);
                    }
                    ".clear" => {
                        println!("Input cleared.");
                    }
                    _ => {
                        println!(
                            "{}Unknown command: {}{}",
                            if highlight { &theme.error } else { "" },
                            cmd,
                            if highlight { ansi::RESET } else { "" }
                        );
                    }
                }
                print_separator(&theme, highlight);
                continue;
            }
            Err(msg) if msg.starts_with("MAGIC:") => {
                let magic_cmd = &msg[6..];
                match magic::process_magic(magic_cmd, &mut state, highlight, &theme_name) {
                    MagicResult::Output(output) => {
                        println!("{}", output);
                    }
                    MagicResult::Help => {
                        print_help(&theme, highlight);
                    }
                    MagicResult::Exit => {
                        break;
                    }
                    MagicResult::Error(e) => {
                        println!(
                            "{}{}{}",
                            if highlight { &theme.error } else { "" },
                            e,
                            if highlight { ansi::RESET } else { "" }
                        );
                    }
                    MagicResult::Handled | MagicResult::NotMagic => {}
                }
                print_separator(&theme, highlight);
                continue;
            }
            Err(e) => {
                let error_prefix = if highlight {
                    format!("{}{}", theme.error, ansi::BOLD)
                } else {
                    String::new()
                };
                let error_suffix = if highlight { ansi::RESET } else { "" };
                eprintln!(
                    "{}Error: {}{}",
                    error_prefix,
                    e,
                    error_suffix
                );
                print_separator(&theme, highlight);
                continue;
            }
        };

        if input.trim().is_empty() {
            continue;
        }

        // Add to readline history
        let hist_line: String = input
            .lines()
            .filter(|l| l.trim() != ".end")
            .collect::<Vec<_>>()
            .join("\n");
        if !hist_line.is_empty() {
            let _ = rl.add_history_entry(&hist_line);
        }

        // Validate input
        if let Err(e) = form::validate_input(&input) {
            println!(
                "{}{}Syntax warning: {}{}",
                if highlight { &theme.error } else { "" },
                if highlight { ansi::BOLD } else { "" },
                e,
                if highlight { ansi::RESET } else { "" }
            );
        }

        // Execute FORM
        if verbose {
            term::verbose_println(&format!("Executing {} bytes of FORM code", input.len()));
        }

        match form::run_form(&input, &form_path, verbose) {
            Ok(result) => {
                let formatted = form::format_output(&result.output, state.show_timing);
                
                if !formatted.trim().is_empty() {
                    println!();
                    
                    // Print output prompt for first line
                    let out_prompt = format_out_prompt(state.session_number, &theme, highlight);
                    
                    // Apply syntax highlighting to output
                    let displayed = if highlight {
                        highlight::highlight_output(&formatted, &theme)
                    } else {
                        formatted.clone()
                    };
                    
                    // Print with proper formatting
                    let lines: Vec<&str> = displayed.lines().collect();
                    for (i, line) in lines.iter().enumerate() {
                        if i == 0 {
                            println!("{}{}", out_prompt, line);
                        } else {
                            // Indent continuation lines to align with output
                            let indent = " ".repeat(out_prompt.chars().filter(|c| !c.is_control()).count());
                            println!("{}{}", indent, line);
                        }
                    }
                }
                
                // Show timing if enabled
                if state.show_timing {
                    println!(
                        "{}⏱ {}{}",
                        if highlight { &theme.timing } else { "" },
                        term::format_duration(result.duration),
                        if highlight { ansi::RESET } else { "" }
                    );
                }
                
                // Record in session history
                state.add_entry(input, Some(formatted), Some(result.duration));
            }
            Err(e) => {
                println!(
                    "\n{}{}Error: {}{}",
                    if highlight { &theme.error } else { "" },
                    if highlight { ansi::BOLD } else { "" },
                    e,
                    if highlight { ansi::RESET } else { "" }
                );
                
                // Still record the attempt
                state.add_entry(input, None, None);
            }
        }

        println!();
        print_separator(&theme, highlight);
    }

    // Save history
    if file_config.history.save_on_exit {
        if let Err(e) = rl.save_history(&history_path) {
            if verbose {
                eprintln!("Warning: Could not save history: {}", e);
            }
        }
    }

    println!("Goodbye!");
}
