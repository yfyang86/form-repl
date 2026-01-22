// Terminal handling for raw mode
use nix::sys::termios::{self, SetArg, SpecialCharacterIndices, LocalFlags};
use std::io::{self, Read, Write};

pub const ERASE_TO_END: &str = "\x1b[K";
pub const ERASE_SCREEN: &str = "\x1b[2J\x1b[H";
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

// Verbose flag - can be set at runtime
pub static mut VERBOSE: bool = false;

#[inline]
pub fn verbose_println(msg: &str) {
    unsafe {
        if VERBOSE {
            eprintln!("{}", msg);
        }
    }
}

// Variadic verbose logging macro - works across modules
#[macro_export]
macro_rules! vprintln {
    () => {
        if unsafe { $crate::modules::term::VERBOSE } {
            eprintln!();
        }
    };
    ($($arg:tt)*) => {
        if unsafe { $crate::modules::term::VERBOSE } {
            eprintln!($($arg)*);
        }
    };
}

pub struct RawModeGuard(termios::Termios);

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = termios::tcsetattr(io::stdin(), SetArg::TCSAFLUSH, &self.0);
    }
}

pub fn enable_raw_mode() -> Result<RawModeGuard, io::Error> {
    let original = termios::tcgetattr(io::stdin())?;
    let mut raw = original.clone();

    // Disable canonical mode (line buffering) and echo
    raw.local_flags &= !(LocalFlags::ICANON | LocalFlags::ECHO | LocalFlags::ISIG);

    // VMIN=1, VTIME=0: block until at least 1 byte available
    raw.control_chars[SpecialCharacterIndices::VMIN as usize] = 1;
    raw.control_chars[SpecialCharacterIndices::VTIME as usize] = 0;

    termios::tcsetattr(io::stdin(), SetArg::TCSAFLUSH, &raw)?;

    Ok(RawModeGuard(original))
}

// Read a single character from stdin, blocking until available
pub fn read_char() -> io::Result<u8> {
    let mut buf = [0u8; 1];
    loop {
        match io::stdin().read(&mut buf[..1]) {
            Ok(1) => return Ok(buf[0]),
            Ok(_) => return Ok(buf[0]), // Handle any positive count
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}

// Read escape sequence after ESC character
pub fn read_escape_sequence() -> String {
    let mut seq = String::new();
    loop {
        match read_char() {
            Ok(b) => {
                let c = b as char;
                if c.is_ascii_digit() || c == ';' || c == '?' || (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') {
                    seq.push(c);
                    if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') {
                        break;
                    }
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
    format!("[{}", seq)
}

// Print a character to stdout
pub fn print_char(ch: char) {
    print!("{}", ch);
    let _ = std::io::stdout().flush();
}

// Handle backspace visually
pub fn handle_backspace() {
    print!("\x08 \x08");
    let _ = std::io::stdout().flush();
}
