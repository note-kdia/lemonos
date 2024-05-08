#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
fn efi_main() {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
