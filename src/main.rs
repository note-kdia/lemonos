#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

use lemonos::efi::{EfiHandle, EfiSimpleTextOutputProtocolWriter, EfiSystemTable};
use lemonos::serial_println;

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: EfiSystemTable) {
    lemonos::init();

    let mut efi_writer = EfiSimpleTextOutputProtocolWriter {
        protocol: efi_system_table.con_out,
    };
    writeln!(efi_writer, "Hello, LemonOS!").unwrap();
    writeln!(efi_writer, "EFI_SYSTEM_TABLE at {:#p}", &efi_system_table).unwrap();

    // serial
    serial_println!("Hello, serial!");

    lemonos::x86_64::rest_in_peace();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    lemonos::x86_64::rest_in_peace();
}
