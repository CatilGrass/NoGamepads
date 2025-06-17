use nogamepads_core::service::service_types::ServiceType;

#[repr(C)]
pub enum FfiServiceType {
    Unknown,
    TCPConnection,
    BlueTooth,
    USB,
}

impl From<&ServiceType> for FfiServiceType {
    fn from(value: &ServiceType) -> Self {
        match value {
            ServiceType::TCPConnection => { FfiServiceType::TCPConnection }
            ServiceType::BlueTooth => { FfiServiceType::BlueTooth }
            ServiceType::USB => { FfiServiceType::USB }
        }
    }
}

impl From<&FfiServiceType> for ServiceType {
    fn from(value: &FfiServiceType) -> Self {
        match value {
            FfiServiceType::TCPConnection => { ServiceType::TCPConnection }
            FfiServiceType::BlueTooth => { ServiceType::BlueTooth }
            FfiServiceType::USB => { ServiceType::USB }
            _ => {
                ServiceType::default()
            }
        }
    }
}

/// Free service type tag
#[unsafe(no_mangle)]
pub extern "C" fn free_ffi_service_type(ptr: *mut FfiServiceType) {
    if !ptr.is_null() {
        unsafe { let _ = Box::from_raw(ptr); }
    }
}