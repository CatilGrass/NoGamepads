use crate::data_c::common::KeyData;
use crate::{c_char_to_string_safe, string_to_c_char_safe};
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::ControlMessage;
use std::ffi::{c_char, c_void};
use std::ptr::null_mut;

#[repr(C)]
#[allow(unused_imports)]
pub struct CtrlMsgC {
    tag: CtrlMsgCTag,
    data: CtrlMsgCUnion
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

impl From<CtrlMsgC> for ControlMessage {
    fn from(msg_c: CtrlMsgC) -> Self {
        match msg_c.tag {
            CtrlMsgCTag::Msg => unsafe {
                let str = c_char_to_string_safe(msg_c.data.str.str);
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

impl From<ControlMessage> for CtrlMsgC {
    fn from(msg_rs: ControlMessage) -> Self {
        match msg_rs {
            ControlMessage::Msg(str_msg) => {
                CtrlMsgC {
                    tag: CtrlMsgCTag::Msg,
                    data: CtrlMsgCUnion {
                        str: StrData { str: string_to_c_char_safe(str_msg).1 },
                    }
                }
            }
            ControlMessage::Pressed(key) => {
                CtrlMsgC {
                    tag: CtrlMsgCTag::Pressed,
                    data: CtrlMsgCUnion {
                        key: KeyData { key },
                    }
                }
            }
            ControlMessage::Released(key) => {
                CtrlMsgC {
                    tag: CtrlMsgCTag::Released,
                    data: CtrlMsgCUnion {
                        key: KeyData { key },
                    }
                }
            }
            ControlMessage::Axis(key, ax) => {
                CtrlMsgC {
                    tag: CtrlMsgCTag::Axis,
                    data: CtrlMsgCUnion {
                        axis: AxisData { key, ax }
                    }
                }
            }
            ControlMessage::Dir(key, (x, y)) => {
                CtrlMsgC {
                    tag: CtrlMsgCTag::Dir,
                    data: CtrlMsgCUnion {
                        dir: DirData { key, x, y }
                    }
                }
            }
            ControlMessage::Exit => {
                CtrlMsgC {
                    tag: CtrlMsgCTag::Exit,
                    data: CtrlMsgCUnion {
                        nul: null_mut(),
                    }
                }
            }
            ControlMessage::Err => {
                CtrlMsgC {
                    tag: CtrlMsgCTag::Err,
                    data: CtrlMsgCUnion {
                        nul: null_mut(),
                    }
                }
            }
        }
    }
}