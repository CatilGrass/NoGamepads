use std::ffi::{c_char, c_double};
use crate::converter::string_converter::{str_c_to_rs, str_rs_to_c};
use crate::data::ngpd_game_info::FfiGameInfo;
use crate::data::ngpd_player::FfiPlayer;
use nogamepads_core::data::message::message_enums::{ConnectionMessage, ConnectionResponseMessage, ControlMessage, ExitReason, GameMessage, JoinFailedMessage};
use std::mem::ManuallyDrop;
use std::ops::Deref;

#[repr(C)]
pub struct FfiControlMessage {
    pub tag: FfiControlMessageTag,
    pub data: FfiControlMessageUnion
}

#[repr(C)]
pub enum FfiControlMessageTag {
    CtrlMsg,
    CtrlPressed,
    CtrlReleased,
    CtrlAxis,
    CtrlDir,
    CtrlExit,
    CtrlError,
    CtrlEnd
}

#[repr(C)]
pub union FfiControlMessageUnion {
    pub none: (),
    pub message: *mut c_char,
    pub key: u8,
    pub key_and_axis: ManuallyDrop<FfiKeyAndAxis>,
    pub key_and_direction: ManuallyDrop<FfiKeyAndDirection>,
}

#[repr(C)]
pub struct FfiKeyAndAxis {
    pub key: u8,
    pub axis: c_double
}

#[repr(C)]
pub struct FfiKeyAndDirection {
    pub key: u8,
    pub x: c_double,
    pub y: c_double
}

#[repr(C)]
pub struct FfiGameMessage {
    pub tag: FfiGameMessageTag,
    pub data: FfiGameMessageUnion
}

#[repr(C)]
pub enum FfiGameMessageTag {
    GameEventTrigger,
    GameMsg,
    GameLetExit,
    GameError,
    GameEnd
}

#[repr(C)]
pub union FfiGameMessageUnion {
    pub none: (),
    pub key: u8,
    pub message: *mut c_char,
    pub exit_reason: ManuallyDrop<FfiExitReason>,
}

#[repr(C)]
pub enum FfiExitReason {
    ExitReason,
    GameOverReason,
    ServerClosedReason,
    YouAreKickedReason,
    YouAreBannedReason,
    ErrorReason
}

#[repr(C)]
pub struct FfiConnectionMessage {
    pub tag: FfiConnectionMessageTag,
    pub data: FfiConnectionMessageUnion
}

#[repr(C)]
pub enum FfiConnectionMessageTag {
    ConnectionJoin,
    ConnectionRequestGameInfos,
    ConnectionRequestLayoutConfigure,
    ConnectionRequestSkinPackage,
    ConnectionReady,
    ConnectionError
}

#[repr(C)]
pub union FfiConnectionMessageUnion {
    pub none: (),
    pub player: ManuallyDrop<FfiPlayer>
}

#[repr(C)]
pub struct FfiConnectionResponseMessage {
    pub tag: FfiConnectionResponseMessageTag,
    pub data: FfiConnectionResponseMessageUnion
}

#[repr(C)]
pub enum FfiConnectionResponseMessageTag {
    GameInfosResponse,
    DenyResponse,
    FailResponse,
    OkResponse,
    WelcomeResponse,
    ErrorResponse
}

#[repr(C)]
pub union FfiConnectionResponseMessageUnion {
    pub none: (),
    pub game_info: ManuallyDrop<FfiGameInfo>,
    pub failed_message: ManuallyDrop<FfiJoinFailedMessage>
}

#[repr(C)]
pub enum FfiJoinFailedMessage {
    ContainIdenticalPlayer, PlayerBanned, GameLocked, UnknownError
}

