# FORM REPL v0.2.0

An interactive Read-Eval-Print Loop (REPL) for the FORM computer algebra system with **syntax highlighting** and **IPython-like UX**.

## What's New in v0.2.0

### ✨ Syntax Highlighting
Real syntax highlighting for FORM code:
- **Keywords**: `id`, `repeat`, `if`, `else`, `while`, etc.
- **Declarations**: `Symbol`, `Local`, `CFunction`, `Index`, etc.
- **Built-in functions**: `abs`, `sin`, `cos`, `gcd`, etc.
- **Comments**: Lines starting with `*`
- **Numbers, operators, strings**

### 🐍 IPython-like UX
- **Numbered prompts**: `In [1]:`, `Out[1]:`
- **Magic commands**: `%history`, `%time`, `%who`, `%reset`
- **Persistent history**: Saved across sessions
- **Output tracking**: Access previous outputs

### 🎨 Multiple Themes
8 built-in themes:
- `default`, `solarized-dark`, `monokai`, `dracula`
- `nord`, `gruvbox`, `one-dark`, `none` (plain)

### ⚙️ Configuration File
Create `~/.form_replrc` to customize defaults.

---

## Installation

### Prerequisites
- Rust 1.70+
- FORM executable in PATH or set `FORM_PATH`

### Build
```sh
cargo build --release
```

Binary: `target/release/form-repl`

---

## Quick Start

```sh
# Basic usage
./form-repl

# With syntax highlighting (recommended)
./form-repl -H

# With a specific theme
./form-repl -t dracula

# See all options
./form-repl --help
```

---

## Command Line Options

```
form-repl [OPTIONS]

Options:
  -h, --help          Show help message
  -V, --version       Show version
  -H, --highlight     Enable syntax highlighting
  -t, --theme NAME    Set color theme
  -v, --verbose       Enable debug output
  --list-themes       List available themes
  --sample-config     Print sample config file
```

**Note**: `-h` is for help (standard convention). Use `-H` for highlighting.

---

## User Interface

### IPython-Style Prompts

```
FORM REPL v0.2.0 — Type %help for help, .quit to exit

────────────────────────────────────────────────────────────
In [1]: Symbol x,y;
   ...: Local E = (x+y)^2;
   ...: Print;
   ...: .end

Out[1]:    E =
              x^2 + 2*x*y + y^2;

────────────────────────────────────────────────────────────
In [2]: 
```

### Syntax Highlighting Example

With `--highlight` or `-H`:
- `Symbol` appears in blue (declaration)
- `Local` appears in blue (declaration)  
- `x`, `y`, `E` appear as identifiers
- `Print` appears in magenta (keyword)
- Numbers like `2` appear in purple
- `.end` appears in orange (preprocessor)

---

## REPL Commands

All commands start with `.` and must be on the first line:

| Command | Description |
|---------|-------------|
| `.help` | Show help message |
| `.quit`, `.exit`, `.q` | Exit the REPL |
| `.clear` | Clear current input buffer |

---

## Magic Commands

All magic commands start with `%`:

| Command | Description |
|---------|-------------|
| `%help`, `%?` | Show help |
| `%quit`, `%exit`, `%q` | Exit |
| `%history [N]` | Show last N history entries (default 10) |
| `%time` | Toggle timing display |
| `%who` | List declared symbols |
| `%reset` | Clear session state |
| `%recall [N]` | Recall input from session N |
| `%last`, `%_` | Show last output |
| `%theme` | List available themes |
| `%info` | Show session info |
| `%lsmagic` | List all magic commands |

### Examples

```
In [1]: %history 5
```
Shows the last 5 history entries with inputs and outputs.

```
In [2]: %time
Timing display: ON
```
Toggle timing information after each execution.

```
In [3]: %who
Declared symbols: E, x, y
```
List all symbols declared in the current session.

---

## Configuration File

Create `~/.form_replrc`:

```toml
[settings]
highlight = true
theme = "dracula"
show_timing = false
verbose = false
auto_end = true

[history]
file = "~/.form_repl_history"
max_entries = 1000
save_on_exit = true
```

Generate a sample config:
```sh
./form-repl --sample-config > ~/.form_replrc
```

---

## Themes

| Theme | Style |
|-------|-------|
| `default` | Subtle, balanced colors |
| `none` | No colors (plain text) |
| `solarized-dark` | Ethan Schoonover's Solarized |
| `monokai` | Sublime Text inspired |
| `dracula` | Dark purple theme |
| `nord` | Arctic, bluish colors |
| `gruvbox` | Retro groove |
| `one-dark` | Atom editor inspired |

---

## Examples

### Basic Expression

```
In [1]: Symbol x;
   ...: Local E = x^2 + 2*x + 1;
   ...: Print;
   ...: .end

Out[1]:    E =
              x^2 + 2*x + 1;
```

### Pattern Matching

```
In [2]: CFunction f;
   ...: Symbol x;
   ...: Local E = f(1,2,x,3,4);
   ...: id f(?a,x,?b) = f(?b,?a);
   ...: Print;
   ...: .end

Out[2]:    E =
              f(3,4,1,2);
```

### Using History

```
In [3]: %history 2
In [1]: Symbol x;
        Local E = x^2 + 2*x + 1;
        Print;
Out[1]:    E = ...

In [2]: CFunction f;
        ...
```

---

## Key Bindings

| Key | Action |
|-----|--------|
| Enter | Continue to next line / Submit if empty |
| Ctrl+C | Cancel current input |
| Ctrl+D | Submit or exit |
| Up/Down | Navigate history |
| Ctrl+A | Beginning of line |
| Ctrl+E | End of line |
| Ctrl+L | Clear screen |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `FORM_PATH` | Path to FORM executable |

---

## Troubleshooting

### "Could not find FORM executable"
Set the FORM_PATH environment variable:
```sh
export FORM_PATH=/path/to/form
```

### Colors Not Showing
Ensure your terminal supports 256 colors and use `-H` flag.

### History Not Saving
Check permissions on `~/.form_repl_history`.

---

## Changes from v0.1

1. **Fixed**: `-h` now shows help (was highlight)
2. **Added**: Real syntax highlighting for FORM syntax
3. **Added**: IPython-style `In [N]:`/`Out[N]:` prompts
4. **Added**: Magic commands (`%history`, `%time`, etc.)
5. **Added**: Persistent history across sessions
6. **Added**: Configuration file support
7. **Added**: `FORM_PATH` environment variable
8. **Added**: 8 color themes
9. **Added**: Input validation (bracket matching)
10. **Improved**: Error messages with context

---

## License

Apache-2.0 license

## See Also

- [FORM Official Website](http://www.nikhef.nl/~form)
- [FORM GitHub Repository](https://github.com/vermaseren/form)
