#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::efi::{EfiSimpleTextOutputProtocolWriter, EfiSystemTable};
use core::panic::PanicInfo;

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

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        use crate::{serial_print, serial_println};
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::qemu::{exit_qemu, ExitCode};
    use crate::serial_println;

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(ExitCode::Success)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    use crate::qemu::{exit_qemu, ExitCode};
    use crate::serial_println;

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(ExitCode::Failed);

    unreachable!();
}

#[cfg(test)]
#[no_mangle]
fn efi_main(_image_handle: crate::efi::EfiHandle, _efi_system_table: EfiSystemTable) {
    test_main();
    unreachable!();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}
