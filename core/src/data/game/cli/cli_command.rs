use std::process::exit;
use std::sync::{Arc, Mutex};
use clap::{Args, Parser, Subcommand};
use clearscreen::clear;
use log::{info, warn};
use nogamepads::entry_mutex;
use crate::data::game::runtime::structs::GameRuntime;
use crate::data::message::traits::MessageManager;
use crate::data::player::structs::Player;

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

    LockGame,

    UnlockGame,

    #[command(about = "Close the game")]
    Close,

    #[command(about = "Exit the console")]
    Exit,

    OnlineList,

    BannedList,

    Ban(PlayerIndex),

    Pardon(PlayerIndex),

    Kick(PlayerIndex),

    Event(SendEventArgs),

    Message(SendMessageArgs),

    Pop,

    PopAll,
}

#[derive(Args, Debug)]
struct PlayerIndex {
    index: usize,
}

#[derive(Args, Debug)]
struct SendEventArgs {
    index: usize,
    event: u8
}

#[derive(Args, Debug)]
struct SendMessageArgs {
    index: usize,
    msg: String
}

pub fn process_game_cli(runtime: Arc<Mutex<GameRuntime>>, cmd: GameCli) {
    match cmd.command {
        Commands::Clear => {
            clear().expect("Failed to clear screen");

        }

        Commands::LockGame => {
            entry_mutex!(runtime, |guard| {
                guard.lock_game();
            })
        }

        Commands::UnlockGame => {
            entry_mutex!(runtime, |guard| {
                guard.unlock_game();
            })
        }

        Commands::Close => {
            entry_mutex!(runtime, |guard| {
                guard.close_game();
            })
        }

        Commands::Exit => {
            exit(1);
        }

        Commands::OnlineList => {
            entry_mutex!(runtime, |guard| {
                let mut i = 0;
                for account in guard.data.online_accounts() {
                    info!("{}.{}", i, account.id);
                    i += 1;
                }
            })
        }

        Commands::BannedList => {
            entry_mutex!(runtime, |guard| {
                let mut i = 0;
                for account in guard.data.banned_accounts() {
                    info!("{}.{}", i, account.id);
                    i += 1;
                }
            })
        }

        Commands::Ban(args) => {
            entry_mutex!(runtime, |guard| {
                if let Some(account) = guard.data.online_accounts().get(args.index) {
                    if let Some(service_type) = guard.data.get_service_type(account) {
                        guard.ban_player(&Player::from(account.clone()), service_type);
                        info!("Account {} banned.", account.id);
                    }
                } else {
                    warn!("Account number {} not found", args.index);
                }
            })
        }

        Commands::Pardon(args) => {
            entry_mutex!(runtime, |guard| {
                if let Some(account) = guard.data.banned_accounts().get(args.index) {
                    guard.pardon_player(&Player::from(account.clone()));
                    info!("Account {} pardoned.", account.id);
                } else {
                    warn!("Account number {} not found", args.index);
                }
            })
        }

        Commands::Kick(args) => {
            entry_mutex!(runtime, |guard| {
                if let Some(account) = guard.data.online_accounts().get(args.index) {
                    if let Some(service_type) = guard.data.get_service_type(account) {
                        guard.kick_player(&Player::from(account.clone()), service_type);
                        info!("Account {} kicked.", account.id);
                    }
                } else {
                    warn!("Account number {} not found", args.index);
                }
            })
        }

        Commands::Event(args) => {
            entry_mutex!(runtime, |guard| {
                if let Some(account) = guard.data.online_accounts().get(args.index) {
                    if let Some(service_type) = guard.data.get_service_type(account) {
                        guard.send_event(account, args.event, service_type);
                        info!("Sent event {} to {}.", args.event, account.id);
                    }
                } else {
                    warn!("Account number {} not found", args.index);
                }
            })
        }

        Commands::Message(args) => {
            entry_mutex!(runtime, |guard| {
                if let Some(account) = guard.data.online_accounts().get(args.index) {
                    if let Some(service_type) = guard.data.get_service_type(account) {
                        guard.send_message(account, args.msg.clone(), service_type);
                        info!("Sent message \"{}\" to {}.", args.msg, account.id);
                    }
                } else {
                    warn!("Account number {} not found", args.index);
                }
            })
        }

        Commands::Pop => {
            entry_mutex!(runtime, |guard| {
                if let Some((account, message)) = guard.pop_event() {
                    info!("{}: {:?}", account.id, message);
                } else {
                    info!("None")
                }
            })
        }

        Commands::PopAll => {
            entry_mutex!(runtime, |guard| {
                while let Some((account, message)) = guard.pop_event() {
                    info!("{}: {:?}", account.id, message);
                }
            })
        }
    }
}