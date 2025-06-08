use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data::game::types::GameInfo;
use crate::data::player::structs::Account;

/// Game pad_client data
/// Describes the basic information of the game pad_client
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct GameData {
    pub info: GameInfo,
    pub control: GameControlData,
    pub archive: GameRuntimeDataArchive
}

/// Game control information
/// Describes the buttons, axes, and directions that can be controlled.
#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct GameControlData {
    pub direction_keys : HashMap<u8, String>,
    pub axis_keys : HashMap<u8, String>,
    pub button_keys : HashMap<u8, String>,
}

/// Archive of game runtime data
/// The game pad_client can convert data into this structure for persistence.
#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct GameRuntimeDataArchive {
    pub banned: Vec<Account>
}