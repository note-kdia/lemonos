use core::arch::asm;

/// # Safety
///
/// Writing to wrong io port will cause undefined behavior.
pub unsafe fn write_io_port(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al",
             in("al") data,
             in("dx") port)
    }
}

pub fn read_io_port(port: u16) -> u8 {
    let mut data: u8;
    unsafe {
        asm!("in al, dx",
             out("al") data,
             in("dx") port)
    }
    data
}

pub fn hlt() {
    unsafe { asm!("hlt") }
}

pub fn rest_in_peace() -> ! {
    loop {
        unsafe { asm!("cli;hlt;") }
    }
}
