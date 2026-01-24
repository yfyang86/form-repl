# FORM REPL GUI

A modern graphical user interface for the FORM computer algebra system, built with [Tauri](https://tauri.app/).

![FORM REPL GUI](screenshot.png)

## Features

- 🎨 **Modern Dark Theme** - Easy on the eyes with a Catppuccin-inspired color scheme
- ✨ **Syntax Highlighting** - Color-coded FORM code for better readability
- 📝 **Multi-line Input** - Write complex FORM programs with proper formatting
- 📜 **Session History** - Navigate through previous commands with Ctrl+↑/↓
- ⚡ **Fast Execution** - Native performance with Tauri's Rust backend
- 🖥️ **Cross-Platform** - Works on Windows, macOS, and Linux

## Prerequisites

1. **Rust** (1.77 or later)
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Tauri CLI v2**
   ```sh
   cargo install tauri-cli --version "^2"
   ```

3. **FORM** - The FORM executable must be available
   - Either in your PATH, or
   - Set the `FORM_PATH` environment variable

### Platform-specific Dependencies

**macOS:**
```sh
xcode-select --install
```

**Ubuntu/Debian:**
```sh
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows:**
- Install Visual Studio Build Tools with C++ workload

## Installation

1. **Clone or extract the project:**
   ```sh
   cd form-repl-gui
   ```

2. **Build and run in development mode:**
   ```sh
   cargo tauri dev
   ```
   Note: A terminal window will be visible in dev mode.

3. **Build for production (no terminal window):**
   ```sh
   cargo tauri build
   ```

   The built application will be in `src-tauri/target/release/bundle/`

### Running Without Terminal Window

**macOS:**
```sh
# Build the app bundle
cargo tauri build

# Run from Finder or use:
open src-tauri/target/release/bundle/macos/FORM\ REPL.app
```

**Windows:**
The release build automatically hides the console window.

**Linux:**
```sh
cargo tauri build
./src-tauri/target/release/form-repl-gui
```

## Usage

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Enter` | Execute code |
| `Ctrl+L` | Clear output |
| `Ctrl+↑` | Previous command in history |
| `Ctrl+↓` | Next command in history |
| `Tab` | Insert 4 spaces |

### Example Session

1. Type FORM code in the input area:
   ```
   Symbol x,y;
   Local E = (x+y)^3;
   Print;
   ```

2. Press `Ctrl+Enter` or click "Run"

3. View the output in the output area:
   ```
   E =
      x^3 + 3*x^2*y + 3*x*y^2 + y^3;
   ```

### Setting FORM Path

If FORM is not in your PATH, set the environment variable before running:

**macOS/Linux:**
```sh
export FORM_PATH=/path/to/form
cargo tauri dev
```

**Windows (PowerShell):**
```powershell
$env:FORM_PATH = "C:\path\to\form.exe"
cargo tauri dev
```

## Project Structure

```
form-repl-gui/
├── src/
│   └── index.html          # Frontend (HTML + CSS + JS)
├── src-tauri/
│   ├── src/
│   │   └── main.rs         # Rust backend
│   ├── capabilities/
│   │   └── default.json    # Tauri v2 permissions
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── build.rs            # Build script
└── README.md
```

## Customization

### Changing the Theme

Edit the CSS variables in `src/index.html`:

```css
:root {
    --bg-primary: #1e1e2e;
    --bg-secondary: #313244;
    --accent-blue: #89b4fa;
    /* ... */
}
```

### Window Size

Edit `src-tauri/tauri.conf.json`:

```json
"windows": [
  {
    "width": 900,
    "height": 700,
    "minWidth": 600,
    "minHeight": 400
  }
]
```

## Troubleshooting

### "FORM not found" Error

1. Ensure FORM is installed and executable
2. Check if `form --version` works in terminal
3. Set `FORM_PATH` to the full path of the FORM executable

### Build Errors on Linux

Install all required dependencies:
```sh
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### Blank Window on Windows

Ensure WebView2 Runtime is installed:
https://developer.microsoft.com/en-us/microsoft-edge/webview2/

## License

MIT License

## See Also

- [FORM Official Website](http://www.nikhef.nl/~form)
- [Tauri Documentation](https://tauri.app/v1/guides/)
- [FORM REPL CLI version](../form-repl-improved/)
