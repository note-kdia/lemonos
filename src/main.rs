#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

use lemonos::efi::{EfiHandle, EfiSystemTable};

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: EfiSystemTable) {
    efi_system_table.con_out.get_ref().write_string("LemonOS\n");
    efi_system_table
        .con_out
        .get_ref()
        .write_string("Hello, world!\n");
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
