const QEMU_EXIT_PORT: u16 = 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10, // QEMU will exit with status 33
    Failed = 0x11,  // QEMU will exit with status 35
}

pub fn exit_qemu(exit_code: ExitCode) {
    use crate::x86_64::write_io_port;

    let mut exit_code_bytes = [0; 4];
    exit_code_bytes.copy_from_slice(&(exit_code as u32).to_le_bytes());

    for (i, byte) in exit_code_bytes.iter().enumerate() {
        unsafe { write_io_port(QEMU_EXIT_PORT + (i as u16), *byte) }
    }
}
