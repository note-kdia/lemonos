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

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    lemonos::x86_64::rest_in_peace();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use lemonos::qemu::{exit_qemu, ExitCode};

    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(ExitCode::Failed);
    lemonos::x86_64::rest_in_peace();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use lemonos::qemu::{exit_qemu, ExitCode};

    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(ExitCode::Success)
}

#[test_case]
fn trivial_assertion() {
    use lemonos::print;

    print!("trivial assertion...");
    let one = 1;
    assert_eq!(1, one);
    println!("[ok]");
}
