use crate::data_c::game_profile_c::GameProfileC;
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::{ConnectionCallbackMessage, ConnectionErrorType};
use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::ptr::null_mut;

#[repr(C)]
#[allow(unused_imports)]
pub struct ConnectionCallbackMessageC {
    pub tag: ConnCallbackMsgCTag,
    pub data: ConnCallbackMsgCUnion,
}

#[repr(C)]
#[allow(unused_imports)]
pub enum ConnCallbackMsgCTag {
    Profile, Deny, Fail, Ok, Welcome, Err
}

#[repr(C)]
#[allow(unused_imports)]
pub union ConnCallbackMsgCUnion {
    nul: *mut c_void,
    profile: ManuallyDrop<GameProfileC>,
    conn_err: ConnErrTypeC
}

#[repr(C)]
#[allow(unused_imports)]
#[derive(Copy, Clone)]
pub enum ConnErrTypeC {
    ContainSamePlayer, PlayerBanned, Timeout, GameLocked, WhatTheHell
}

impl From<ConnErrTypeC> for ConnectionErrorType {
    fn from(value: ConnErrTypeC) -> Self {
        match value {
            ConnErrTypeC::ContainSamePlayer => { ConnectionErrorType::ContainSamePlayer }
            ConnErrTypeC::PlayerBanned => { ConnectionErrorType::PlayerBanned }
            ConnErrTypeC::Timeout => { ConnectionErrorType::Timeout }
            ConnErrTypeC::GameLocked => { ConnectionErrorType::GameLocked }
            ConnErrTypeC::WhatTheHell => { ConnectionErrorType::WhatTheHell }
        }
    }
}

impl From<ConnectionErrorType> for ConnErrTypeC {
    fn from(value: ConnectionErrorType) -> Self {
        match value {
            ConnectionErrorType::ContainSamePlayer => { ConnErrTypeC::ContainSamePlayer }
            ConnectionErrorType::PlayerBanned => { ConnErrTypeC::PlayerBanned }
            ConnectionErrorType::Timeout => { ConnErrTypeC::Timeout }
            ConnectionErrorType::GameLocked => { ConnErrTypeC::GameLocked }
            ConnectionErrorType::WhatTheHell => { ConnErrTypeC::WhatTheHell }
        }
    }
}

impl From<ConnectionCallbackMessageC> for ConnectionCallbackMessage {
    fn from(mut value: ConnectionCallbackMessageC) -> Self {
        match value.tag {
            ConnCallbackMsgCTag::Profile => unsafe {
                let profile = ManuallyDrop::take(&mut value.data.profile);
                ConnectionCallbackMessage::Profile(profile.into())
            }
            ConnCallbackMsgCTag::Deny => unsafe {
                ConnectionCallbackMessage::Deny(value.data.conn_err.into())
            }
            ConnCallbackMsgCTag::Fail => unsafe {
                ConnectionCallbackMessage::Fail(value.data.conn_err.into())
            }
            ConnCallbackMsgCTag::Ok => {
                ConnectionCallbackMessage::Ok

            }
            ConnCallbackMsgCTag::Welcome => {
                ConnectionCallbackMessage::Welcome
            }
            ConnCallbackMsgCTag::Err => {
                ConnectionCallbackMessage::Err
            }
        }
    }
}

impl From<ConnectionCallbackMessage> for ConnectionCallbackMessageC {
    fn from(value: ConnectionCallbackMessage) -> Self {
        match value {
            ConnectionCallbackMessage::Profile(profile) => {
                ConnectionCallbackMessageC {
                    tag: ConnCallbackMsgCTag::Profile,
                    data: ConnCallbackMsgCUnion {
                        profile: ManuallyDrop::new(profile.into())
                    }
                }
            }
            ConnectionCallbackMessage::Deny(conn_err) => {
                ConnectionCallbackMessageC {
                    tag: ConnCallbackMsgCTag::Deny,
                    data: ConnCallbackMsgCUnion { conn_err: conn_err.into() }
                }
            }
            ConnectionCallbackMessage::Fail(conn_err) => {
                ConnectionCallbackMessageC {
                    tag: ConnCallbackMsgCTag::Fail,
                    data: ConnCallbackMsgCUnion { conn_err: conn_err.into() }
                }
            }
            ConnectionCallbackMessage::Ok => {
                ConnectionCallbackMessageC {
                    tag: ConnCallbackMsgCTag::Ok,
                    data: ConnCallbackMsgCUnion { nul: null_mut() }
                }
            }
            ConnectionCallbackMessage::Welcome => {
                ConnectionCallbackMessageC {
                    tag: ConnCallbackMsgCTag::Welcome,
                    data: ConnCallbackMsgCUnion { nul: null_mut() }
                }
            }
            ConnectionCallbackMessage::Err => {
                ConnectionCallbackMessageC {
                    tag: ConnCallbackMsgCTag::Err,
                    data: ConnCallbackMsgCUnion { nul: null_mut() }
                }
            }
        }
    }
}