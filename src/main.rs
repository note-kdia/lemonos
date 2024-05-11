#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use core::pin::Pin;

type EfiHandle = usize;

#[repr(C)]
pub enum EfiStatus {
    Success = 0,
}

// https://uefi.org/specs/UEFI/2.9_A/04_EFI_System_Table.html#id4
#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    _reserved: u32,
}

// https://uefi.org/specs/UEFI/2.9_A/12_Protocols_Console_Support.html#efi-simple-text-output-protocol
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    reset: EfiHandle,
    output_string:
        extern "win64" fn(this: *const EfiSimpleTextOutputProtocol, str: *const u16) -> EfiStatus,
    test_string: EfiHandle,
    query_mode: EfiHandle,
    set_mode: EfiHandle,
    set_attribute: EfiHandle,
    clear_screen: EfiHandle,
    set_cursor_position: EfiHandle,
    enable_cursor: EfiHandle,
    mode: EfiHandle,
}

impl EfiSimpleTextOutputProtocol {
    pub fn write_char(&self, c: u8) {
        let c16: [u16; 2] = [c.into(), 0];
        (self.output_string)(self, c16.as_ptr());
    }

    // TODO; `self.output_string`に，ヌル終端されたCHAR16のポインタを直接渡す
    pub fn write_string(&self, str: &str) {
        for c in str.bytes() {
            if c == b'\n' {
                // use CRLF
                self.write_char(b'\r');
            }
            self.write_char(c);
        }
    }
}

// https://uefi.org/specs/UEFI/2.9_A/04_EFI_System_Table.html#id6
#[repr(C)]
pub struct EfiSystemTable {
    header: EfiTableHeader,
    firmware_vendor: EfiHandle, // *CHAR16
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    con_in: EfiHandle, // *EFI_SIMPLE_TEXT_INPUT_PROTOCOL
    console_out_handle: EfiHandle,
    con_out: Pin<&'static EfiSimpleTextOutputProtocol>,
    standard_error_handle: EfiHandle,
    stderr: EfiHandle,            // *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
    runtime_services: EfiHandle,  // *EFI_RUNTIME_SERVICES
    efi_boot_services: EfiHandle, // *EFI_BOOT_SERVICES
    number_of_table_entries: usize,
    configuration_table: EfiHandle, // *EFI_CONFIGURATION_TABLE
}

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: EfiSystemTable) {
    efi_system_table.con_out.get_ref().write_string("LemonOS\n");
    efi_system_table
        .con_out
        .get_ref()
        .write_string("Hello, world!\n");
    loop {
        unsafe { asm!("hlt") }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}
