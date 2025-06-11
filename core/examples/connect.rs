use std::sync::Arc;
use log::LevelFilter::Trace;
use nogamepads::logger_utils::logger_build;
use nogamepads_core::data::controller::structs::ControllerData;
use nogamepads_core::data::player::structs::Player;
use nogamepads_core::service::tcp_network::pad_client::structs::PadClientNetwork;
use nogamepads_core::service::tcp_network::utils::tokio_utils::build_tokio_runtime;

fn main () {
    logger_build(Trace);

    let mut player = Player::register(
        env!("TEST_PLAYER_ACCOUNT").to_string(),
        env!("TEST_PLAYER_PASSWORD").to_string()
    );

    player.nickname(&env!("TEST_PLAYER_NICKNAME").to_string());

    let mut controller = ControllerData::default();
    controller.bind_player(player);

    let runtime = controller.runtime();

    let client = PadClientNetwork::build(Arc::clone(&runtime));

    build_tokio_runtime("Tokio Runtime".to_string()).block_on(client.build_entry())
}