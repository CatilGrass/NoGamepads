use std::sync::{Arc, Mutex};
use bevy::log::info;
use bevy::prelude::{App, Commands, Component, OnEnter, Plugin, PreStartup, Query, ResMut, Startup, States};
use bevy_tokio_tasks::TokioTasksRuntime;
use nogamepads::entry_mutex;
use nogamepads_core::data::controller::controller_data::ControllerData;
use nogamepads_core::data::controller::controller_runtime::ControllerRuntime;
use nogamepads_core::data::player::player_data::Player;
use nogamepads_core::service::tcp_network::pad_client::pad_client_service::PadClientNetwork;
use crate::bevy_plugins::plugin_client_app::GameState::Exiting;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Waiting,
    Playing,
    Exiting
}

#[derive(Component)]
struct ClientComponent {
    pad_client: Arc<Mutex<ControllerRuntime>>,
}

pub struct ClientAppPlugins;
impl Plugin for ClientAppPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, client_init);
        app.add_systems(Startup, client_start);
        app.add_systems(OnEnter(Exiting), client_close);
    }
}

fn client_init(
    mut commands: Commands
) {
    let mut controller_data = ControllerData::default();
    controller_data.bind_player(
        Player::register(
            env!("TEST_PLAYER_ACCOUNT").to_string(),
            env!("TEST_PLAYER_PASSWORD").to_string())
    );

    let runtime = controller_data.runtime();

    commands.spawn(
        ClientComponent {
            pad_client: Arc::clone(&runtime),
        }
    );
}

fn client_start(
    client_components: Query<&mut ClientComponent>,
    runtime: ResMut<TokioTasksRuntime>
) {
    for client_component in client_components.iter() {
        let client_runtime = Arc::clone(&client_component.pad_client);
        let network = PadClientNetwork::build(client_runtime);
        let entry = network.build_entry();

        runtime.spawn_background_task(|_ctx| async move {
            entry.await;
            info!("Client finished.")
        });
    }
}

fn client_close(
    client_components: Query<&mut ClientComponent>
) {
    for client_component in client_components.iter() {
        entry_mutex!(client_component.pad_client, |guard| {
            guard.close();
        })
    };
}
