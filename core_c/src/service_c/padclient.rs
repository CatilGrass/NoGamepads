use crate::data_c::control_message_c::ControlMessageC;
use crate::data_c::game_message_c::GameMessageC;
use crate::data_c::player_info_c::PlayerInfoC;
use crate::{box_into_raw, str_c_to_rs, str_rs_to_c};
use nogamepads_lib_rs::pad_data::pad_messages::nogamepads_messages::GameMessage;
use nogamepads_lib_rs::pad_service::client::nogamepads_client::PadClient;
use nogamepads_lib_rs::DEFAULT_PORT;
use std::ffi::c_char;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(unused_imports)]
pub struct PadClientC {
    target_address: *const c_char,
    target_port: u16,
    bind_player: PlayerInfoC,
    enable_console: bool,
    quiet: bool
}

impl From<PadClientC> for PadClient {
    fn from(value: PadClientC) -> Self {
        let address = IpAddr::from_str(str_c_to_rs(value.target_address).unwrap_or("".to_string()).as_str()).unwrap_or(
            IpAddr::from(Ipv4Addr::new(127, 0, 0, 1))
        );
        let port = value.target_port;
        let mut client = PadClient::bind_addr_with_port(address, port);
        client.bind_player(value.bind_player.into());
        if value.enable_console { client.enable_console(); }
        if value.quiet { client.quiet(); }
        client
    }
}

impl From<PadClient> for PadClientC {
    fn from(mut value: PadClient) -> Self {
        let (addr, port) = value.clone_addr();

        PadClientC {
            target_address: str_rs_to_c(addr.to_string()).1,
            target_port: port,
            bind_player: value.unbind_player().into(),
            enable_console: value.is_enable_console(),
            quiet: value.is_quiet(),
        }
    }
}
impl From<&mut PadClient> for PadClientC {
    fn from(value: &mut PadClient) -> Self {
        value.into()
    }
}

// 构建

#[unsafe(no_mangle)]
pub extern "C" fn init_client(address: *const c_char) -> PadClientC {
    let addr_str = str_c_to_rs(address).unwrap_or("127.0.0.1".to_string());
    let addr_v4 = Ipv4Addr::from_str(addr_str.as_str()).unwrap_or(
        Ipv4Addr::new(127, 0, 0, 1));
    PadClient::bind_addr_with_port(IpAddr::from(addr_v4), DEFAULT_PORT).into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_client_with_port(address: *const c_char, port: u16) -> PadClientC {
    let addr_str = str_c_to_rs(address).unwrap_or("127.0.0.1".to_string());
    let addr_v4 = Ipv4Addr::from_str(addr_str.as_str()).unwrap_or(
        Ipv4Addr::new(127, 0, 0, 1));
    PadClient::bind_addr_with_port(IpAddr::from(addr_v4), port).into()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_client_quiet(client: PadClientC) -> PadClientC {
    let mut raw: PadClient = client.into();
    raw.quiet().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn enable_client_console(client: PadClientC) -> PadClientC {
    let mut raw: PadClient = client.into();
    raw.enable_console().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn bind_player_to_client(client: PadClientC, info: PlayerInfoC) -> PadClientC {
    let mut raw: PadClient = client.into();
    raw.bind_player(info.into()).into()
}

#[unsafe(no_mangle)]
pub extern "C" fn complete(client: PadClientC) -> *mut PadClient {
    let client: PadClient = client.into();
    box_into_raw!(client)
}

// 状态控制

#[unsafe(no_mangle)]
pub extern "C" fn connect_client_to_server(client: &mut PadClient) {
    let client = unsafe { Box::from_raw(client) };
    client.connect();
}

#[unsafe(no_mangle)]
pub extern "C" fn exit_client_from_server(client: &PadClient) {
    client.exit_server();
}

// 消息管理

#[unsafe(no_mangle)]
pub extern "C" fn put_a_msg_to_server(client: &PadClient, msg: ControlMessageC) {
    client.put_msg(msg.into())
}

#[unsafe(no_mangle)]
pub extern "C" fn pop_a_msg_from_server(client: &PadClient) -> GameMessageC {
    client.pop_a_msg().unwrap_or(GameMessage::Err).into()
}

#[unsafe(no_mangle)]
pub extern "C" fn pop_msg_from_server_or(client: &PadClient, or: GameMessageC) -> GameMessageC {
    client.pop_msg_or(or.into()).into()
}