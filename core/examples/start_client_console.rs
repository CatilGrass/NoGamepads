use std::net::{IpAddr, Ipv4Addr};
use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
use nogamepads_lib_rs::pad_service::client::nogamepads_client::PadClient;

fn main() {

    // 构建玩家信息
    let mut player_info = PlayerInfo::new();
    player_info.setup_account_info(
        env!("TEST_PLAYER_ACCOUNT"),
        env!("TEST_PLAYER_PASSWORD"));
    player_info.set_nickname(env!("TEST_PLAYER_NICKNAME"));
    player_info.set_customize_color_hsv(320, 0.5, 1.0); // PINK

    // 构建客户端
    let mut client = PadClient::bind_addr(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    );
    client.bind_player(player_info); // 绑定玩家
    client.enable_console(); // 启用控制台

    // 连接至目标地址
    client.connect();
}