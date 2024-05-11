#![no_std]
#![no_main]

pub mod efi;
pub mod serial;
pub mod x86_64;

pub fn init() {
    serial::initialize(serial::IO_ADDR_COM1);
}
