use std::sync::{Arc, Mutex};
use clap::{Parser, Subcommand};
use nogamepads::entry_mutex;
use crate::data::game::runtime::structs::GameRuntime;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GameCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {

    #[command(about = "Clean the screen")]
    Clear,

    #[command(about = "Close the game")]
    Close
}

pub fn process_game_cli(runtime: Arc<Mutex<GameRuntime>>, cmd: GameCli) {
    match cmd.command {
        Commands::Clear => {

        }

        Commands::Close => {
            entry_mutex!(runtime, |guard| {
                guard.close_game();
            })
        }
    }
}