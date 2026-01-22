# FORM REPL Manual

An interactive Read-Eval-Print Loop (REPL) for the FORM computer algebra system.


![img](./assets/demo.png)

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Command Line Options](#command-line-options)
4. [User Interface](#user-interface)
5. [Input Methods](#input-methods)
6. [REPL Commands](#repl-commands)
7. [Examples](#examples)
8. [Tips and Tricks](#tips-and-tricks)
9. [Configuration](#configuration)
10. [Requirements](#requirements)

---

## Installation

### Prerequisites

- Rust 1.70 or later
- FORM executable (`sources/form` or in PATH)

### Building FORM

If FORM is not yet built:

```sh
cd /path/to/form-4.3.1-x86_64-osx/src/form
cd ..
autoreconf -i
./configure
make
```

### Building the REPL

```sh
cargo build --release
```

The binary will be at `target/release/form-repl`.

---

## Quick Start

```sh
# Start the REPL
./target/release/form-repl

# With syntax highlighting
./target/release/form-repl --highlight

# With a specific theme
./target/release/form-repl --highlight --theme monokai
```

---

## Command Line Options

```
form-repl [OPTIONS]

Options:
  --highlight, -h    Enable syntax highlighting
  --theme, -t NAME   Set color theme (default, solarized-dark, monokai, dracula)
  --verbose, -v      Enable verbose debug output
  --help, -H         Show this help message
```

### Theme Options

| Theme | Description |
|-------|-------------|
| `default` | No colors (monochrome) |
| `solarized-dark` | Solarized dark theme (blue prompt, green output, red error) |
| `monokai` | Monokai color scheme (orange prompt, green output, pink error) |
| `dracula` | Dracula theme (purple prompt, green output, pink error) |

### Examples

```sh
form-repl                     # Basic mode (no colors)
form-repl --highlight         # With colors
form-repl --theme monokai     # Monokai theme
form-repl --verbose           # With debug output
form-repl --help              # Show help message
```

---

## User Interface

### Prompt Styles

The REPL uses session-based prompts with a horizontal separator:

```
────────────────────────────────────────
[1] > Symbol x;
... > Local E = x^2 + 1;
... > Print;
────────────────────────────────────────
[2] > ...
```

- **`[N] >`** - Session prompt (first line of input, N = session number)
- **`... >`** - Continuation prompt (subsequent lines in same session)
- **`────────────────────────────────────────`** - Session separator

### Visual Layout

```
────────────────────────────────────────
[1] > CFunction f;              [Session 1, first line]
... > Symbol x;                 [Continuation line]
... > Local E = f(1,2,x,3,4);   [Continuation line]
... > .end                      [Submit with .end]
   E =
      f(3,4,1,2);
────────────────────────────────────────
[2] >                           [New session, ready for input]
```

### Color Coding (with `--highlight`)

| Area | Color | Example |
|------|-------|---------|
| Session prompts | Theme-dependent | `[1] >` (orange in monokai) |
| Continuation | Theme-dependent | `... >` (yellow in monokai) |
| Output | Green | Expression results |
| Errors | Pink/Bold | Error messages |
| Separator | Theme prompt color | Horizontal line |

---

## Input Methods

### Multi-line Input

The REPL uses **`.end`** to submit multi-line input:

1. **Type a line and press Enter** → Adds line to input buffer, continues to next line
2. **Type `.end` on a line by itself** → Submits all buffered input to FORM
3. **Press Enter on an empty line** → Submits all buffered input (alternative method)
4. **Press Ctrl+D** → Submits immediately (if input buffer has content)

### Input Completion Flow

```
────────────────────────────────────────
[1] > Symbol x;            [Press Enter]
... > Local E = x^2;       [Press Enter]
... > Print;               [Press Enter]
... > .end                 [Submit with .end]
   E =
      x^2;
────────────────────────────────────────
[2] >
```

### Important Notes

- Leading and trailing whitespace is automatically trimmed from each line
- The `.end` directive is required to submit (unlike earlier versions that added it automatically)
- Each session has a unique session number for reference

---

## REPL Commands

All REPL commands start with a dot (`.`) and must be entered on the first line of input.

| Command | Description | Example |
|---------|-------------|---------|
| `.help` | Show help message | `[1] > .help` |
| `.quit` | Exit the REPL | `[1] > .quit` |
| `.exit` | Exit the REPL (alias for `.quit`) | `[1] > .exit` |
| `.clear` | Clear the current input buffer | `[1] > .clear` |

### Command Details

#### `.help`

Displays the help message with all available commands and usage instructions.

```
────────────────────────────────────────
[1] > .help
FORM REPL - Multi-line input mode
  - Type statements, press Enter to continue on next line
  - Press Enter on empty line to submit, or type .end
  - Type .help for commands, .quit to exit

Available commands:
  .help   - Show this help
  .quit   - Exit the REPL
  .clear  - Clear the input buffer
────────────────────────────────────────
[2] >
```

#### `.quit` / `.exit`

Exits the REPL gracefully.

```
────────────────────────────────────────
[1] > .quit
Goodbye!
```

#### `.clear`

Clears the current input buffer and resets line counter. Useful if you made a mistake while typing a multi-line expression.

```
────────────────────────────────────────
[1] > Symbol x;
... > Local E = x^10;
... > Wrong statement
... > .clear       [Clears buffer, starts fresh]
────────────────────────────────────────
[2] >
```

---

## Examples

### Example 1: Basic Arithmetic

```
────────────────────────────────────────
[1] > Symbol x;
... > Local E = x^2 + 2*x + 1;
... > Print;
... > .end
   E =
      x^2 + 2*x + 1;

  0.00 sec out of 0.00 sec
────────────────────────────────────────
[2] >
```

### Example 2: Pattern Matching

```
────────────────────────────────────────
[1] > CFunction f;
... > Symbol x;
... > Local E = f(1,2,x,3,4);
... > id f(?a,x,?b) = f(?b,?a);
... > Print;
... > .end
   E =
      f(3,4,1,2);

  0.00 sec out of 0.00 sec
────────────────────────────────────────
[2] >
```

### Example 3: Fibonacci with Repeat

```
────────────────────────────────────────
[1] > Symbol x,n;
... > Local E = x^10;
... > repeat id x^n?{>1} = x^(n-1) + x^(n-2);
... > Print;
... > .end
   E =
      34 + 55*x;

  0.00 sec out of 0.00 sec
────────────────────────────────────────
[2] >
```

### Example 4: Multi-statement Session

```
────────────────────────────────────────
[1] > Symbol x,y;
... > Local A = x + y;
... > Local B = x - y;
... > Print A, B;
... > .end
   A =
      x + y;

   B =
      x - y;

  0.00 sec out of 0.00 sec
────────────────────────────────────────
[2] >
```

### Example 5: Error Handling

```
────────────────────────────────────────
[1] > Symbol x;
... > Local E = x^;
... > .end
Error: FORM exited with status: exit status: 1
────────────────────────────────────────
[2] >
```

### Example 6: Pasting Multi-line Code

You can paste multi-line FORM code directly into the REPL:

```
────────────────────────────────────────
[1] > CFunction f;
... > Symbol x;
... > Local E = f(1,2,x,3,4);
... > id f(?a,x,?b) = f(?b,?a);
... > Print;
... > .end
   E =
      f(3,4,1,2);

  0.00 sec out of 0.00 sec
────────────────────────────────────────
[2] >
```

---

## Tips and Tricks

### 1. Using Command History

- **Up/Down arrows** - Navigate between commands in history
- History persists within the current session
- Use history to recall and modify previous FORM expressions

### 2. Line Editing

The REPL uses `rustyline` for full-featured line editing:

| Key | Action |
|-----|--------|
| **Left/Right arrows** | Move cursor character by character |
| **Ctrl+A** | Move to beginning of line |
| **Ctrl+E** | Move to end of line |
| **Backspace** | Delete previous character |
| **Ctrl+D** | Delete character at cursor (or submit if line is empty) |
| **Ctrl+L** | Clear screen |
| **Ctrl+C** | Cancel current input |

### 3. Efficient Multi-line Input

Type all lines without waiting for output:

```
────────────────────────────────────────
[1] > Symbol x,y;
... > Local E = (x+y)^5;
... > expand;
... > Print;
... > .end                 [Submit all at once]
────────────────────────────────────────
[2] >
```

### 4. Canceling Input

Press **Ctrl+C** to cancel current input and start fresh:

```
────────────────────────────────────────
[1] > Symbol x;
... > Local E = x^100;
... > ^C       [Cancels and clears buffer]
────────────────────────────────────────
[1] >
```

### 5. Debug Mode

Use `--verbose` flag to see debug information:

```sh
./target/release/form-repl --verbose
```

---

## Configuration

### Environment Variables

| Variable | Description |
|----------|-------------|
| `FORM_PATH` | Path to FORM executable (overrides auto-detection) |

### Terminal Requirements

For best experience, use a terminal that supports:
- ANSI escape codes (for colors with `--highlight`)
- Raw mode terminal input (most modern terminals)
- 256 color mode (for theme colors)

Recommended terminals:
- iTerm2 (macOS)
- Terminal.app (macOS)
- GNOME Terminal (Linux)
- Windows Terminal (Windows)
- Alacritty (cross-platform)

---

## Requirements

### System Requirements

- **Operating System**: macOS, Linux, or Windows (via WSL)
- **Memory**: Minimal (REPL is lightweight)
- **Disk**: ~5 MB for binary

### Software Dependencies

- **Rust**: 1.70.0 or later
- **FORM**: 4.0 or later (included with this distribution)

### Optional Dependencies

For full functionality:
- **GMP** - For arbitrary precision arithmetic
- **MPFR** - For floating-point arithmetic
- **zlib/zstd** - For compression

These are FORM dependencies, not REPL dependencies.

---

## Troubleshooting

### "Could not find FORM executable"

Ensure FORM is built:
```sh
cd /path/to/form-4.3.1-x86_64-osx/src/form
make
```

Or manually specify path:
```sh
export FORM_PATH=/path/to/form
./target/release/form-repl
```

### Colors Not Working

1. Ensure your terminal supports ANSI colors
2. Try a different theme:
   ```sh
   form-repl --highlight --theme default
   ```
3. Check terminal settings for 256 color support

### Input Not Visible

- Ensure terminal supports raw mode
- Try without `--highlight` flag
- Check that stdin is a TTY

### Performance Issues

- Use release build: `cargo build --release`
- Disable highlighting if slow: `./form-repl` (without `--highlight`)

---

## See Also

- [FORM Official Website](http://www.nikhef.nl/~form)
- [FORM GitHub Repository](https://github.com/vermaseren/form)
- [FORM Manual](https://github.com/vermaseren/form/releases)
