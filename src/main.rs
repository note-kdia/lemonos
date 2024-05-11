#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;

use lemonos::efi::{EfiHandle, EfiSimpleTextOutputProtocolWriter, EfiSystemTable};

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: EfiSystemTable) {
    let mut efi_writer = EfiSimpleTextOutputProtocolWriter {
        protocol: efi_system_table.con_out,
    };
    writeln!(efi_writer, "Hello, LemonOS!").unwrap();
    writeln!(efi_writer, "EFI_SYSTEM_TABLE at {:#p}", &efi_system_table).unwrap();

    loop {
        unsafe { asm!("hlt") }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}
