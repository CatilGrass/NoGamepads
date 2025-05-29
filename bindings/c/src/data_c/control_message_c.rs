use crate::data_c::common::KeyData;
use crate::{str_c_to_rs, str_rs_to_c};
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::ControlMessage;
use std::ffi::{c_char, c_void};
use std::ptr::null_mut;

#[repr(C)]
#[allow(unused_imports)]
pub struct ControlMessageC {
    pub tag: CtrlMsgCTag,
    pub data: CtrlMsgCUnion
}

#[repr(C)]
#[allow(unused_imports)]
pub enum CtrlMsgCTag {
    Msg, Pressed, Released, Axis, Dir, Exit, Err
}

#[repr(C)]
#[allow(unused_imports)]
pub union CtrlMsgCUnion {
    nul: *mut c_void,
    key: KeyData,
    axis: AxisData,
    dir: DirData,
    str: StrData
}

#[repr(C)]
#[allow(unused_imports)]
#[derive(Copy, Clone)]
pub struct AxisData {
    key: u8,
    ax: f64
}

#[repr(C)]
#[allow(unused_imports)]
#[derive(Copy, Clone)]
pub struct DirData {
    key: u8,
    x: f64,
    y: f64
}

#[repr(C)]
#[allow(unused_imports)]
#[derive(Copy, Clone)]
pub struct StrData {
    str: *const c_char
}

impl From<ControlMessageC> for ControlMessage {
    fn from(msg_c: ControlMessageC) -> Self {
        match msg_c.tag {
            CtrlMsgCTag::Msg => unsafe {
                let str = str_c_to_rs(msg_c.data.str.str);
                if str.is_some() {
                    ControlMessage::Msg(str.unwrap())
                } else {
                    ControlMessage::Err
                }
            }
            CtrlMsgCTag::Pressed => unsafe {
                ControlMessage::Pressed(msg_c.data.key.key)
            }
            CtrlMsgCTag::Released => unsafe {
                ControlMessage::Released(msg_c.data.key.key)
            }
            CtrlMsgCTag::Axis => unsafe {
                let data = msg_c.data.axis;
                ControlMessage::Axis(data.key, data.ax)
            }
            CtrlMsgCTag::Dir => unsafe {
                let data = msg_c.data.dir;
                ControlMessage::Dir(data.key, (data.x, data.y))
            }
            CtrlMsgCTag::Exit => {
                ControlMessage::Exit
            }
            CtrlMsgCTag::Err => {
                ControlMessage::Err
            }
        }
    }
}

impl From<ControlMessage> for ControlMessageC {
    fn from(msg_rs: ControlMessage) -> Self {
        match msg_rs {
            ControlMessage::Msg(str_msg) => {
                ControlMessageC {
                    tag: CtrlMsgCTag::Msg,
                    data: CtrlMsgCUnion {
                        str: StrData { str: str_rs_to_c(str_msg).1 },
                    }
                }
            }
            ControlMessage::Pressed(key) => {
                ControlMessageC {
                    tag: CtrlMsgCTag::Pressed,
                    data: CtrlMsgCUnion {
                        key: KeyData { key },
                    }
                }
            }
            ControlMessage::Released(key) => {
                ControlMessageC {
                    tag: CtrlMsgCTag::Released,
                    data: CtrlMsgCUnion {
                        key: KeyData { key },
                    }
                }
            }
            ControlMessage::Axis(key, ax) => {
                ControlMessageC {
                    tag: CtrlMsgCTag::Axis,
                    data: CtrlMsgCUnion {
                        axis: AxisData { key, ax }
                    }
                }
            }
            ControlMessage::Dir(key, (x, y)) => {
                ControlMessageC {
                    tag: CtrlMsgCTag::Dir,
                    data: CtrlMsgCUnion {
                        dir: DirData { key, x, y }
                    }
                }
            }
            ControlMessage::Exit => {
                ControlMessageC {
                    tag: CtrlMsgCTag::Exit,
                    data: CtrlMsgCUnion {
                        nul: null_mut(),
                    }
                }
            }
            ControlMessage::Err => {
                ControlMessageC {
                    tag: CtrlMsgCTag::Err,
                    data: CtrlMsgCUnion {
                        nul: null_mut(),
                    }
                }
            }
        }
    }
}