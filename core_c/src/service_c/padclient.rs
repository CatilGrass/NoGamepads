use std::ffi::c_char;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use nogamepads_lib_rs::DEFAULT_PORT;
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::GameMessage;
use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
use nogamepads_lib_rs::pad_service::client::nogamepads_client::PadClient;
use crate::{box_into_raw, c_char_to_string_safe};
use crate::data_c::control_message_c::CtrlMsgC;
use crate::data_c::game_message_c::GameMsgC;

// 构建

#[unsafe(no_mangle)]
pub extern "C" fn init_client(address: *const c_char) -> *mut PadClient {
    let addr_str = c_char_to_string_safe(address).unwrap_or("127.0.0.1".to_string());
    let addr_v4 = Ipv4Addr::from_str(addr_str.as_str()).unwrap_or(
        Ipv4Addr::new(127, 0, 0, 1));
    box_into_raw!(PadClient::bind_addr_with_port(IpAddr::from(addr_v4), DEFAULT_PORT))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_client_with_port(address: *const c_char, port: u16) -> *mut PadClient {
    let addr_str = c_char_to_string_safe(address).unwrap_or("127.0.0.1".to_string());
    let addr_v4 = Ipv4Addr::from_str(addr_str.as_str()).unwrap_or(
        Ipv4Addr::new(127, 0, 0, 1));
    box_into_raw!(PadClient::bind_addr_with_port(IpAddr::from(addr_v4), port))
}

#[unsafe(no_mangle)]
pub extern "C" fn set_client_quiet(client: &mut PadClient) {
    client.quiet();
}

#[unsafe(no_mangle)]
pub extern "C" fn enable_client_console(client: &mut PadClient) {
    client.enable_console();
}

#[unsafe(no_mangle)]
pub extern "C" fn bind_player_to_client(client: &mut PadClient, info: &mut PlayerInfo) {
    client.bind_player(info.clone());
}

// 状态控制

#[unsafe(no_mangle)]
pub extern "C" fn connect_client_to_server(client: *mut PadClient) {
    let client = unsafe { Box::from_raw(client) };
    client.connect();
}

#[unsafe(no_mangle)]
pub extern "C" fn exit_client_from_server(client: &PadClient) {
    client.exit_server();
}

// 消息管理

#[unsafe(no_mangle)]
pub extern "C" fn put_a_msg_to_server(client: &PadClient, msg: CtrlMsgC) {
    client.put_msg(msg.into())
}

#[unsafe(no_mangle)]
pub extern "C" fn pop_a_msg_from_server(client: &PadClient) -> GameMsgC {
    client.pop_a_msg().unwrap_or(GameMessage::Err).into()
}

#[unsafe(no_mangle)]
pub extern "C" fn pop_msg_from_server_or(client: &PadClient, or: GameMsgC) -> GameMsgC {
    client.pop_msg_or(or.into()).into()
}