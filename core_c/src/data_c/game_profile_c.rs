use crate::{str_c_to_rs, str_rs_to_c};
use nogamepads_lib_rs::pad_data::game_profile::game_profile::GameProfile;
use std::ffi::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(unused_imports)]
pub struct GameProfileC {
    game_name: *const c_char,
    game_description: *const c_char,
    organization: *const c_char,
    version: *const c_char,
    website: *const c_char,
    email: *const c_char
}

impl From<GameProfileC> for GameProfile {
    fn from(value: GameProfileC) -> Self {
        let unknown = "".to_string();
        GameProfile {
            game_name: str_c_to_rs(value.game_name).unwrap_or(unknown.clone()),
            game_description: str_c_to_rs(value.game_description).unwrap_or(unknown.clone()),
            organization: str_c_to_rs(value.organization).unwrap_or(unknown.clone()),
            version: str_c_to_rs(value.version).unwrap_or(unknown.clone()),
            website: str_c_to_rs(value.website).unwrap_or(unknown.clone()),
            email: str_c_to_rs(value.email).unwrap_or(unknown.clone())
        }
    }
}

impl From<GameProfile> for GameProfileC {
    fn from(value: GameProfile) -> Self {
        GameProfileC {
            game_name: str_rs_to_c(value.game_name).1,
            game_description: str_rs_to_c(value.game_description).1,
            organization: str_rs_to_c(value.organization).1,
            version: str_rs_to_c(value.version).1,
            website: str_rs_to_c(value.website).1,
            email: str_rs_to_c(value.email).1
        }
    }
}

impl From<&GameProfileC> for &GameProfile {
    fn from(value: &GameProfileC) -> Self {
        value.into()
    }
}

impl From<&GameProfile> for &GameProfileC {
    fn from(value: &GameProfile) -> Self {
        value.into()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn init_game_profile() -> GameProfileC {
    GameProfile::default().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_name(game_profile: GameProfileC, value: *const c_char) -> GameProfileC {
    let value = str_c_to_rs(value).unwrap_or_default();
    let mut raw = GameProfile::from(game_profile);
    raw.game_name(value.as_str()).to_owned().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_description(game_profile: GameProfileC, value: *const c_char) -> GameProfileC {
    let value = str_c_to_rs(value).unwrap_or_default();
    let mut raw = GameProfile::from(game_profile);
    raw.game_description(value.as_str()).to_owned().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_organization(game_profile: GameProfileC, value: *const c_char) -> GameProfileC {
    let value = str_c_to_rs(value).unwrap_or_default();
    let mut raw = GameProfile::from(game_profile);
    raw.organization(value.as_str()).to_owned().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_version(game_profile: GameProfileC, value: *const c_char) -> GameProfileC {
    let value = str_c_to_rs(value).unwrap_or_default();
    let mut raw = GameProfile::from(game_profile);
    raw.version(value.as_str()).to_owned().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_website(game_profile: GameProfileC, value: *const c_char) -> GameProfileC {
    let value = str_c_to_rs(value).unwrap_or_default();
    let mut raw = GameProfile::from(game_profile);
    raw.website(value.as_str()).to_owned().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_email(game_profile: GameProfileC, value: *const c_char) -> GameProfileC {
    let value = str_c_to_rs(value).unwrap_or_default();
    let mut raw = GameProfile::from(game_profile);
    raw.email(value.as_str()).to_owned().into()
}