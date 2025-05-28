use std::ffi::c_char;
use crate::{box_into_raw, c_char_to_string_safe};
use nogamepads_lib_rs::pad_data::game_profile::game_profile::GameProfile;

#[unsafe(no_mangle)]
pub extern "C" fn init_game_profile() -> *mut GameProfile {
    box_into_raw!(GameProfile::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_name(game_profile: &mut GameProfile, value: *const c_char) {
    let value = c_char_to_string_safe(value).unwrap_or_default();
    game_profile.game_name(value.as_str());
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_description(game_profile: &mut GameProfile, value: *const c_char) {
    let value = c_char_to_string_safe(value).unwrap_or_default();
    game_profile.game_description(value.as_str());
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_organization(game_profile: &mut GameProfile, value: *const c_char) {
    let value = c_char_to_string_safe(value).unwrap_or_default();
    game_profile.organization(value.as_str());
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_version(game_profile: &mut GameProfile, value: *const c_char) {
    let value = c_char_to_string_safe(value).unwrap_or_default();
    game_profile.version(value.as_str());
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_website(game_profile: &mut GameProfile, value: *const c_char) {
    let value = c_char_to_string_safe(value).unwrap_or_default();
    game_profile.website(value.as_str());
}

#[unsafe(no_mangle)]
pub extern "C" fn set_game_profile_email(game_profile: &mut GameProfile, value: *const c_char) {
    let value = c_char_to_string_safe(value).unwrap_or_default();
    game_profile.email(value.as_str());
}