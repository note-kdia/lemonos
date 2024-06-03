#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lemonos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use lemonos::efi::{EfiHandle, EfiSystemTable};
use lemonos::print;

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: EfiSystemTable) -> ! {
    lemonos::init(&efi_system_table);

    test_main();

    unreachable!();
}

#[test_case]
fn test_print() {
    print!("test_print output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lemonos::test_panic_handler(info)
}
