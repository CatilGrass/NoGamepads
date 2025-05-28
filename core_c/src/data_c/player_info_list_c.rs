use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;

#[repr(C)]
pub struct PlayerList {
    pub(crate) players: *mut PlayerInfo,
    pub(crate) len: usize,
    pub(crate) capacity: usize,
}