use std::net::{IpAddr, Ipv4Addr};
use nogamepads_lib_rs::DEFAULT_PORT;
use nogamepads_lib_rs::pad_data::game_profile::game_profile::GameProfile;
use nogamepads_lib_rs::pad_service::server::nogamepads_server::PadServer;

fn main() {

    // 简易构建服务端
    // let server = PadServer::build_simple();
    // server.start_listening_debug()

    // 构建服务端
    let server = PadServer::default()

        // 地址
        .addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEFAULT_PORT)

        // 配置信息
        .put_profile(
            GameProfile::default()
                .game_name("My Multiplayer Game")
                .game_description("My Game Description")
                .version("0.1 alpha")
                .to_owned()
        )

        // 键位注册
        .register_button(0, "Shoot")
        .register_button(1, "Jump")
        .register_direction(0, "Walk")
        .register_axis(0, "View Yaw")

        // 控制台
        .enable_console()
        // .quiet()
        .build();

    // 运行服务端
    server.start_server();
}