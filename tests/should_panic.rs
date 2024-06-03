#![no_std]
#![no_main]

use core::panic::PanicInfo;

use lemonos::{
    qemu::{exit_qemu, ExitCode},
    serial_print, serial_println,
};

#[no_mangle]
pub fn efi_main() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(ExitCode::Failed);
    unreachable!();
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    let one = 1;
    assert_eq!(0, one);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(ExitCode::Success);
    unreachable!();
}
