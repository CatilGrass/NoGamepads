use crate::data::player::structs::Player;
use bincode::{Decode, Encode};
use crate::data::game::types::GameInfo;

/// Control messages.
/// Messages sent from controller to game pad_client after establishing persistent connection
#[derive(Default, Encode, Decode, PartialEq, Debug, Clone)]
pub enum ControlMessage {
    /// Plain message containing a string
    /// The message will be handed over to the game for its own processing
    Msg(String),

    /// Press event
    /// Indicates that a button has been pressed
    Pressed(u8),

    /// Release event
    /// Indicates that a button has been released
    Released(u8),

    /// Axis input
    /// Indicates that the value of an axis has been changed
    Axis(u8, f64),

    /// Directional input
    /// Indicates that the value of a direction has been changed
    Dir(u8, (f64, f64)),

    /// Exit command
    /// Sends a disconnect request to the pad_server
    Exit,

    #[default]
    /// Error state
    Err,

    /// Indicates the termination message, which is the final message in a long-lived connection.
    End
}

/// Game messages.
/// Messages sent from game pad_client to controller after establishing persistent connection
#[derive(Default, Encode, Decode, PartialEq, Debug, Clone)]
pub enum GameMessage {
    /// Event trigger
    /// Sends an event to the controller; if skins are enabled, this will trigger corresponding animations, sounds, vibrations, etc.
    EventTrigger(u8),

    /// Plain message containing a string
    /// The message will be handed over to the controller for its own processing
    Msg(String),

    /// Disconnect request
    /// Notifies the pad_client that the connection will be terminated
    LetExit(ExitReason),

    /// Error state
    #[default]
    Err,

    /// Indicates the termination message, which is the final message in a long-lived connection.
    End
}

/// Exit reasons.
/// Reason provided when requesting disconnection
#[derive(Default, Encode, Decode, PartialEq, Debug, Clone)]
pub enum ExitReason {
    /// Normal exit
    /// No specific reason, simply requesting to disconnect
    Exit,

    /// Game has ended
    GameOver,

    /// Server shutdown (normal)
    ServerClosed,

    /// Kicked by pad_server
    YouAreKicked,

    /// Account banned
    YouAreBanned,

    /// Error state
    #[default]
    Err
}

/// Connection messages.
/// Messages sent by pad_client when requesting pad_server connection
#[derive(Default, Encode, Decode, PartialEq, Debug, Clone)]
pub enum ConnectionMessage {
    /// Requests to join the game
    Join(Player),

    /// Request for game information
    RequestGameInfos,

    /// Request for game layout configuration file
    RequestLayoutConfigure,

    /// Request to download game skin assets
    RequestSkinPackage,

    /// Ready state to establish persistent connection
    Ready,

    /// Error state
    #[default]
    Err
}

/// Connection Response.
/// Messages from pad_server responding to pad_client connection requests
#[derive(Default, Encode, Decode, PartialEq, Debug, Clone)]
pub enum ConnectionResponseMessage {
    /// Game information data
    GameInfos(GameInfo),

    /// Rejection with reason
    Deny(JoinFailedMessage),

    /// Failure with reason
    Fail(JoinFailedMessage),

    /// Approval confirmation
    Ok,

    /// Welcome acknowledgment
    Welcome,

    /// Error state
    #[default]
    Err
}

/// Game Join Failure Information.
/// Reason provided when pad_client fails to join
#[derive(Default, Encode, Decode, PartialEq, Debug, Clone)]
pub enum JoinFailedMessage {
    /// Game already contains identical player
    ContainIdenticalPlayer,

    /// Player is banned
    PlayerBanned,

    /// Game is locked, no further joins allowed
    GameLocked,

    /// Unknown error
    #[default]
    UnknownError
}