use core::fmt;

use crate::{efi::EfiSimpleTextOutputProtocolWriter, spinlock::SpinLock};

pub struct GlobalWriter {
    writer: SpinLock<Option<EfiSimpleTextOutputProtocolWriter>>,
}

impl GlobalWriter {
    pub fn set_writer(&self, writer: EfiSimpleTextOutputProtocolWriter) {
        *self.writer.lock() = Some(writer);
    }
}

pub static GLOBAL_WRITER: GlobalWriter = GlobalWriter {
    writer: SpinLock::new(None),
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    if let Some(writer) = &mut *GLOBAL_WRITER.writer.lock() {
        writer.write_fmt(args).unwrap();
    } else {
        panic!("No writer for GLOBAL_WRITER")
    }
}
