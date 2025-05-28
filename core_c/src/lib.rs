use std::ffi::{c_char, CStr, CString};

pub mod data_c;
pub mod service_c;

pub fn string_to_c_char_safe(s: String) -> (CString, *const c_char) {
    let c_string = CString::new(s).expect("");
    let ptr = c_string.as_ptr();
    (c_string, ptr)
}

pub fn c_char_to_string_safe(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    unsafe {
        CStr::from_ptr(ptr)
            .to_str()
            .ok()
            .map(|s| s.to_owned())
    }
}

#[macro_export]
macro_rules! box_into_raw {
    ($expr:expr) => {
        $crate::box_into_raw!(@inner $expr)
    };

    ($expr:expr => $t:ty) => {
        $crate::box_into_raw!(@inner ($expr) as $t)
    };

    (@inner $expr:expr) => {
        ::std::boxed::Box::into_raw(::std::boxed::Box::new($expr))
    };

    (@inner ($expr:expr) as $t:ty) => {
        ::std::boxed::Box::into_raw(::std::boxed::Box::new($expr) as ::std::boxed::Box<$t>)
    };
}