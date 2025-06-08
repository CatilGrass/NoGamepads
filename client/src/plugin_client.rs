

// #[derive(Component)]
// struct ClientComponent {
//     pad_client: Arc<PadClient>,
// }

use bevy::prelude::{App, Plugin};

pub struct PadClientPlugin;
impl Plugin for PadClientPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(PreStartup, client_init);
        // app.add_systems(Startup, client_start);
        // app.add_systems(Update, client_exit.run_if(input_just_pressed(KeyCode::Escape)));
    }
}
//
// fn client_init(
//     mut commands: Commands
// ) {
//     // 构建玩家信息
//     let mut player_info = PlayerInfo::new();
//     player_info.setup_account_info(
//         env!("TEST_PLAYER_ACCOUNT"),
//         env!("TEST_PLAYER_PASSWORD"));
//     player_info.set_nickname(env!("TEST_PLAYER_NICKNAME"));
//     player_info.set_customize_color_hsv(320, 0.5, 1.0); // PINK
//
//     // 构建客户端
//     let mut pad_client = PadClient::default();
//     pad_client.bind_player(player_info);
//     pad_client.quiet();
//
//     // 将客户端放入实体
//     commands.spawn(
//         ClientComponent {
//             pad_client: pad_client.build(),
//         }
//     );
// }
//
// fn client_start(
//     client_components: Query<&mut ClientComponent>,
//     runtime: ResMut<TokioTasksRuntime>
// ) {
//     for client_component in client_components.iter() {
//         let pad_client = Arc::clone(&client_component.pad_client);
//
//         // 启动服务器
//         runtime.spawn_background_task(|_ctx|async move {
//             let entry = pad_client.get_connect_entry();
//             block_on(entry);
//             info!("Finished");
//         });
//     }
// }
//
// fn client_exit(
//     client_components: Query<&mut ClientComponent>
// ){
//     for client_component in client_components.iter() {
//         let pad_client = Arc::clone(&client_component.pad_client);
//         pad_client.request_disconnect();
//     }
// }