use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::hash::{Hash};

/// Player information
/// Describes a player's specific details, which are frequently exchanged between the controller and game pad_client.
#[derive(Default, Clone, Encode, Decode, Serialize, Deserialize, Debug)]
pub struct Player {

    /// Account information
    pub account: Account,

    /// Custom information (Optional)
    pub customize: Option<Customize>
}

/// Account information
/// Essential data for verifying player uniqueness, including the player's hash value and account ID.
#[derive(Default, Clone, Encode, Decode, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct Account {

    /// Player name stored in data, allowing only lowercase letters and underscores
    pub id: String,

    /// Player hash value proving player uniqueness
    pub player_hash: String
}

/// Custom information
/// Describes personalized player details displayed in-game, such as name, color, or other customizations.
#[derive(Default, Clone, Encode, Decode, Serialize, Deserialize, PartialEq, Debug)]
pub struct Customize {

    /// Player name displayed in the game
    pub nickname: String,

    /// HSV Color - Hue (Range: 0 - 360)
    pub color_hue: i32,

    /// HSV Color - Saturation (Range: 0 - 1)
    pub color_saturation: f64,

    /// HSV Color - Value (Range: 0 - 1)
    pub color_value: f64
}