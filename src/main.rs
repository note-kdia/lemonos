#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use lemonos::efi::{EfiHandle, EfiSystemTable};
use lemonos::println;

fn main() -> ! {
    println!("Enter main()");

    lemonos::x86_64::rest_in_peace();
}

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: EfiSystemTable) {
    lemonos::init(&efi_system_table);

    #[cfg(test)]
    test_main();

    println!("Loading LemonOS");
    println!("EFI_SYSTEM_TABLE at {:#p}", &efi_system_table);

    main();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    lemonos::x86_64::rest_in_peace();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    lemonos::x86_64::rest_in_peace();
}
