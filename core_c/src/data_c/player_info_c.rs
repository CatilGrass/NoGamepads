use crate::{str_c_to_rs, str_rs_to_c};
use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::{PlayerAccountInfo, PlayerCustomizeInfo, PlayerInfo};
use std::ffi::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(unused_imports)]
pub struct PlayerInfoC {
    account_id: *const c_char,
    account_hash: *const c_char,

    customize_nickname: *const c_char,
    customize_color_hue: i32,
    customize_color_saturation: f64,
    customize_color_value: f64,
}

impl From<PlayerInfoC> for PlayerInfo {
    fn from(value: PlayerInfoC) -> Self {
        let unknown = "unknown".to_string();
        PlayerInfo {
            account : PlayerAccountInfo {
                id: str_c_to_rs(value.account_id).unwrap_or(unknown.clone()),
                player_hash: str_c_to_rs(value.account_hash).unwrap_or(unknown.clone()),
            },
            customize: PlayerCustomizeInfo {
                nickname: str_c_to_rs(value.customize_nickname).unwrap_or(unknown.clone()),
                color_hue: value.customize_color_hue,
                color_saturation: value.customize_color_saturation,
                color_value: value.customize_color_value,
            }
        }
    }
}

impl From<PlayerInfo> for PlayerInfoC {
    fn from(value: PlayerInfo) -> Self {
        PlayerInfoC {
            account_id: str_rs_to_c(value.account.id).1,
            account_hash: str_rs_to_c(value.account.player_hash).1,
            customize_nickname: str_rs_to_c(value.customize.nickname).1,
            customize_color_hue: value.customize.color_hue,
            customize_color_saturation: value.customize.color_saturation,
            customize_color_value: value.customize.color_value,
        }
    }
}

impl From<&PlayerInfoC> for &PlayerInfo {
    fn from(value: &PlayerInfoC) -> Self {
        value.into()
    }
}

impl From<&PlayerInfo> for &PlayerInfoC {
    fn from(value: &PlayerInfo) -> Self {
        value.into()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn init_player_info() -> PlayerInfoC {
    PlayerInfo::new().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_player_info_account(
    info: PlayerInfoC, id: *const c_char, password: *const c_char) -> PlayerInfoC {

    let id = str_c_to_rs(id).unwrap_or("".to_string());
    let password = str_c_to_rs(password).unwrap_or("".to_string());

    let mut raw_info = PlayerInfo::from(info);
    raw_info.setup_account_info(id.as_str(), password.as_str());
    raw_info.into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_player_info_color(
    info: PlayerInfoC, h: i32, s: f64, v: f64) -> PlayerInfoC {

    let mut raw_info = PlayerInfo::from(info);
    raw_info.set_customize_color_hsv(h, s, v);
    raw_info.into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_player_info_nickname(
    info: PlayerInfoC, nickname: *const c_char) -> PlayerInfoC {
    let nickname = str_c_to_rs(nickname).unwrap_or("".to_string());

    let mut raw_info = PlayerInfo::from(info);
    raw_info.set_nickname(nickname.as_str());
    raw_info.into()
}