#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use lemonos::efi::{EfiHandle, EfiSystemTable};
use lemonos::println;
use lemonos::serial_println;

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
    serial_println!("Error: {}\n", info);
    lemonos::x86_64::rest_in_peace();
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        use lemonos::{serial_print, serial_println};
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use lemonos::qemu::{exit_qemu, ExitCode};
    use lemonos::serial_println;

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(ExitCode::Failed);
    lemonos::x86_64::rest_in_peace();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    use lemonos::qemu::{exit_qemu, ExitCode};
    use lemonos::serial_println;

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(ExitCode::Success)
}

#[test_case]
fn trivial_assertion() {
    let one = 1;
    assert_eq!(1, one);
}
