use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::ptr::null_mut;
use nogamepads_lib_rs::pad_data::game_profile::game_profile::GameProfile;
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::{ConnectionCallbackMessage, ConnectionErrorType};

#[repr(C)]
#[allow(unused_imports)]
pub struct ConnCallbackMsgC {
    tag: ConnCallbackMsgCTag,
    data: ConnCallbackMsgCUnion,
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
    profile: ManuallyDrop<GameProfile>,
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

impl From<ConnCallbackMsgC> for ConnectionCallbackMessage {
    fn from(mut value: ConnCallbackMsgC) -> Self {
        match value.tag {
            ConnCallbackMsgCTag::Profile => unsafe {
                let profile = ManuallyDrop::take(&mut value.data.profile);
                ConnectionCallbackMessage::Profile(profile)
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

impl From<ConnectionCallbackMessage> for ConnCallbackMsgC {
    fn from(value: ConnectionCallbackMessage) -> Self {
        match value {
            ConnectionCallbackMessage::Profile(profile) => {
                ConnCallbackMsgC {
                    tag: ConnCallbackMsgCTag::Profile,
                    data: ConnCallbackMsgCUnion {
                        profile: ManuallyDrop::new(profile)
                    }
                }
            }
            ConnectionCallbackMessage::Deny(conn_err) => {
                ConnCallbackMsgC {
                    tag: ConnCallbackMsgCTag::Deny,
                    data: ConnCallbackMsgCUnion { conn_err: conn_err.into() }
                }
            }
            ConnectionCallbackMessage::Fail(conn_err) => {
                ConnCallbackMsgC {
                    tag: ConnCallbackMsgCTag::Fail,
                    data: ConnCallbackMsgCUnion { conn_err: conn_err.into() }
                }
            }
            ConnectionCallbackMessage::Ok => {
                ConnCallbackMsgC {
                    tag: ConnCallbackMsgCTag::Ok,
                    data: ConnCallbackMsgCUnion { nul: null_mut() }
                }
            }
            ConnectionCallbackMessage::Welcome => {
                ConnCallbackMsgC {
                    tag: ConnCallbackMsgCTag::Welcome,
                    data: ConnCallbackMsgCUnion { nul: null_mut() }
                }
            }
            ConnectionCallbackMessage::Err => {
                ConnCallbackMsgC {
                    tag: ConnCallbackMsgCTag::Err,
                    data: ConnCallbackMsgCUnion { nul: null_mut() }
                }
            }
        }
    }
}