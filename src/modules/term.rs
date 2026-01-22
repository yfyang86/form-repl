// Terminal utilities - minimal now that we use rustyline
pub static mut VERBOSE: bool = false;

#[inline]
pub fn verbose_println(msg: &str) {
    unsafe {
        if VERBOSE {
            eprintln!("{}", msg);
        }
    }
}

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
