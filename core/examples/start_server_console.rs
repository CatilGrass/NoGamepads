use nogamepads_core::pad_data::game_profile::game_profile::GameProfile;
use nogamepads_core::pad_service::server::nogamepads_server::PadServer;
use nogamepads_core::DEFAULT_PORT;
use std::net::{IpAddr, Ipv4Addr};

fn main() {

    // 简易构建服务端
    // let server = PadServer::build_simple();
    // server.start_listening_debug()

    // 构建服务端
    let server = PadServer::default()
        .addr(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        DEFAULT_PORT
        )
        .put_profile(
            GameProfile::default()
                .game_name("My Multiplayer Game")
                .game_description("My Game Description")
                .version("0.1 alpha")
                .to_owned()
        )
        .enable_console()
        // .quiet()
        .build();

    // 运行服务端
    server.start_server();
}