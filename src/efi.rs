use core::fmt;
use core::pin::Pin;

pub type EfiHandle = usize;

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
        extern "win64" fn(this: &EfiSimpleTextOutputProtocol, str: *const u16) -> EfiStatus,
    test_string: EfiHandle,
    query_mode: EfiHandle,
    set_mode: EfiHandle,
    set_attribute: EfiHandle,
    clear_screen: EfiHandle,
    set_cursor_position: EfiHandle,
    enable_cursor: EfiHandle,
    mode: EfiHandle,
}

/// UEFIの画面にテキストを表示する
///
/// # Examples
/// ```
/// let mut efi_writer = EfiSimpleTextOutputProtocolWriter {
///     protocol: efi_system_table.con_out,
/// };
/// writeln!(efi_writer, "Hello, {}", "world!").unwrap(); // Hello, world!
/// ```
///
pub struct EfiSimpleTextOutputProtocolWriter {
    pub protocol: Pin<&'static EfiSimpleTextOutputProtocol>,
}

impl EfiSimpleTextOutputProtocolWriter {
    pub fn _write_char(&self, c: u8) {
        let c16: [u16; 2] = [c.into(), 0];
        (self.protocol.output_string)(&self.protocol, c16.as_ptr());
    }

    // TODO; `self.protocol.output_string`に，ヌル終端されたCHAR16のポインタを直接渡す
    pub fn _write_str(&self, str: &str) {
        for c in str.bytes() {
            if c == b'\n' {
                // use CRLF
                self._write_char(b'\r');
            }
            self._write_char(c);
        }
    }
}

impl fmt::Write for EfiSimpleTextOutputProtocolWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self._write_str(s);
        Ok(())
    }
}

// https://uefi.org/specs/UEFI/2.9_A/04_EFI_System_Table.html#id6
#[repr(C)]
pub struct EfiSystemTable {
    pub header: EfiTableHeader,
    pub firmware_vendor: EfiHandle, // *CHAR16
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    pub con_in: EfiHandle, // *EFI_SIMPLE_TEXT_INPUT_PROTOCOL
    pub console_out_handle: EfiHandle,
    pub con_out: Pin<&'static EfiSimpleTextOutputProtocol>,
    pub standard_error_handle: EfiHandle,
    pub stderr: EfiHandle,            // *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
    pub runtime_services: EfiHandle,  // *EFI_RUNTIME_SERVICES
    pub efi_boot_services: EfiHandle, // *EFI_BOOT_SERVICES
    pub number_of_table_entries: usize,
    pub configuration_table: EfiHandle, // *EFI_CONFIGURATION_TABLE
}
