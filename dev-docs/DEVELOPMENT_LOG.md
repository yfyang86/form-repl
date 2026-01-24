# FORM REPL Development Logs & Rules

## Project Overview

- **Project**: FORM REPL - Interactive environment for FORM computer algebra system
- **Repository**: https://github.com/form-dev/form
- **Languages**: Rust, HTML/CSS/JavaScript
- **Frameworks**: Tauri v2 (GUI), rustyline (CLI)

---

## Development Log

### Session: 2026-01-24

#### 1. Code Review Completed

**Issues Identified:**

| Priority | Issue | Resolution |
|----------|-------|------------|
| Critical | `-h` flag conflicted with `--help` convention | Changed highlight to `-H`, help to `-h` |
| High | No actual syntax highlighting (only prompt colors) | Implemented full FORM lexer |
| High | Missing `FORM_PATH` env var support | Added to `find_form_executable()` |
| Medium | Regex lookahead not supported in Rust | Replaced with word lists + string matching |
| Medium | FORM temp file error in bundled app | Set `current_dir()` to system temp |
| Low | Code duplication in separators | Extracted helper functions |

#### 2. CLI Version (form-repl-improved)

**New Features:**
- ✅ Real syntax highlighting for FORM language
- ✅ IPython-style prompts (`In [N]:` / `Out[N]:`)
- ✅ Magic commands (`%history`, `%time`, `%who`, `%reset`)
- ✅ Persistent history across sessions
- ✅ Configuration file support (`~/.form_replrc`)
- ✅ 8 color themes
- ✅ Input validation (bracket matching)

**Files Created:**
```
src/
├── main.rs           # Main REPL loop
└── modules/
    ├── mod.rs        # Module exports
    ├── config.rs     # Config file handling
    ├── form.rs       # FORM execution
    ├── highlight.rs  # Syntax highlighting
    ├── magic.rs      # Magic commands
    ├── term.rs       # Terminal utilities
    └── theme.rs      # Color themes
```

#### 3. GUI Version (form-repl-gui)

**Features:**
- ✅ Tauri v2 application
- ✅ Dark theme UI (Catppuccin-inspired)
- ✅ Syntax highlighting for input
- ✅ Session history
- ✅ Keyboard shortcuts
- ✅ No terminal window in release build

**Files Created:**
```
src/
└── index.html        # Single-file frontend
src-tauri/
├── Cargo.toml
├── tauri.conf.json   # Tauri v2 config
├── build.rs
├── capabilities/
│   └── default.json  # Permissions
└── src/
    └── main.rs       # Rust backend
```

#### 4. Bugs Fixed

| Bug | Cause | Fix |
|-----|-------|-----|
| Regex panic on startup | Rust `regex` crate doesn't support lookahead | Use word lists instead of regex |
| "Tauri API not available" | API not initialized on DOM ready | Added retry loop for `window.__TAURI__` |
| FORM temp file error | App ran from `/Applications` (read-only) | Set `current_dir()` to temp directory |
| Broken HTML in output | Regex replaced `-` in class names | Disabled output highlighting |
| FORM stats showing | Filter didn't catch all metadata | Added filters for "Terms in output", "Bytes used" |

---

## Development Rules

### 1. Rust Rules

#### 1.1 Regex
```rust
// ❌ DON'T: Use lookahead/lookbehind (not supported)
Regex::new(r"(?i)^(keyword)(?![a-zA-Z])").unwrap();

// ✅ DO: Use word lists and manual checking
const KEYWORDS: &[&str] = &["if", "else", "repeat"];
fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains(&word.to_lowercase().as_str())
}
```

#### 1.2 Temporary Values
```rust
// ❌ DON'T: Return reference to temporary
if condition { &format!("{}text", prefix) } else { "" }

// ✅ DO: Use let binding
let text = if condition {
    format!("{}text", prefix)
} else {
    String::new()
};
```

#### 1.3 External Process Execution
```rust
// ❌ DON'T: Assume current directory is writable
Command::new(path).spawn()

// ✅ DO: Set working directory for apps that create temp files
Command::new(path)
    .current_dir(std::env::temp_dir())
    .spawn()
```

#### 1.4 String Type Consistency
```rust
// ❌ DON'T: Mix &String and &str in tuples
let (a, b) = if cond { (&string, "text") } else { ("", "") };

// ✅ DO: Use consistent types
let a = if cond { string.as_str() } else { "" };
let b = if cond { "text" } else { "" };
```

### 2. Tauri Rules

