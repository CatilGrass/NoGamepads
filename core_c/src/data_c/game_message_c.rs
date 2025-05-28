use std::ffi::c_void;
use std::ptr::null_mut;
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::{GameMessage, LeaveReason};
use crate::data_c::common::KeyData;

#[repr(C)]
#[allow(unused_imports)]
pub struct GameMsgC {
    tag: GameMsgCTag,
    data: GameMsgCUnion
}


#[repr(C)]
#[allow(unused_imports)]
pub enum GameMsgCTag {
    SkinEventTrigger, DisableKey, EnableKey, Leave, Err
}

#[repr(C)]
#[allow(unused_imports)]
pub union GameMsgCUnion {
    nul: *mut c_void,
    key: KeyData,
    reason: LeaveReasonData
}

#[repr(C)]
#[allow(unused_imports)]
#[derive(Copy, Clone)]
pub enum LeaveReasonData {
    GameOver, ServerClosed, YouAreKicked, YouAreBanned
}

impl From<LeaveReasonData> for LeaveReason {
    fn from(value: LeaveReasonData) -> Self {
        match value {
            LeaveReasonData::GameOver => { LeaveReason::GameOver }
            LeaveReasonData::ServerClosed => { LeaveReason::ServerClosed }
            LeaveReasonData::YouAreKicked => { LeaveReason::YouAreKicked }
            LeaveReasonData::YouAreBanned => { LeaveReason::YouAreBanned }
        }
    }
}

impl From<LeaveReason> for LeaveReasonData {
    fn from(value: LeaveReason) -> Self {
        match value {
            LeaveReason::GameOver => { LeaveReasonData::GameOver }
            LeaveReason::ServerClosed => { LeaveReasonData::ServerClosed }
            LeaveReason::YouAreKicked => { LeaveReasonData::YouAreKicked }
            LeaveReason::YouAreBanned => { LeaveReasonData::YouAreBanned }
        }
    }
}

impl From<GameMsgC> for GameMessage {
    fn from(msg: GameMsgC) -> Self {
        match msg.tag {
            GameMsgCTag::SkinEventTrigger => unsafe {
                GameMessage::SkinEventTrigger(msg.data.key.key)
            }
            GameMsgCTag::DisableKey => unsafe {
                GameMessage::DisableKey(msg.data.key.key)
            }
            GameMsgCTag::EnableKey => unsafe {
                GameMessage::EnableKey(msg.data.key.key)
            }
            GameMsgCTag::Leave => unsafe {
                GameMessage::Leave(msg.data.reason.into())
            }
            GameMsgCTag::Err => {
                GameMessage::Err
            }
        }
    }
}

impl From<GameMessage> for GameMsgC {
    fn from(msg: GameMessage) -> Self {
        match msg {
            GameMessage::SkinEventTrigger(key) => {
                GameMsgC {
                    tag: GameMsgCTag::SkinEventTrigger,
                    data: GameMsgCUnion { key: KeyData { key } }
                }
            }
            GameMessage::DisableKey(key) => {
                GameMsgC {
                    tag: GameMsgCTag::DisableKey,
                    data: GameMsgCUnion { key: KeyData { key } }
                }
            }
            GameMessage::EnableKey(key) => {
                GameMsgC {
                    tag: GameMsgCTag::EnableKey,
                    data: GameMsgCUnion { key: KeyData { key } }
                }
            }
            GameMessage::Leave(reason) => {
                GameMsgC {
                    tag: GameMsgCTag::Leave,
                    data: GameMsgCUnion { reason: reason.into() }
                }
            }
            GameMessage::Err => {
                GameMsgC {
                    tag: GameMsgCTag::Err,
                    data: GameMsgCUnion { nul: null_mut() }
                }
            }
        }
    }
}