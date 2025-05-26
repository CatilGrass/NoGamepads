use std::ffi::c_schar;

#[unsafe(no_mangle)]
pub extern "C" fn test (a: i32, b: c_schar) -> bool {
    true
}