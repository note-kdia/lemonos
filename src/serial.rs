use crate::x86_64;
use core::arch::asm;
use core::fmt;

pub const IO_ADDR_COM1: u16 = 0x03f8;
const BAUD_DIVISOR: u16 = 0x0001; // baud rate = 115200 / BAUD_DIVISOR

pub fn initialize(base_io_addr: u16) {
    x86_64::write_io_port(base_io_addr + 1, 0x00); // Disable all interrupts
    x86_64::write_io_port(base_io_addr + 3, 0x80); // Enable DLAB (set baud rate divisor)
    x86_64::write_io_port(base_io_addr, (BAUD_DIVISOR & 0xff).try_into().unwrap());
    x86_64::write_io_port(base_io_addr + 1, (BAUD_DIVISOR >> 8).try_into().unwrap());
    x86_64::write_io_port(base_io_addr + 3, 0x03); // 8 bits, no parity, one stop bit
    x86_64::write_io_port(base_io_addr + 2, 0xc7); // Enable FIFO, clear them, with 14-byte
                                                   // threshold
    x86_64::write_io_port(base_io_addr + 4, 0x0b); // IRQs enabled, RTS/DSR set
}

pub fn send_char(base_io_addr: u16, c: char) {
    while (x86_64::read_io_port(base_io_addr + 5) & 0x20) == 0 {
        unsafe { asm!("pause") }
    }
    x86_64::write_io_port(base_io_addr, c as u8)
}

pub fn send_str(base_io_addr: u16, s: &str) {
    let sc = s.chars();
    for c in sc {
        send_char(base_io_addr, c);
    }
}

pub struct SerialWriter {}

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        send_str(IO_ADDR_COM1, s);
        Ok(())
    }
}

#[macro_export]
macro_rules! serial_print {
        ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
        () => ($crate::print!("\n"));
            ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut writer = crate::serial::SerialWriter {};
    fmt::write(&mut writer, args).unwrap();
}