#### 2.1 Version Compatibility
```json
// Tauri v1 config
{
  "package": { "productName": "App" },
  "tauri": { "windows": [...] }
}

// Tauri v2 config
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "App",
  "app": { "windows": [...] }
}
```

#### 2.2 API Initialization
```javascript
// ❌ DON'T: Assume API is ready immediately
const { invoke } = window.__TAURI__.tauri;

// ✅ DO: Wait for API with retries
async function initializeTauri() {
    for (let i = 0; i < 10; i++) {
        if (window.__TAURI__?.core?.invoke) {
            invoke = window.__TAURI__.core.invoke;
            return;
        }
        await new Promise(r => setTimeout(r, 50));
    }
}
```

#### 2.3 Console Window
```rust
// Hide console in release builds (Windows)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

#### 2.4 Tauri v2 Dependencies
```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-shell = "2"

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

### 3. JavaScript Rules

#### 3.1 HTML Escaping Before Highlighting
```javascript
// ❌ DON'T: Apply regex to raw text then escape
highlightCode(code);
escapeHtml(highlighted);

// ✅ DO: Escape first, then highlight (or highlight carefully)
const escaped = escapeHtml(code);
const highlighted = highlightCode(escaped);
```

#### 3.2 Regex Safety with HTML
```javascript
// ❌ DON'T: Replace characters that appear in HTML
result.replace(/([+\-*^])/g, '<span>$1</span>');
// This breaks: class="hl-number" → class="hl<span>-</span>number"

// ✅ DO: Be specific or skip problematic patterns
// Option 1: Skip
function highlightOutput(output) {
    return output; // Plain text is fine
}
// Option 2: Use word boundaries
result.replace(/\b(\d+)\b/g, '<span class="num">$1</span>');
```

### 4. FORM Integration Rules

#### 4.1 Input Format
```rust
// Always ensure .end directive
let input = if !code.trim_end().ends_with(".end") {
    format!("{}\n.end\n", code)
} else {
    format!("{}\n", code)
};
```

#### 4.2 Output Filtering
```rust
// Filter these patterns from FORM output:
// - "FORM X.X" (version line)
// - "Run at:" (timestamp)
// - "Generated terms" (statistics)
// - "Terms in output" (statistics)
// - "Bytes used" (statistics)
// - "X.XX sec out of" (timing)
// - "Time =" (timing)
```

#### 4.3 Working Directory
```rust
// FORM creates temp files like ./xform*
// Must run in writable directory
Command::new(form_path)
    .current_dir(std::env::temp_dir())
```

---

## Testing Checklist

### CLI Version
- [ ] `cargo build --release` succeeds
- [ ] `-h` shows help
- [ ] `-H` enables highlighting
- [ ] Basic FORM code executes
- [ ] Multi-line input works
- [ ] History navigation works
- [ ] Magic commands work (`%history`, `%time`)

### GUI Version
- [ ] `cargo tauri build` succeeds
- [ ] App launches without terminal
- [ ] FORM path detected or error shown
- [ ] Code execution works
- [ ] Output displays correctly
- [ ] No HTML corruption in output
- [ ] Keyboard shortcuts work (Ctrl+Enter)

### FORM Compatibility
- [ ] Simple expression: `Symbol x; Local E = x^2; Print;`
- [ ] Pattern matching: `id f(?a,x,?b) = f(?b,?a);`
- [ ] Repeat loop: `repeat id x^n?{>1} = x^(n-1) + x^(n-2);`
- [ ] Error handling: Invalid syntax shows error

---

## File Locations

| Item | Path |
|------|------|
| CLI Source | `form-repl-improved/src/` |
| GUI Source | `form-repl-gui/src-tauri/src/` |
| GUI Frontend | `form-repl-gui/src/index.html` |
| CLI Config | `~/.form_replrc` |
| History | `~/.form_repl_history` |
| FORM Temp | System temp directory |

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | - | Original CLI version |
| 0.2.0 | 2026-01-24 | Syntax highlighting, IPython UX, bug fixes |
| GUI 0.1.0 | 2026-01-24 | Initial Tauri GUI version |

---

## Known Limitations

1. **No persistent state between FORM executions** - Each execution is independent
2. **No auto-completion** - Could be added with rustyline helpers
3. **No FORM error line highlighting** - FORM error messages don't always include line numbers
4. **Theme switching requires restart** - Runtime theme switching not implemented

---

## Future Improvements

- [ ] Add tab completion for FORM keywords
- [ ] Implement runtime theme switching in GUI
- [ ] Add file open/save functionality
- [ ] Add multiple tabs for different sessions
- [ ] Add FORM documentation lookup
- [ ] Package FORM executable with the app