impl From<ControlMessage> for FfiControlMessage {
    fn from(value: ControlMessage) -> Self {
        match value {
            ControlMessage::Msg(msg) => unsafe {
                FfiControlMessage {
                    tag: FfiControlMessageTag::CtrlMsg,
                    data: FfiControlMessageUnion {
                        message: str_rs_to_c(msg),
                    }
                }
            }
            ControlMessage::Pressed(key) => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::CtrlPressed,
                    data: FfiControlMessageUnion {
                        key: key.into()
                    }
                }
            }
            ControlMessage::Released(key) => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::CtrlReleased,
                    data: FfiControlMessageUnion {
                        key: key.into()
                    }
                }
            }
            ControlMessage::Axis(key, axis) => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::CtrlAxis,
                    data: FfiControlMessageUnion {
                        key_and_axis: ManuallyDrop::new(FfiKeyAndAxis {
                            key: key.into(),
                            axis: axis.into()
                        })
                    }
                }
            }
            ControlMessage::Dir(key, (x, y)) => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::CtrlAxis,
                    data: FfiControlMessageUnion {
                        key_and_direction: ManuallyDrop::new(FfiKeyAndDirection {
                            key: key.into(),
                            x, y
                        })
                    }
                }
            }
            ControlMessage::Exit => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::CtrlAxis,
                    data: FfiControlMessageUnion { none: () }
                }
            }
            ControlMessage::Err => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::CtrlAxis,
                    data: FfiControlMessageUnion { none: () }
                }
            }
            ControlMessage::End => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::CtrlAxis,
                    data: FfiControlMessageUnion { none: () }
                }
            }
        }
    }
}

impl From<FfiControlMessage> for ControlMessage {
    fn from(value: FfiControlMessage) -> Self {
        match value.tag {
            FfiControlMessageTag::CtrlMsg => unsafe {
                ControlMessage::Msg(str_c_to_rs(value.data.message))
            }
            FfiControlMessageTag::CtrlPressed => unsafe {
                ControlMessage::Pressed(value.data.key.into())
            }
            FfiControlMessageTag::CtrlReleased => unsafe {
                ControlMessage::Released(value.data.key.into())
            }
            FfiControlMessageTag::CtrlAxis => {
                let value = unsafe { value.data.key_and_axis };
                ControlMessage::Axis(value.key.into(), value.axis.into())
            }
            FfiControlMessageTag::CtrlDir => {
                let value = unsafe { value.data.key_and_direction };
                ControlMessage::Dir(value.key.into(), (value.x.into(), value.y.into()))
            }
            FfiControlMessageTag::CtrlExit => {
                ControlMessage::Exit
            }
            FfiControlMessageTag::CtrlError => {
                ControlMessage::Err
            }
            FfiControlMessageTag::CtrlEnd => {
                ControlMessage::End
            }
        }
    }
}

impl From<GameMessage> for FfiGameMessage {
    fn from(value: GameMessage) -> Self {
        match value {
            GameMessage::EventTrigger(key) => {
                FfiGameMessage {
                    tag: FfiGameMessageTag::GameEventTrigger,
                    data: FfiGameMessageUnion {
                        key: key.into()
                    }
                }
            }
            GameMessage::Msg(msg) => unsafe {
                FfiGameMessage {
                    tag: FfiGameMessageTag::GameMsg,
                    data: FfiGameMessageUnion {
                        message: str_rs_to_c(msg)
                    }
                }
            }
            GameMessage::LetExit(reason) => {
                FfiGameMessage {
                    tag: FfiGameMessageTag::GameLetExit,
                    data: FfiGameMessageUnion {
                        exit_reason: ManuallyDrop::new(FfiExitReason::from(&reason))
                    }
                }
            }
            GameMessage::Err => {
                FfiGameMessage {
                    tag: FfiGameMessageTag::GameError,
                    data: FfiGameMessageUnion { none: () }
                }
            }
            GameMessage::End => {
                FfiGameMessage {
                    tag: FfiGameMessageTag::GameEnd,
                    data: FfiGameMessageUnion { none: () }
                }
            }
        }
    }
}

