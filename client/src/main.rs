use bevy::prelude::App;
use bevy::prelude::ResMut;
use bevy::prelude::Startup;
use bevy::tasks::block_on;
use bevy::DefaultPlugins;
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
use nogamepads_lib_rs::pad_service::client::nogamepads_client::PadClient;
use std::net::{IpAddr, Ipv4Addr};
use tokio::runtime::Builder;

fn main() {
    let mut app = App::new();

    // 插件
    app.add_plugins(DefaultPlugins);
    app.add_plugins(TokioTasksPlugin {
        make_runtime: Box::new(|| {
            Builder::new_multi_thread()
                .enable_all()
                .build().unwrap()
        }),
        ..TokioTasksPlugin::default()
    });

    // 系统
    app.add_systems(Startup, client_init);

    app.run();
}

fn client_init(
    runtime: ResMut<TokioTasksRuntime>
) {
    let mut client =
        PadClient::bind_addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let mut player_info = PlayerInfo::new();
    player_info.setup_account_info(
        env!("TEST_PLAYER_ACCOUNT"),
        env!("TEST_PLAYER_PASSWORD"));
    player_info.set_nickname(env!("TEST_PLAYER_NICKNAME"));
    player_info.set_customize_color_hsv(320, 0.5, 1.0); // PINK

    client.bind_player(player_info); // 绑定玩家

    runtime.spawn_background_task(|context| async move {
        let entry = client.get_connect_entry();
        block_on(entry);
    });
}
