use crate::converter::string_converter::{str_c_to_rs, str_rs_to_c};
use crate::data::ngpd_game_info::FfiGameInfo;
use crate::data::ngpd_player::FfiPlayer;
use nogamepads_core::data::message::message_enums::{ConnectionMessage, ConnectionResponseMessage, ControlMessage, ExitReason, GameMessage, JoinFailedMessage};
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::os::raw::{c_char, c_double};

#[repr(C)]
pub struct FfiControlMessage {
    pub tag: FfiControlMessageTag,
    pub data: FfiControlMessageUnion
}

#[repr(C)]
pub enum FfiControlMessageTag {
    Msg, Pressed, Released, Axis, Dir, Exit, Err, End
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
    EventTrigger, Msg, LetExit, Err, End
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
    Exit, GameOver, ServerClosed, YouAreKicked, YouAreBanned, Err
}

#[repr(C)]
pub struct FfiConnectionMessage {
    pub tag: FfiConnectionMessageTag,
    pub data: FfiConnectionMessageUnion
}

#[repr(C)]
pub enum FfiConnectionMessageTag {
    Join, RequestGameInfos, RequestLayoutConfigure, RequestSkinPackage, Ready, Err
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
    GameInfos, Deny, Fail, Ok, Welcome, Err
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
                    tag: FfiControlMessageTag::Msg,
                    data: FfiControlMessageUnion {
                        message: str_rs_to_c(msg),
                    }
                }
            }
            ControlMessage::Pressed(key) => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::Pressed,
                    data: FfiControlMessageUnion {
                        key: key.into()
                    }
                }
            }
            ControlMessage::Released(key) => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::Released,
                    data: FfiControlMessageUnion {
                        key: key.into()
                    }
                }
            }
            ControlMessage::Axis(key, axis) => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::Axis,
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
                    tag: FfiControlMessageTag::Axis,
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
                    tag: FfiControlMessageTag::Axis,
                    data: FfiControlMessageUnion { none: () }
                }
            }
            ControlMessage::Err => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::Axis,
                    data: FfiControlMessageUnion { none: () }
                }
            }
            ControlMessage::End => {
                FfiControlMessage {
                    tag: FfiControlMessageTag::Axis,
                    data: FfiControlMessageUnion { none: () }
                }
            }
        }
    }
}

impl From<FfiControlMessage> for ControlMessage {
    fn from(value: FfiControlMessage) -> Self {
        match value.tag {
            FfiControlMessageTag::Msg => unsafe {
                ControlMessage::Msg(str_c_to_rs(value.data.message))
            }
            FfiControlMessageTag::Pressed => unsafe {
                ControlMessage::Pressed(value.data.key.into())
            }
            FfiControlMessageTag::Released => unsafe {
                ControlMessage::Released(value.data.key.into())
            }
            FfiControlMessageTag::Axis => {
                let value = unsafe { value.data.key_and_axis };
                ControlMessage::Axis(value.key.into(), value.axis.into())
            }
            FfiControlMessageTag::Dir => {
                let value = unsafe { value.data.key_and_direction };
                ControlMessage::Dir(value.key.into(), (value.x.into(), value.y.into()))
            }
            FfiControlMessageTag::Exit => {
                ControlMessage::Exit
            }
            FfiControlMessageTag::Err => {
                ControlMessage::Err
            }
            FfiControlMessageTag::End => {
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
                    tag: FfiGameMessageTag::EventTrigger,
                    data: FfiGameMessageUnion {
                        key: key.into()
                    }
                }
            }
            GameMessage::Msg(msg) => unsafe {
                FfiGameMessage {
                    tag: FfiGameMessageTag::Msg,
                    data: FfiGameMessageUnion {
                        message: str_rs_to_c(msg)
                    }
                }
            }
            GameMessage::LetExit(reason) => {
                FfiGameMessage {
                    tag: FfiGameMessageTag::LetExit,
                    data: FfiGameMessageUnion {
                        exit_reason: ManuallyDrop::new(FfiExitReason::from(&reason))
                    }
                }
            }
            GameMessage::Err => {
                FfiGameMessage {
                    tag: FfiGameMessageTag::Err,
                    data: FfiGameMessageUnion { none: () }
                }
            }
            GameMessage::End => {
                FfiGameMessage {
                    tag: FfiGameMessageTag::End,
                    data: FfiGameMessageUnion { none: () }
                }
            }
        }
    }
}

impl From<FfiGameMessage> for GameMessage {
    fn from(value: FfiGameMessage) -> Self {
        match value.tag {
            FfiGameMessageTag::EventTrigger => unsafe {
                GameMessage::EventTrigger(value.data.key.into())
            }
            FfiGameMessageTag::Msg => unsafe {
                GameMessage::Msg(str_c_to_rs(value.data.message))
            }
            FfiGameMessageTag::LetExit => unsafe {
                GameMessage::LetExit(value.data.exit_reason.deref().into())
            }
            FfiGameMessageTag::Err => {
                GameMessage::Err
            }
            FfiGameMessageTag::End => {
                GameMessage::End
            }
        }
    }
}

impl From<&ExitReason> for FfiExitReason {
    fn from(value: &ExitReason) -> Self {
        match value {
            ExitReason::Exit => { FfiExitReason::Exit }
            ExitReason::GameOver => { FfiExitReason::GameOver }
            ExitReason::ServerClosed => { FfiExitReason::ServerClosed }
            ExitReason::YouAreKicked => { FfiExitReason::YouAreKicked }
            ExitReason::YouAreBanned => { FfiExitReason::YouAreBanned }
            ExitReason::Err => { FfiExitReason::Err }
        }
    }
}

