// Terminal utilities - minimal now that we use rustyline
use std::sync::atomic::{AtomicBool, Ordering};

// Thread-safe verbose flag using AtomicBool instead of unsafe static mut
// This prevents data races and follows Rust's safety guarantees
pub static VERBOSE: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn verbose_println(msg: &str) {
    if VERBOSE.load(Ordering::Relaxed) {
        eprintln!("{}", msg);
    }
}

/// Macro for conditional verbose printing
/// Uses atomic operations for thread-safe access to VERBOSE flag
#[macro_export]
macro_rules! vprintln {
    () => {
        if $crate::modules::term::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) {
            eprintln!();
        }
    };
    ($($arg:tt)*) => {
        if $crate::modules::term::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) {
            eprintln!($($arg)*);
        }
    };
}
