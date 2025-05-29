use crate::data_c::connection_message_c::ConnMsgCTag::{Connection, Ready, RequestLayoutConfigure, RequestProfile, RequestSkinPackage};
use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::ptr::null_mut;
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::ConnectionMessage;
use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;

#[repr(C)]
#[allow(unused_imports)]
pub struct ConnectionMessageC {
    pub tag: ConnMsgCTag,
    pub data: ConnMsgCUnion
}

#[repr(C)]
#[allow(unused_imports)]
pub enum ConnMsgCTag {
    Connection, RequestProfile, RequestLayoutConfigure, RequestSkinPackage, Ready, Err
}

#[repr(C)]
#[allow(unused_imports)]
pub union ConnMsgCUnion {
    nul: *mut c_void,
    info: ManuallyDrop<PlayerInfo>
}

impl From<ConnectionMessageC> for ConnectionMessage {
    fn from(mut value: ConnectionMessageC) -> Self {
        match value.tag {
            Connection => unsafe {
                let info = ManuallyDrop::take(&mut value.data.info);
                ConnectionMessage::Connection(info)
            }
            RequestProfile => {
                ConnectionMessage::RequestProfile
            }
            RequestLayoutConfigure => {
                ConnectionMessage::RequestLayoutConfigure
            }
            RequestSkinPackage => {
                ConnectionMessage::RequestSkinPackage
            }
            Ready => {
                ConnectionMessage::Ready
            }
            ConnMsgCTag::Err => {
                ConnectionMessage::Err
            }
        }
    }
}

impl From<ConnectionMessage> for ConnectionMessageC {
    fn from(value: ConnectionMessage) -> Self {
        match value {
            ConnectionMessage::Connection(info) => {
                ConnectionMessageC {
                    tag: Connection,
                    data: ConnMsgCUnion { info: ManuallyDrop::new(info) }
                }
            }
            ConnectionMessage::RequestProfile => {
                ConnectionMessageC {
                    tag: RequestProfile,
                    data: ConnMsgCUnion { nul: null_mut() }
                }
            }
            ConnectionMessage::RequestLayoutConfigure => {
                ConnectionMessageC {
                    tag: RequestLayoutConfigure,
                    data: ConnMsgCUnion { nul: null_mut() }
                }
            }
            ConnectionMessage::RequestSkinPackage => {
                ConnectionMessageC {
                    tag: RequestSkinPackage,
                    data: ConnMsgCUnion { nul: null_mut() }
                }
            }
            ConnectionMessage::Ready => {
                ConnectionMessageC {
                    tag: Ready,
                    data: ConnMsgCUnion { nul: null_mut() }
                }
            }
            ConnectionMessage::Err => {
                ConnectionMessageC {
                    tag: ConnMsgCTag::Err,
                    data: ConnMsgCUnion { nul: null_mut() }
                }
            }
        }
    }
}