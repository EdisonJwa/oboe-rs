use oboe_sys as ffi;

pub struct Version;

impl Version {
    pub const MAJOR: u8 = ffi::oboe_Version_Major;

    pub const MINOR: u8 = ffi::oboe_Version_Minor;

    pub const PATCH: u16 = ffi::oboe_Version_Patch;

    pub const NUMBER: u32 = ffi::oboe_Version_Number;

    pub fn text() -> &'static str {
        let bytes = ffi::oboe_Version_Text.as_ref();
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        std::str::from_utf8(&bytes[..end]).unwrap_or("")
    }
}
