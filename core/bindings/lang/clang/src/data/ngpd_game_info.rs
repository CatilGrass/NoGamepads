use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct KeyValuePair {
    key: *mut c_char,
    value: *mut c_char,
}

#[repr(C)]
pub struct FfiGameInfo {
    data: *mut KeyValuePair,
    len: usize,
    cap: usize,
}

impl From<&HashMap<String, String>> for FfiGameInfo {
    fn from(map: &HashMap<String, String>) -> Self {
        let mut kv_pairs: Vec<KeyValuePair> = map
            .into_iter()
            .map(|(k, v)| {
                KeyValuePair {
                    key: CString::new(k.clone()).unwrap().into_raw(),
                    value: CString::new(v.clone()).unwrap().into_raw(),
                }
            })
            .collect();

        let cap = kv_pairs.capacity();
        let len = kv_pairs.len();
        let data_ptr = kv_pairs.as_mut_ptr();

        std::mem::forget(kv_pairs);

        FfiGameInfo {
            data: data_ptr,
            len,
            cap,
        }
    }
}

impl TryFrom<&FfiGameInfo> for HashMap<String, String> {
    type Error = String;

    fn try_from(ffi_info: &FfiGameInfo) -> Result<Self, Self::Error> {
        if ffi_info.len == 0 {
            return Ok(HashMap::new());
        }

        if ffi_info.data.is_null() {
            return Err("FfiGameInfo data pointer is null".to_string());
        }

        let kv_slice = unsafe {
            std::slice::from_raw_parts(ffi_info.data, ffi_info.len)
        };

        let mut map = HashMap::with_capacity(ffi_info.len);

        for (i, pair) in kv_slice.iter().enumerate() {
            if pair.key.is_null() {
                return Err(format!("Key pointer is null at index {}", i));
            }

            if pair.value.is_null() {
                return Err(format!("Value pointer is null at index {}", i));
            }

            let key_str = unsafe {
                CStr::from_ptr(pair.key)
                    .to_str()
                    .map_err(|_| "Invalid UTF-8 in key")?
            };

            let value_str = unsafe {
                CStr::from_ptr(pair.value)
                    .to_str()
                    .map_err(|_| "Invalid UTF-8 in value")?
            };

            map.insert(key_str.to_owned(), value_str.to_owned());
        }

        Ok(map)
    }
}

impl From<&FfiGameInfo> for Option<HashMap<String, String>> {
    fn from(ffi_info: &FfiGameInfo) -> Self {
        match HashMap::try_from(ffi_info) {
            Ok(map) => Some(map),
            Err(_) => None,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_game_info(map: FfiGameInfo) {
    let kv_pairs = unsafe { Vec::from_raw_parts(map.data, map.len, map.cap) };

    for pair in kv_pairs {
        unsafe {
            let _ = CString::from_raw(pair.key);
            let _ = CString::from_raw(pair.value);
        }
    }
}