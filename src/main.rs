#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lemonos::test_runner)]
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

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use lemonos::serial_println;
    serial_println!("Error: {}\n", info);
    lemonos::x86_64::rest_in_peace();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lemonos::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    let one = 1;
    assert_eq!(1, one);
}