impl From<FfiGameMessage> for GameMessage {
    fn from(value: FfiGameMessage) -> Self {
        match value.tag {
            FfiGameMessageTag::GameEventTrigger => unsafe {
                GameMessage::EventTrigger(value.data.key.into())
            }
            FfiGameMessageTag::GameMsg => unsafe {
                GameMessage::Msg(str_c_to_rs(value.data.message))
            }
            FfiGameMessageTag::GameLetExit => unsafe {
                GameMessage::LetExit(value.data.exit_reason.deref().into())
            }
            FfiGameMessageTag::GameError => {
                GameMessage::Err
            }
            FfiGameMessageTag::GameEnd => {
                GameMessage::End
            }
        }
    }
}

impl From<&ExitReason> for FfiExitReason {
    fn from(value: &ExitReason) -> Self {
        match value {
            ExitReason::Exit => { FfiExitReason::ExitReason }
            ExitReason::GameOver => { FfiExitReason::GameOverReason }
            ExitReason::ServerClosed => { FfiExitReason::ServerClosedReason }
            ExitReason::YouAreKicked => { FfiExitReason::YouAreKickedReason }
            ExitReason::YouAreBanned => { FfiExitReason::YouAreBannedReason }
            ExitReason::Err => { FfiExitReason::ErrorReason }
        }
    }
}

impl From<&FfiExitReason> for ExitReason {
    fn from(value: &FfiExitReason) -> Self {
        match value {
            FfiExitReason::ExitReason => { ExitReason::Exit }
            FfiExitReason::GameOverReason => { ExitReason::GameOver }
            FfiExitReason::ServerClosedReason => { ExitReason::ServerClosed }
            FfiExitReason::YouAreKickedReason => { ExitReason::YouAreKicked }
            FfiExitReason::YouAreBannedReason => { ExitReason::YouAreBanned }
            FfiExitReason::ErrorReason => { ExitReason::Err }
        }
    }
}

impl From<ConnectionMessage> for FfiConnectionMessage {
    fn from(value: ConnectionMessage) -> Self {
        match value {
            ConnectionMessage::Join(player) => {
                let c_player : FfiPlayer = (&player).into();
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::ConnectionJoin,
                    data: FfiConnectionMessageUnion {
                        player: ManuallyDrop::new(c_player),
                    }
                }
            }
            ConnectionMessage::RequestGameInfos => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::ConnectionRequestGameInfos,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
            ConnectionMessage::RequestLayoutConfigure => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::ConnectionRequestLayoutConfigure,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
            ConnectionMessage::RequestSkinPackage => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::ConnectionRequestSkinPackage,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
            ConnectionMessage::Ready => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::ConnectionReady,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
            ConnectionMessage::Err => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::ConnectionError,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
        }
    }
}

impl From<FfiConnectionMessage> for ConnectionMessage {
    fn from(value: FfiConnectionMessage) -> Self {
        match value.tag {
            FfiConnectionMessageTag::ConnectionJoin => unsafe {
                ConnectionMessage::Join(value.data.player.deref().try_into().unwrap_or_default())
            }
            FfiConnectionMessageTag::ConnectionRequestGameInfos => { ConnectionMessage::RequestGameInfos }
            FfiConnectionMessageTag::ConnectionRequestLayoutConfigure => { ConnectionMessage::RequestLayoutConfigure }
            FfiConnectionMessageTag::ConnectionRequestSkinPackage => { ConnectionMessage::RequestSkinPackage }
            FfiConnectionMessageTag::ConnectionReady => { ConnectionMessage::Ready }
            FfiConnectionMessageTag::ConnectionError => { ConnectionMessage::Err }
        }
    }
}

