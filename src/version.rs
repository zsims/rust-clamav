use std::ffi::CStr;
use std::str;

use ffi;

/// Returns the database version level that the engine supports
pub fn flevel() -> u32 {
    unsafe {
        ffi::cl_retflevel()
    }
}

/// Gets the clamav engine version
pub fn version() -> String {
    unsafe {
        let ptr = ffi::cl_retver();
        let bytes = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8(bytes)
            .ok()
            .expect("Invalid UTF8 string")
            .to_string()
    }
}