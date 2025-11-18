use std::ffi::CString;
use std::os::raw::c_char;

pub fn copy(src: &CString, dst: *mut c_char) {
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst, src.as_bytes_with_nul().len());
    }
}
