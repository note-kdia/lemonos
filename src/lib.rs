#![no_std]
#![no_main]

use self::efi::{EfiSimpleTextOutputProtocolWriter, EfiSystemTable};

pub mod efi;
pub mod error;
pub mod print;
pub mod qemu;
pub mod serial;
pub mod spinlock;
pub mod x86_64;

pub fn init(efi_system_table: &EfiSystemTable) {
    serial::initialize(serial::IO_ADDR_COM1);

    efi_system_table.con_out.clear_screen();

    let efi_writer = EfiSimpleTextOutputProtocolWriter {
        protocol: efi_system_table.con_out,
    };
    print::GLOBAL_WRITER.set_writer(efi_writer);
}
