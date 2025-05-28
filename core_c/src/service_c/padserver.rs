use std::collections::HashMap;
use crate::{box_into_raw, c_char_to_string_safe};
use nogamepads_lib_rs::pad_data::game_profile::game_profile::GameProfile;
use nogamepads_lib_rs::pad_service::server::nogamepads_server::PadServer;
use nogamepads_lib_rs::DEFAULT_PORT;
use std::ffi::c_char;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::sync::{Arc, MutexGuard, PoisonError};
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::ControlMessage;
use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
use crate::data_c::control_message_c::CtrlMsgC;
use crate::data_c::game_message_c::GameMsgC;
use crate::data_c::player_info_list_c::PlayerList;

// 构建

#[unsafe(no_mangle)]
pub extern "C" fn init_server(address: *const c_char) -> *mut PadServer {
    let addr_str = c_char_to_string_safe(address).unwrap_or("127.0.0.1".to_string());
    let addr_v4 = Ipv4Addr::from_str(addr_str.as_str()).unwrap_or(
        Ipv4Addr::new(127, 0, 0, 1));
    let mut server = PadServer::default();
    server.addr(IpAddr::from(addr_v4), DEFAULT_PORT);
    box_into_raw!(server)
}

#[unsafe(no_mangle)]
pub extern "C" fn init_server_with_port(address: *const c_char, port: u16) -> *mut PadServer {
    let addr_str = c_char_to_string_safe(address).unwrap_or("127.0.0.1".to_string());
    let addr_v4 = Ipv4Addr::from_str(addr_str.as_str()).unwrap_or(
        Ipv4Addr::new(127, 0, 0, 1));
    let mut server = PadServer::default();
    server.addr(IpAddr::from(addr_v4), port);
    box_into_raw!(server)
}

#[unsafe(no_mangle)]
pub extern "C" fn set_server_quiet(server: &mut PadServer) {
    server.quiet();
}

#[unsafe(no_mangle)]
pub extern "C" fn enable_server_console(server: &mut PadServer) {
    server.enable_console();
}

#[unsafe(no_mangle)]
pub extern "C" fn bind_profile_to_server(server: &mut PadServer, profile: &mut GameProfile) {
    server.put_profile(profile.clone());
}

// 状态控制

#[unsafe(no_mangle)]
pub extern "C" fn start_server(server: *mut PadServer) {
    let server = unsafe { Box::from_raw(server) };
    let arc = Arc::new(server.as_ref().clone());
    PadServer::start_server(arc);
}

#[unsafe(no_mangle)]
pub extern "C" fn stop_server(server: *mut PadServer) {
    let server = unsafe { Box::from_raw(server) };
    server.stop_server();
}

#[unsafe(no_mangle)]
pub extern "C" fn lock_game_on_server(server: *mut PadServer) {
    let server = unsafe { Box::from_raw(server) };
    server.lock_game();
}

#[unsafe(no_mangle)]
pub extern "C" fn unlock_game_on_server(server: *mut PadServer) {
    let server = unsafe { Box::from_raw(server) };
    server.unlock_game();
}

#[unsafe(no_mangle)]
pub extern "C" fn is_server_game_locked(server: *mut PadServer) -> bool {
    let server = unsafe { Box::from_raw(server) };
    server.is_game_locked()
}

// 消息管理
#[unsafe(no_mangle)]
pub extern "C" fn put_a_msg_to_player(server: *mut PadServer, msg: GameMsgC, player: &PlayerInfo) {
    let server = unsafe { Box::from_raw(server) };
    server.put_msg_to(msg.into(), player);
}

#[unsafe(no_mangle)]
pub extern "C" fn put_msg_to_all_players(server: *mut PadServer, msg: GameMsgC) {
    let server = unsafe { Box::from_raw(server) };
    server.put_msg_to_all(&msg.into());
}

#[unsafe(no_mangle)]
pub extern "C" fn pop_a_msg_from_player(server: *mut PadServer, player: &PlayerInfo) -> CtrlMsgC {
    let server = unsafe { Box::from_raw(server) };
    server.pop_a_msg(player).unwrap_or(ControlMessage::Err).into()
}

#[unsafe(no_mangle)]
pub extern "C" fn pop_msg_from_player_or(server: *mut PadServer, player: &PlayerInfo, or: CtrlMsgC) -> CtrlMsgC {
    let server = unsafe { Box::from_raw(server) };
    let msg = server.pop_msg_or(player, or.into());
    msg.into()
}

// 玩家管理
#[unsafe(no_mangle)]
pub extern "C" fn is_player_online(server: *mut PadServer, player: &PlayerInfo) -> bool {
    let server = unsafe { Box::from_raw(server) };
    server.is_player_online(player)
}

#[unsafe(no_mangle)]
pub extern "C" fn is_player_banned(server: *mut PadServer, player: &PlayerInfo) -> bool {
    let server = unsafe { Box::from_raw(server) };
    server.is_player_banned(player)
}

#[unsafe(no_mangle)]
pub extern "C" fn kick_player(server: *mut PadServer, player: &PlayerInfo) {
    let server = unsafe { Box::from_raw(server) };
    server.kick_player(player);
}

#[unsafe(no_mangle)]
pub extern "C" fn ban_player(server: *mut PadServer, player: &PlayerInfo) {
    let server = unsafe { Box::from_raw(server) };
    server.ban_player(player);
}

#[unsafe(no_mangle)]
pub extern "C" fn pardon_player(server: *mut PadServer, player: &PlayerInfo) {
    let server = unsafe { Box::from_raw(server) };
    server.pardon_player(player);
}

#[unsafe(no_mangle)]
pub extern "C" fn list_online_players(server: *mut PadServer) -> PlayerList {
    let server = unsafe { Box::from_raw(server) };
    let list = server.list_players();
    process_list_result(list)
}

#[unsafe(no_mangle)]
pub extern "C" fn list_banned_players(server: *mut PadServer) -> PlayerList {
    let server = unsafe { Box::from_raw(server) };
    let list = server.list_players_banned();
    process_list_result(list)
}

fn process_list_result(list: Result<Vec<PlayerInfo>, PoisonError<MutexGuard<HashMap<String, PlayerInfo>>>>) -> PlayerList {
    let result : Vec<PlayerInfo>;
    if list.is_ok() {
        result = list.unwrap();
    } else {
        result = vec![];
    }

    let mut vec = result.into_boxed_slice();
    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    let capacity = vec.len();

    PlayerList {
        players: ptr,
        len,
        capacity
    }
}