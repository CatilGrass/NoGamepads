use std::ffi::c_char;
use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
use crate::{box_into_raw, c_char_to_string_safe};

#[unsafe(no_mangle)]
pub extern "C" fn init_player_info() -> *mut PlayerInfo {
    box_into_raw!(PlayerInfo::new())
}

#[unsafe(no_mangle)]
pub extern "C" fn set_player_info_account(
    info: &mut PlayerInfo,
    id: *const c_char,
    password: *const c_char) {
    let id = c_char_to_string_safe(id).unwrap_or("".to_string());
    let password = c_char_to_string_safe(password).unwrap_or("".to_string());
    info.setup_account_info(id.as_str(), password.as_str());
}

#[unsafe(no_mangle)]
pub extern "C" fn set_player_info_color(
    info: &mut PlayerInfo,
    h: i32, s: f64, v: f64) {
    info.set_customize_color_hsv(h, s, v);
}

#[unsafe(no_mangle)]
pub extern "C" fn set_player_info_nickname(
    info: &mut PlayerInfo,
    nickname: *const c_char) {
    let nickname = c_char_to_string_safe(nickname).unwrap_or("".to_string());
    info.set_nickname(nickname.as_str());
}