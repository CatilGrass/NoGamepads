use clap::CommandFactory;
use nogamepads::logger_utils::logger_build;
use nogamepads_core::data::game::game_cli::{process_game_cli, GameCli};
use nogamepads_core::data::game::structs::{GameData, GameRuntimeDataArchive};
use nogamepads_core::run_services;
use nogamepads_core::service::cli_addition::runtime_consoles::RuntimeConsole;
use nogamepads_core::service::tcp_network::pad_server::structs::PadServerNetwork;
use std::sync::Arc;
use log::LevelFilter::Trace;

fn main () {
    logger_build(Trace);

    // Initialize game data
    let mut game = GameData::new();
    game.name("My Game".to_string());
    game.version("0.1".to_string());
    game.load_data(
        GameRuntimeDataArchive {
            banned: vec![],
        }
    );

    // Generate runtime from game data
    let runtime = game.runtime();

    // Establish TCP pad_server using runtime
    let tcp_server = PadServerNetwork::build(Arc::clone(&runtime));

    let server = tcp_server.build_entry();

    // Cmd
    let cmd = RuntimeConsole::build(
        GameCli::command(),
        "gamecli".to_string(),
        Arc::clone(&runtime),
        |runtime, cmd: GameCli| {
            process_game_cli(runtime, cmd)
        }
    ).build_entry();

    run_services!(server, cmd);
}