use crate::data_c::player_info_c::PlayerInfoC;

#[repr(C)]
pub struct PlayerList {
    pub(crate) players: *mut PlayerInfoC,
    pub(crate) len: usize,
    pub(crate) capacity: usize,
}