use std::ffi::{c_char, CStr, CString};

pub unsafe fn str_rs_to_c(content: String) -> *mut c_char {
    let c_str = CString::new(content).expect("CString::new failed");

    c_str.into_raw()
}

pub unsafe fn str_c_to_rs(content_ptr: *mut c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(content_ptr) };

    c_str.to_str()
        .expect("Invalid UTF-8 sequence")
        .to_owned()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_c_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    let _ = unsafe { CString::from_raw(ptr) };
}