impl From<&FfiExitReason> for ExitReason {
    fn from(value: &FfiExitReason) -> Self {
        match value {
            FfiExitReason::Exit => { ExitReason::Exit }
            FfiExitReason::GameOver => { ExitReason::GameOver }
            FfiExitReason::ServerClosed => { ExitReason::ServerClosed }
            FfiExitReason::YouAreKicked => { ExitReason::YouAreKicked }
            FfiExitReason::YouAreBanned => { ExitReason::YouAreBanned }
            FfiExitReason::Err => { ExitReason::Err }
        }
    }
}

impl From<ConnectionMessage> for FfiConnectionMessage {
    fn from(value: ConnectionMessage) -> Self {
        match value {
            ConnectionMessage::Join(player) => {
                let c_player : FfiPlayer = (&player).into();
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::Join,
                    data: FfiConnectionMessageUnion {
                        player: ManuallyDrop::new(c_player),
                    }
                }
            }
            ConnectionMessage::RequestGameInfos => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::RequestGameInfos,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
            ConnectionMessage::RequestLayoutConfigure => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::RequestLayoutConfigure,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
            ConnectionMessage::RequestSkinPackage => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::RequestSkinPackage,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
            ConnectionMessage::Ready => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::Ready,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
            ConnectionMessage::Err => {
                FfiConnectionMessage {
                    tag: FfiConnectionMessageTag::Err,
                    data: FfiConnectionMessageUnion { none: () }
                }
            }
        }
    }
}

impl From<FfiConnectionMessage> for ConnectionMessage {
    fn from(value: FfiConnectionMessage) -> Self {
        match value.tag {
            FfiConnectionMessageTag::Join => unsafe {
                ConnectionMessage::Join(value.data.player.deref().try_into().unwrap_or_default())
            }
            FfiConnectionMessageTag::RequestGameInfos => { ConnectionMessage::RequestGameInfos }
            FfiConnectionMessageTag::RequestLayoutConfigure => { ConnectionMessage::RequestLayoutConfigure }
            FfiConnectionMessageTag::RequestSkinPackage => { ConnectionMessage::RequestSkinPackage }
            FfiConnectionMessageTag::Ready => { ConnectionMessage::Ready }
            FfiConnectionMessageTag::Err => { ConnectionMessage::Err }
        }
    }
}

impl From<ConnectionResponseMessage> for FfiConnectionResponseMessage {
    fn from(value: ConnectionResponseMessage) -> Self {
        match value {
            ConnectionResponseMessage::GameInfos(info) => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::GameInfos,
                    data: FfiConnectionResponseMessageUnion {
                        game_info: ManuallyDrop::new((&info).into())
                    }
                }
            }
            ConnectionResponseMessage::Deny(fail) => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::Deny,
                    data: FfiConnectionResponseMessageUnion {
                        failed_message: ManuallyDrop::new((&fail).into())
                    }
                }
            }
            ConnectionResponseMessage::Fail(fail) => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::Fail,
                    data: FfiConnectionResponseMessageUnion {
                        failed_message: ManuallyDrop::new((&fail).into())
                    }
                }
            }
            ConnectionResponseMessage::Ok => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::Ok,
                    data: FfiConnectionResponseMessageUnion { none: () }
                }
            }
            ConnectionResponseMessage::Welcome => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::Welcome,
                    data: FfiConnectionResponseMessageUnion { none: () }
                }
            }
            ConnectionResponseMessage::Err => {
                FfiConnectionResponseMessage {
                    tag: FfiConnectionResponseMessageTag::Err,
                    data: FfiConnectionResponseMessageUnion { none: () }
                }
            }
        }
    }
}

impl From<FfiConnectionResponseMessage> for ConnectionResponseMessage {
    fn from(value: FfiConnectionResponseMessage) -> Self {
        match value.tag {
            FfiConnectionResponseMessageTag::GameInfos => unsafe {
                ConnectionResponseMessage::GameInfos(value.data.game_info.deref().try_into().unwrap_or_default())
            }
            FfiConnectionResponseMessageTag::Deny => unsafe {
                ConnectionResponseMessage::Deny(value.data.failed_message.deref().try_into().unwrap_or_default())
            }
            FfiConnectionResponseMessageTag::Fail => unsafe {
                ConnectionResponseMessage::Fail(value.data.failed_message.deref().try_into().unwrap_or_default())
            }
            FfiConnectionResponseMessageTag::Ok => {
                ConnectionResponseMessage::Ok
            }
            FfiConnectionResponseMessageTag::Welcome => {
                ConnectionResponseMessage::Welcome
            }
            FfiConnectionResponseMessageTag::Err => {
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

#[unsafe(no_mangle)]
pub extern "C" fn free_control_message(msg: FfiControlMessage) {
    let _ = msg;
}

#[unsafe(no_mangle)]
pub extern "C" fn free_game_message(msg: FfiGameMessage) {
    let _ = msg;
}

#[unsafe(no_mangle)]
pub extern "C" fn free_exit_reason(msg: FfiExitReason) {
    let _ = msg;
}

#[unsafe(no_mangle)]
pub extern "C" fn free_connection_message(msg: FfiConnectionMessage) {
    let _ = msg;
}

#[unsafe(no_mangle)]
pub extern "C" fn free_connection_response_message(msg: FfiConnectionResponseMessage) {
    let _ = msg;
}

#[unsafe(no_mangle)]
pub extern "C" fn free_join_failed_message(msg: FfiJoinFailedMessage) {
    let _ = msg;
}