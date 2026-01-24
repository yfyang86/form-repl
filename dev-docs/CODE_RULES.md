# FORM REPL Code Structure & Rust Development Rules

## Table of Contents

1. [Project Structure](#project-structure)
2. [CLI Version Architecture](#cli-version-architecture)
3. [GUI Version Architecture](#gui-version-architecture)
4. [Rust Development Rules](#rust-development-rules)
5. [Code Patterns](#code-patterns)
6. [Error Handling](#error-handling)
7. [Testing Guidelines](#testing-guidelines)

---

## Project Structure

### Overview

```
form-repl/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── README.md
│   ├── CODE_REVIEW.md
│   └── src/
│       ├── main.rs              # Entry point, REPL loop
│       └── modules/
│           ├── mod.rs           # Module exports
│           ├── config.rs        # Configuration file handling
│           ├── form.rs          # FORM process execution
│           ├── highlight.rs     # Syntax highlighting lexer
│           ├── magic.rs         # IPython-style magic commands
│           ├── term.rs          # Terminal utilities
│           └── theme.rs         # Color theme definitions
│
├── form-repl-gui/               # GUI Version (Tauri)
│   ├── README.md
│   ├── src/
│   │   └── index.html           # Frontend (HTML + CSS + JS)
│   └── src-tauri/
│       ├── Cargo.toml
│       ├── build.rs
│       ├── tauri.conf.json      # Tauri v2 configuration
│       ├── capabilities/
│       │   └── default.json     # Permissions
│       └── src/
│           └── main.rs          # Backend, Tauri commands
│
└── dev-docs
│   ├──DEVELOPMENT_LOG.md        # Development history & rules
│   └──CODE_RULES.md             # Rule of code 
```

---

## CLI Version Architecture

### Module Dependency Graph

```
main.rs
    ├── modules::config      # Load ~/.form_replrc
    ├── modules::theme       # Get color theme
    ├── modules::form        # Execute FORM
    ├── modules::highlight   # Syntax highlighting
    ├── modules::magic       # %commands
    └── modules::term        # Terminal utilities
```

### Module Responsibilities

#### `main.rs` - Entry Point & REPL Loop

```rust
// Responsibilities:
// 1. Parse command-line arguments
// 2. Initialize rustyline editor
// 3. Main REPL loop (read → eval → print)
// 4. Handle REPL commands (.quit, .help)
// 5. Coordinate between modules

// Key structures:
struct CliConfig {
    highlight: bool,
    theme_name: String,
    verbose: bool,
}

// Key functions:
fn main()                    // Entry point
fn parse_args() -> CliConfig // CLI argument parsing
fn print_help()              // Display help message
fn is_repl_command()         // Check for .commands
fn read_multiline_input()    // Handle multi-line input
fn format_in_prompt()        // IPython-style "In [N]:"
fn format_out_prompt()       // IPython-style "Out[N]:"
```

#### `modules/config.rs` - Configuration

```rust
// Responsibilities:
// 1. Load config from ~/.form_replrc or ./.form_replrc
// 2. Parse TOML configuration
// 3. Provide defaults

// Key structures:
struct Config {
    settings: Settings,
    history: HistoryConfig,
}

struct Settings {
    highlight: bool,
    theme: String,
    show_timing: bool,
    verbose: bool,
    auto_end: bool,
}

struct HistoryConfig {
    file: String,
    max_entries: usize,
    save_on_exit: bool,
}

// Key functions:
fn Config::load() -> Self           // Load from file or defaults
fn expand_path(path: &str) -> PathBuf  // Expand ~ to home dir
fn sample_config() -> &'static str  // Generate sample config
```

#### `modules/form.rs` - FORM Execution

```rust
// Responsibilities:
// 1. Find FORM executable
// 2. Execute FORM with input
// 3. Parse and format output
// 4. Handle errors

// Key structures:
enum FormError {
    SpawnError(std::io::Error),
    WriteError(std::io::Error),
    ReadError(std::io::Error),
    ExecutionError { status: i32, stderr: String },
    Timeout,
    InvalidUtf8(std::string::FromUtf8Error),
    NotFound,
}

struct FormResult {
    output: String,
    stderr: String,
    duration: Duration,
    exit_code: i32,
}

// Key functions:
fn find_form_executable() -> Option<PathBuf>  // Search for FORM
fn run_form(input, path, verbose) -> Result<FormResult, FormError>
fn format_output(output, show_timing) -> String  // Clean output
fn validate_input(input) -> Result<(), String>   // Check brackets
```

#### `modules/highlight.rs` - Syntax Highlighting

```rust
// Responsibilities:
// 1. Tokenize FORM code
// 2. Classify tokens (keyword, declaration, etc.)
// 3. Apply ANSI colors

// Key structures:
enum TokenType {
    Keyword,      // if, repeat, id, print
    Declaration,  // Symbol, Local, CFunction
    Function,     // abs, sin, gcd
    Preprocessor, // .end, .sort, #define
    Number,
    Operator,
    Comment,      // * comment
    String,
    Identifier,
    Punctuation,
    Whitespace,
}

struct Token {
    token_type: TokenType,
    text: String,
}

// Key constants:
const KEYWORDS: &[&str]     // FORM keywords
const DECLARATIONS: &[&str] // Declaration keywords
const FUNCTIONS: &[&str]    // Built-in functions

// Key functions:
fn tokenize(line: &str) -> Vec<Token>
fn is_keyword(word: &str) -> bool
fn is_declaration(word: &str) -> bool
fn is_function(word: &str) -> bool
fn highlight_line(line, theme) -> String
fn highlight_code(code, theme) -> String
fn highlight_output(output, theme) -> String
```

#### `modules/magic.rs` - Magic Commands

```rust
// Responsibilities:
// 1. Parse %commands
// 2. Maintain session state
// 3. Execute magic commands

// Key structures:
struct HistoryEntry {
    number: usize,
    input: String,
    output: Option<String>,
    duration: Option<Duration>,
}

struct SessionState {
    history: Vec<HistoryEntry>,
    session_number: usize,
    last_outputs: VecDeque<String>,
    show_timing: bool,
}

enum MagicResult {
    Output(String),  // Display this text
    Handled,         // Command done, no output
    NotMagic,        // Not a magic command
    Error(String),   // Error message
    Exit,            // Exit REPL
    Help,            // Show help
}

// Key functions:
fn process_magic(cmd, state, highlight, theme) -> MagicResult
fn format_history(history, n) -> String
fn extract_symbols(history) -> Vec<String>  // For %who
```

#### `modules/theme.rs` - Color Themes

```rust
// Responsibilities:
// 1. Define color schemes
// 2. Provide ANSI escape codes

// Key structures:
struct Theme {
    // UI colors
    prompt_in: String,
    prompt_out: String,
    prompt_cont: String,
    separator: String,
    error: String,
    timing: String,
    output_label: String,
    
    // Syntax colors
    keyword: String,
    declaration: String,
    function: String,
    preprocessor: String,
    number: String,
    operator: String,
    comment: String,
    string: String,
    identifier: String,
}

// Key functions:
fn Theme::default() -> Self
fn Theme::none() -> Self           // No colors
fn Theme::solarized_dark() -> Self
fn Theme::monokai() -> Self
fn Theme::dracula() -> Self
fn Theme::nord() -> Self
fn Theme::gruvbox() -> Self
fn Theme::one_dark() -> Self
fn get_theme(name: &str) -> Theme
fn list_themes() -> Vec<&'static str>
```

#### `modules/term.rs` - Terminal Utilities

```rust
// Responsibilities:
// 1. Verbose logging
// 2. ANSI escape codes
// 3. Terminal utilities

// Key items:
static VERBOSE: AtomicBool  // Thread-safe verbose flag

mod ansi {
    const RESET: &str
    const BOLD: &str
    const DIM: &str
    const ITALIC: &str
}

// Key functions:
fn is_verbose() -> bool
fn set_verbose(enabled: bool)
fn verbose_println(msg: &str)
fn format_duration(duration: Duration) -> String
fn separator(width, colored, color) -> String

// Macro:
macro_rules! vprintln { ... }  // Conditional verbose print
```

---

## GUI Version Architecture

### Tauri Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (WebView)                   │
│  ┌─────────────────────────────────────────────────┐    │
│  │                 index.html                      │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │    │
│  │  │   CSS    │  │   HTML   │  │  JavaScript  │   │    │
│  │  │  Styles  │  │   DOM    │  │    Logic     │   │    │
│  │  └──────────┘  └──────────┘  └──────────────┘   │    │
│  └─────────────────────────────────────────────────┘    │
│                          │                              │
│                    invoke()                             │
│                          ▼                              │
├─────────────────────────────────────────────────────────┤
│                   Tauri Bridge                          │
├─────────────────────────────────────────────────────────┤
│                    Backend (Rust)                       │
│  ┌─────────────────────────────────────────────────┐    │
│  │                  main.rs                        │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │    │
│  │  │ AppState │  │ Commands │  │ FORM Runner  │   │    │
│  │  │ (Mutex)  │  │ (tauri)  │  │  (process)   │   │    │
│  │  └──────────┘  └──────────┘  └──────────────┘   │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

### Backend (`src-tauri/src/main.rs`)

```rust
// State management
struct AppState {
    history: Mutex<Vec<HistoryEntry>>,
    session_count: Mutex<usize>,
    form_path: Mutex<Option<PathBuf>>,
}

// Data structures
struct HistoryEntry { number, input, output, error, duration_ms }
struct FormResult { success, output, error, duration_ms, session_number }
struct AppInfo { version, form_path, session_count, history_count }

// Tauri commands (callable from JavaScript)
#[tauri::command]
fn execute_form(input: String, state: State<AppState>) -> FormResult
fn get_history(count: Option<usize>, state: State<AppState>) -> Vec<HistoryEntry>
fn clear_history(state: State<AppState>)
fn get_app_info(state: State<AppState>) -> AppInfo
fn set_form_path(path: String, state: State<AppState>) -> Result<String, String>

// Internal functions
fn find_form_executable() -> Option<PathBuf>
fn run_form(input: &str, form_path: &PathBuf) -> Result<(String, u64), String>
fn format_output(output: &str) -> String
```

### Frontend (`src/index.html`)

```javascript
// Global state
let invoke = null;           // Tauri invoke function
let sessionNumber = 1;
let isRunning = false;
let commandHistory = [];
let historyIndex = -1;

// Initialization
document.addEventListener('DOMContentLoaded', initializeTauri);
async function initializeTauri()  // Setup Tauri API
async function initializeApp()    // Check FORM path

// Core functions
async function executeCode()      // Run FORM code
function clearOutput()            // Clear output area
async function showHistory()      // Display history

// UI helpers
function setStatus(text, isError)
function updateSessionInfo(count)
function appendError(message)

// Syntax highlighting
function highlightCode(code)      // Highlight input
function highlightOutput(output)  // Highlight output (disabled)
function escapeHtml(text)         // Escape HTML entities

// Event handlers
function handleKeyDown(e)         // Keyboard shortcuts
function navigateHistory(dir)     // History navigation
```

---

## Rust Development Rules

### 1. Error Handling

#### Rule 1.1: Use Custom Error Types

```rust
// ❌ DON'T: Use String for errors
fn do_something() -> Result<T, String>

// ✅ DO: Define custom error enums
#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(String),
    NotFound,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO error: {}", e),
            MyError::ParseError(s) => write!(f, "Parse error: {}", s),
            MyError::NotFound => write!(f, "Not found"),
        }
    }
}

impl std::error::Error for MyError {}
```

#### Rule 1.2: Use `map_err` for Error Conversion

```rust
// ❌ DON'T: Manual match for error conversion
match file.read_to_string(&mut buf) {
    Ok(_) => Ok(buf),
    Err(e) => Err(MyError::IoError(e)),
}

// ✅ DO: Use map_err
file.read_to_string(&mut buf)
    .map_err(MyError::IoError)?;
```

#### Rule 1.3: Propagate Errors with `?`

```rust
// ❌ DON'T: Manual unwrapping
let file = match File::open(path) {
    Ok(f) => f,
    Err(e) => return Err(e.into()),
};

// ✅ DO: Use ? operator
let file = File::open(path)?;
```

### 2. Memory & Ownership

#### Rule 2.1: Avoid Unnecessary Clones

```rust
// ❌ DON'T: Clone when not needed
fn process(data: &String) {
    let owned = data.clone();
    println!("{}", owned);
}

// ✅ DO: Use references
fn process(data: &str) {
    println!("{}", data);
}
```

#### Rule 2.2: Use `Cow` for Conditional Ownership

```rust
use std::borrow::Cow;

// ✅ DO: Use Cow when you might or might not need to own
fn maybe_modify(s: &str, modify: bool) -> Cow<str> {
    if modify {
        Cow::Owned(s.to_uppercase())
    } else {
        Cow::Borrowed(s)
    }
}
```

#### Rule 2.3: Temporary Values in Conditionals

```rust
// ❌ DON'T: Return reference to temporary
let msg = if error {
    &format!("Error: {}", e)  // Temporary dropped!
} else {
    "OK"
};

// ✅ DO: Use let binding or return owned
let msg = if error {
    format!("Error: {}", e)
} else {
    String::from("OK")
};

// ✅ DO: Or use consistent reference types
let error_msg;
let msg: &str = if error {
    error_msg = format!("Error: {}", e);
    &error_msg
} else {
    "OK"
};
```

### 3. Concurrency

#### Rule 3.1: Use Atomic Types for Simple Flags

```rust
// ❌ DON'T: Use Mutex for simple flags
static VERBOSE: Mutex<bool> = Mutex::new(false);

// ✅ DO: Use AtomicBool
use std::sync::atomic::{AtomicBool, Ordering};
static VERBOSE: AtomicBool = AtomicBool::new(false);

fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

fn set_verbose(v: bool) {
    VERBOSE.store(v, Ordering::Relaxed);
}
```

#### Rule 3.2: Lock Ordering with Multiple Mutexes

```rust
// ❌ DON'T: Inconsistent lock order (deadlock risk)
fn update(state: &AppState) {
    let a = state.mutex_a.lock().unwrap();
    let b = state.mutex_b.lock().unwrap();  // May deadlock
}

// ✅ DO: Always lock in same order, drop early
fn update(state: &AppState) {
    let count = {
        let a = state.mutex_a.lock().unwrap();
        a.count  // Copy value, release lock
    };
    let mut b = state.mutex_b.lock().unwrap();
    b.update(count);
}
```

### 4. String Handling

#### Rule 4.1: Accept `&str`, Return `String`

```rust
// ❌ DON'T: Accept owned String unnecessarily
fn process(s: String) -> String

// ✅ DO: Accept &str, return String
fn process(s: &str) -> String {
    s.to_uppercase()
}
```

#### Rule 4.2: Use `format!` Sparingly

```rust
// ❌ DON'T: Use format! for simple concatenation
let s = format!("{}", text);

// ✅ DO: Use to_string() or into()
let s = text.to_string();
let s: String = text.into();

// ✅ DO: Use format! only when needed
let s = format!("[{}] {}: {}", level, module, msg);
```

#### Rule 4.3: Use `write!` for Building Strings

```rust
use std::fmt::Write;

// ❌ DON'T: Repeated string concatenation
let mut s = String::new();
s = s + "Hello";
s = s + " ";
s = s + "World";

// ✅ DO: Use write! macro
let mut s = String::new();
write!(s, "Hello {} {}", "World", 123).unwrap();

// ✅ DO: Or use push_str
let mut s = String::new();
s.push_str("Hello");
s.push(' ');
s.push_str("World");
```

### 5. External Processes

#### Rule 5.1: Set Working Directory

```rust
// ❌ DON'T: Assume cwd is writable
Command::new("program").spawn()

// ✅ DO: Set explicit working directory
Command::new("program")
    .current_dir(std::env::temp_dir())
    .spawn()
```

#### Rule 5.2: Handle All Streams

```rust
// ❌ DON'T: Ignore stderr
let output = Command::new("prog")
    .output()?;
let result = String::from_utf8_lossy(&output.stdout);

// ✅ DO: Capture and handle both streams
let mut child = Command::new("prog")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

let mut stdout = child.stdout.take().unwrap();
let mut stderr = child.stderr.take().unwrap();

let mut out = Vec::new();
let mut err = Vec::new();
stdout.read_to_end(&mut out)?;
stderr.read_to_end(&mut err)?;

let status = child.wait()?;
if !status.success() {
    return Err(format!("Failed: {}", String::from_utf8_lossy(&err)));
}
```

#### Rule 5.3: Close stdin Before Reading

```rust
// ❌ DON'T: Read while stdin still open (may hang)
let mut child = Command::new("prog")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
child.stdin.as_mut().unwrap().write_all(b"input")?;
let output = child.wait_with_output()?;  // May deadlock

// ✅ DO: Drop stdin before reading stdout
let mut stdin = child.stdin.take().unwrap();
stdin.write_all(b"input")?;
drop(stdin);  // Close stdin

let mut stdout = child.stdout.take().unwrap();
let mut output = Vec::new();
stdout.read_to_end(&mut output)?;
```

### 6. Regex

#### Rule 6.1: No Lookahead/Lookbehind

```rust
// ❌ DON'T: Use lookahead (not supported in `regex` crate)
Regex::new(r"keyword(?![a-z])")

// ✅ DO: Use word lists and post-processing
const KEYWORDS: &[&str] = &["if", "else", "while"];

fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains(&word.to_lowercase().as_str())
}

fn tokenize(input: &str) -> Vec<Token> {
    let word_re = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    word_re.find_iter(input)
        .map(|m| {
            let word = m.as_str();
            let token_type = if is_keyword(word) {
                TokenType::Keyword
            } else {
                TokenType::Identifier
            };
            Token { token_type, text: word.to_string() }
        })
        .collect()
}
```

#### Rule 6.2: Compile Regex Once

```rust
// ❌ DON'T: Compile regex on every call
fn process(text: &str) -> String {
    let re = Regex::new(r"\d+").unwrap();  // Compiled every time!
    re.replace_all(text, "X").to_string()
}

// ✅ DO: Use lazy_static or LazyLock
use std::sync::LazyLock;

static NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d+").unwrap()
});

fn process(text: &str) -> String {
    NUMBER_RE.replace_all(text, "X").to_string()
}
```

### 7. Tauri Specific

#### Rule 7.1: Use State for Shared Data

```rust
// ❌ DON'T: Use global static mut
static mut COUNTER: usize = 0;

#[tauri::command]
fn increment() -> usize {
    unsafe { COUNTER += 1; COUNTER }  // Unsafe!
}

// ✅ DO: Use Tauri managed state
struct AppState {
    counter: Mutex<usize>,
}

#[tauri::command]
fn increment(state: State<AppState>) -> usize {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    *counter
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { counter: Mutex::new(0) })
        .invoke_handler(tauri::generate_handler![increment])
        .run(tauri::generate_context!())
        .unwrap();
}
```

#### Rule 7.2: Tauri v2 Command Return Types

```rust
// Commands can return:
// - Simple types (String, i32, bool)
// - Structs with #[derive(Serialize)]
// - Result<T, String> for errors

#[derive(Serialize)]
struct MyResult {
    success: bool,
    data: String,
}

#[tauri::command]
fn my_command() -> MyResult {
    MyResult { success: true, data: "OK".into() }
}

#[tauri::command]
fn fallible_command() -> Result<String, String> {
    if something_wrong {
        Err("Something went wrong".into())
    } else {
        Ok("Success".into())
    }
}
```

---

## Code Patterns

### Pattern 1: Builder Pattern for Configuration

```rust
struct Config {
    highlight: bool,
    theme: String,
    verbose: bool,
}

impl Config {
    fn new() -> Self {
        Config {
            highlight: false,
            theme: "default".into(),
            verbose: false,
        }
    }
    
    fn highlight(mut self, v: bool) -> Self {
        self.highlight = v;
        self
    }
    
    fn theme(mut self, v: impl Into<String>) -> Self {
        self.theme = v.into();
        self
    }
}

// Usage
let config = Config::new()
    .highlight(true)
    .theme("dracula");
```

### Pattern 2: Newtype for Type Safety

```rust
// ❌ DON'T: Use raw types that can be confused
fn set_size(width: u32, height: u32)
set_size(100, 200);  // Which is which?

// ✅ DO: Use newtypes
struct Width(u32);
struct Height(u32);

fn set_size(width: Width, height: Height)
set_size(Width(100), Height(200));  // Clear!
```

### Pattern 3: State Machine with Enums

```rust
enum InputState {
    FirstLine,
    Continuation { buffer: String },
    Complete { input: String },
    Cancelled,
}

fn read_input(state: InputState, line: &str) -> InputState {
    match state {
        InputState::FirstLine => {
            if line.is_empty() {
                InputState::FirstLine  // Stay in first line
            } else if line == ".end" {
                InputState::Complete { input: String::new() }
            } else {
                InputState::Continuation { buffer: line.to_string() }
            }
        }
        InputState::Continuation { mut buffer } => {
            if line.is_empty() || line == ".end" {
                InputState::Complete { input: buffer }
            } else {
                buffer.push('\n');
                buffer.push_str(line);
                InputState::Continuation { buffer }
            }
        }
        other => other,
    }
}
```

---

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_keyword() {
        let tokens = tokenize("id f(x) = g(x);");
        assert!(tokens.iter().any(|t| 
            t.token_type == TokenType::Keyword && t.text == "id"
        ));
    }
    
    #[test]
    fn test_validate_balanced_parens() {
        assert!(validate_input("f(x)").is_ok());
        assert!(validate_input("f(x").is_err());
    }
    
    #[test]
    fn test_format_output_removes_timing() {
        let output = "FORM 4.3\n\n   E = x;\n\n  0.00 sec\n";
        let formatted = format_output(output);
        assert!(!formatted.contains("FORM"));
        assert!(!formatted.contains("sec"));
        assert!(formatted.contains("E ="));
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use std::process::Command;

#[test]
fn test_form_execution() {
    let output = Command::new("./target/debug/form-repl")
        .args(&["--help"])
        .output()
        .expect("Failed to execute");
    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("FORM REPL"));
}
```

### Test Naming Convention

```rust
#[test]
fn test_<module>_<function>_<scenario>() { }

// Examples:
fn test_tokenize_handles_empty_input() { }
fn test_format_output_preserves_expression() { }
fn test_validate_input_detects_unclosed_paren() { }
```

---

## Summary Checklist

### Before Committing

- [ ] `cargo fmt` - Format code
- [ ] `cargo clippy` - Check for warnings
- [ ] `cargo test` - Run tests
- [ ] No `unwrap()` in production code (use `?` or handle errors)
- [ ] No hardcoded paths (use env vars or config)
- [ ] Process working directory set if temp files needed
- [ ] Regex patterns don't use lookahead/lookbehind
- [ ] State properly protected with Mutex/Atomic
- [ ] stdin closed before reading stdout in subprocesses
