use core::{fmt, fmt::Write};

use spin::Mutex;

use crate::kernel::syscall::user_api::usr_kernel_print;

// The global writer that can used as an interface from other modules
// It is threadsafe by using 'Mutex'
pub static WRITER: Mutex<Writer> = Mutex::new(Writer {});

// Defining a Writer for writing formatted strings to the CGA screen
pub struct Writer {}

// Implementation of the 'core::fmt::Write' trait for our Writer
// Required to output formatted strings
// Requires only one function 'write_str'
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        usr_kernel_print(s.as_ptr(), s.len());
        return Ok(());
    }
}

// Provide macros like in the 'io' module of Rust
// The $crate variable ensures that the macro also works
// from outside the 'std' crate.
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        $crate::kernel::kprint::kprint(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => {
        $crate::kernel::kprint::kprint(format_args!(concat!($fmt, "\n")));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::kernel::kprint::kprint(format_args!(concat!($fmt, "\n"), $($arg)*));
    };
}

// Helper function of print macros (must be public)
pub fn kprint(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
