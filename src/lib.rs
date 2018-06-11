extern crate libc;

use std::ffi::CStr;
use std::str;
use std::sync::{Once, ONCE_INIT};

mod engine;
mod error;
mod ffi;
mod scan_settings;
pub mod version;

pub use engine::*;
pub use error::*;
pub use scan_settings::*;

/// Initializes clamav
///
/// This must be called once per process. This is safe to call multiple times.
pub fn initialize() -> Result<(), ClamError> {
    // the cl_init implementation isn't thread-safe, which is painful for tests
    static ONCE: Once = ONCE_INIT;
    static mut RESULT: ffi::cl_error = ffi::cl_error::CL_SUCCESS;
    unsafe {
        ONCE.call_once(|| {
            let result = ffi::cl_init(ffi::CL_INIT_DEFAULT);
            // copy so it's safe to use outside this fn
            RESULT = result;
        });
        match RESULT {
            ffi::cl_error::CL_SUCCESS => Ok(()),
            _ => Err(ClamError::new(RESULT)),
        }
    }
}

/// Gets the default database directory for clamav
pub fn default_database_directory() -> String {
    unsafe {
        let ptr = ffi::cl_retdbdir();
        let bytes = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8(bytes)
            .ok()
            .expect("Invalid UTF8 string")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_success() {
        assert!(initialize().is_ok(), "initialize should succeed");
    }

    #[test]
    fn default_database_directory_success() {
        initialize().expect("initialize should succeed");
        assert!(
            default_database_directory().len() > 0,
            "should have a default db dir"
        );
    }
}
