use crate::plugin::XPLANE_C_CHAR_BUFFER_SIZE;
use std::ffi::CString;
use std::os::raw::c_char;
use tracing::error;

pub fn write_c_char(c_buffer: *mut c_char, r_str: &CString) {
    if c_buffer.is_null() {
        error!("write_c_char: c_buffer is null");
        return;
    }

    let r_str_length = r_str.as_bytes_with_nul().len();
    if r_str_length > XPLANE_C_CHAR_BUFFER_SIZE {
        error!(
            "write_c_char: r_str_length ({}) is greater than c_buffer_size ({})",
            r_str_length, XPLANE_C_CHAR_BUFFER_SIZE
        );
        return;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(r_str.as_ptr(), c_buffer, r_str_length);
    }
}

#[cfg(test)]
mod tests {
    use crate::plugin::XPLANE_C_CHAR_BUFFER_SIZE;
    use crate::safe;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    use std::panic::catch_unwind;

    #[test]
    fn test_write_c_char() {
        let mut c_buffer = vec![0; XPLANE_C_CHAR_BUFFER_SIZE];
        let r_str = CString::new("hello").unwrap();
        safe::write_c_char(c_buffer.as_mut_ptr(), &r_str);
        let c_str = unsafe { CStr::from_ptr(c_buffer.as_ptr() as *const c_char) };
        assert_eq!(
            c_str.to_str().unwrap(),
            r_str.to_str().unwrap(),
            "test failed: c_str should be equal to r_str"
        );
        assert_eq!(c_buffer[5], 0, "test failed: c_buffer[5] should be 0");
        assert!(c_buffer[6..].iter().all(|&c| c == 0), "test failed: c_buffer[6..] should be 0");
    }

    #[test]
    fn test_write_c_char_with_null_ptr() {
        let r_str = CString::new("hello").unwrap();
        let result = catch_unwind(|| safe::write_c_char(std::ptr::null_mut(), &r_str));
        assert!(result.is_ok(), "test failed: write_c_char should not panic when c_buffer is null");
    }

    #[test]
    fn test_write_c_char_with_oversized_string() {
        let mut c_buffer = vec![0; XPLANE_C_CHAR_BUFFER_SIZE];
        let c_buffer_copy = c_buffer.clone();
        let r_str = CString::new("hello".repeat(XPLANE_C_CHAR_BUFFER_SIZE)).unwrap();
        safe::write_c_char(c_buffer.as_mut_ptr(), &r_str);
        assert_eq!(c_buffer, c_buffer_copy, "test failed: c_buffer should not be modified");
    }
}
