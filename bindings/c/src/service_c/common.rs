use nogamepads_lib_rs::DEFAULT_PORT;

#[unsafe(no_mangle)]
pub extern "C" fn get_default_port() -> u16 { DEFAULT_PORT }