impl From<ConnectionResponseMessage> for FfiConnectionResponseMessage {
    fn from(value: ConnectionResponseMessage) -> Self {
        match value {
            ConnectionResponseMessage::GameInfos(info) => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::GameInfosResponse,
                    data: FfiConnectionResponseMessageUnion {
                        game_info: ManuallyDrop::new((&info).into())
                    }
                }
            }
            ConnectionResponseMessage::Deny(fail) => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::DenyResponse,
                    data: FfiConnectionResponseMessageUnion {
                        failed_message: ManuallyDrop::new((&fail).into())
                    }
                }
            }
            ConnectionResponseMessage::Fail(fail) => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::FailResponse,
                    data: FfiConnectionResponseMessageUnion {
                        failed_message: ManuallyDrop::new((&fail).into())
                    }
                }
            }
            ConnectionResponseMessage::Ok => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::OkResponse,
                    data: FfiConnectionResponseMessageUnion { none: () }
                }
            }
            ConnectionResponseMessage::Welcome => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::WelcomeResponse,
                    data: FfiConnectionResponseMessageUnion { none: () }
                }
            }
            ConnectionResponseMessage::Err => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::ErrorResponse,
                    data: FfiConnectionResponseMessageUnion { none: () }
                }
            }
        }
    }
}

impl From<FfiConnectionResponseMessage> for ConnectionResponseMessage {
    fn from(value: FfiConnectionResponseMessage) -> Self {
        match value.tag {
            FfiConnectionResponseMessageTag::GameInfosResponse => unsafe {
                ConnectionResponseMessage::GameInfos(value.data.game_info.deref().try_into().unwrap_or_default())
            }
            FfiConnectionResponseMessageTag::DenyResponse => unsafe {
                ConnectionResponseMessage::Deny(value.data.failed_message.deref().try_into().unwrap_or_default())
            }
            FfiConnectionResponseMessageTag::FailResponse => unsafe {
                ConnectionResponseMessage::Fail(value.data.failed_message.deref().try_into().unwrap_or_default())
            }
            FfiConnectionResponseMessageTag::OkResponse => {
                ConnectionResponseMessage::Ok
            }
            FfiConnectionResponseMessageTag::WelcomeResponse => {
                ConnectionResponseMessage::Welcome
            }
            FfiConnectionResponseMessageTag::ErrorResponse => {
                ConnectionResponseMessage::Err
            }
        }
    }
}

impl From<&JoinFailedMessage> for FfiJoinFailedMessage {
    fn from(value: &JoinFailedMessage) -> Self {
        match value {
            JoinFailedMessage::ContainIdenticalPlayer => { FfiJoinFailedMessage::ContainIdenticalPlayer }
            JoinFailedMessage::PlayerBanned => { FfiJoinFailedMessage::PlayerBanned }
            JoinFailedMessage::GameLocked => { FfiJoinFailedMessage::GameLocked }
            JoinFailedMessage::UnknownError => { FfiJoinFailedMessage::UnknownError }
        }
    }
}

impl From<&FfiJoinFailedMessage> for JoinFailedMessage {
    fn from(value: &FfiJoinFailedMessage) -> Self {
        match value {
            FfiJoinFailedMessage::ContainIdenticalPlayer => { JoinFailedMessage::ContainIdenticalPlayer }
            FfiJoinFailedMessage::PlayerBanned => { JoinFailedMessage::PlayerBanned }
            FfiJoinFailedMessage::GameLocked => { JoinFailedMessage::GameLocked }
            FfiJoinFailedMessage::UnknownError => { JoinFailedMessage::UnknownError }
        }
    }
}

/// Free ControlMessage
#[unsafe(no_mangle)]
pub extern "C" fn free_control_message(msg: FfiControlMessage) {
    let _ = msg;
}

/// Free GameMessage
#[unsafe(no_mangle)]
pub extern "C" fn free_game_message(msg: FfiGameMessage) {
    let _ = msg;
}

/// Free ExitReason
#[unsafe(no_mangle)]
pub extern "C" fn free_exit_reason(msg: FfiExitReason) {
    let _ = msg;
}

/// Free ConnectionMessage
#[unsafe(no_mangle)]
pub extern "C" fn free_connection_message(msg: FfiConnectionMessage) {
    let _ = msg;
}

/// Free ConnectionResponseMessage
#[unsafe(no_mangle)]
pub extern "C" fn free_connection_response_message(msg: FfiConnectionResponseMessage) {
    let _ = msg;
}

/// Free JoinFailedMessage
#[unsafe(no_mangle)]
pub extern "C" fn free_join_failed_message(msg: FfiJoinFailedMessage) {
    let _ = msg;
}