use std::collections::HashMap;
use crate::pad_data::pad_messages::nogamepads_messages::{ControlMessage, GameMessage};
use crate::pad_io::server::nogamepads_server::PadServer;
use clap::{Args, Parser, Subcommand};
use std::ops::{Index};
use std::sync::{Arc, MutexGuard, PoisonError};
use log::{error, info};
use crate::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;

/// NoGamePads Server - Cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Psc {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {

    // 清屏
    #[command(about = "Clean the screen")]
    Clear,

    // 关闭服务器
    #[command(about = "Close the server")]
    Stop,

    // 展示所有玩家
    #[command(about = "List all online players")]
    List,

    // 展示所有封禁的玩家
    #[command(about = "List all banned players")]
    Banned,

    // 检查收到的消息
    #[command(about = "Check received")]
    Received(ReceivedArgs),

    // 取出一条消息
    #[command(about = "Pop a message")]
    Pop(PlayerArgs),

    // 踢出玩家
    #[command(about = "Kick a player")]
    Kick(PlayerArgs),

    // 封禁玩家
    #[command(about = "Ban a player")]
    Ban(PlayerArgs),

    // 解封（赦免）玩家
    #[command(about = "Pardon a player")]
    Pardon(PlayerArgs),

    // 激活事件触发器
    #[command(about = "Send SkinEventTrigger")]
    Event(EventArgs)
}

// 检查收到的消息
#[derive(Args, Debug)]
struct ReceivedArgs {

    #[arg(default_value = "0")]
    player: usize,

    #[arg(long)]
    list: bool,
}

/// 激活事件触发器 参数
#[derive(Args, Debug)]
struct EventArgs {

    // 玩家序号
    #[arg(value_name = "PLAYER_INDEX")]
    index: usize,

    // 事件编号
    #[arg(value_name = "CONTENT")]
    message: u8,
}

#[derive(Args, Debug)]
struct PlayerArgs {

    // 玩家序号
    #[arg(value_name = "PLAYER_INDEX")]
    index: usize
}

pub fn process_debug_cmd (cmd: Psc, server: Arc<PadServer>) {
    match cmd.command {

        Commands::Clear => {
            clearscreen::clear().expect("Failed to clear screen");
        }

        Commands::Stop => {
            server.stop_server();
        }

        Commands::List => {
            print_player_list(server.list_players());
        }

        Commands::Banned => {
            print_player_list(server.list_players_banned());
        }

        Commands::Received(args) => {
            let players = server.list_players().unwrap_or(Vec::new());
            let player = players.index(args.player.clamp(0, players.iter().count() -1));
            if args.list {
                for msg in server.list_received(player) {
                    info!("{:?}", msg);
                }
            } else {
                info!("Total {} messsage(s)!", server.list_received(player).iter().count());
            }
        }

        Commands::Pop(args) => {
            match get_player_by_index(&server, args.index) {
                None => {
                    error!("Pup message failed : Player index \"{}\" not found!", args.index);
                }
                Some(player) => {
                    let message = server.pop_msg_or(&player, ControlMessage::Err);
                    info!("{:?}", message);
                }
            }
        }

        Commands::Kick(args) => {
            let player = get_player_by_index(&server, args.index);
            if player.is_some() {
                let player = player.unwrap();
                server.kick_player(&player);
            }
        }

        Commands::Ban(args) => {
            let player = get_player_by_index(&server, args.index);
            if player.is_some() {
                let player = player.unwrap();
                server.ban_player(&player);
            }
        }

        Commands::Pardon(args) => {
            let player = get_player_by_ban_index(&server, args.index);
            if player.is_some() {
                let player = player.unwrap();
                server.pardon_player(&player);
            }
        }

        Commands::Event(args) => {
            put_to_list(server, args.index, GameMessage::SkinEventTrigger(args.message));
        }
    }
}

fn put_to_list(server: Arc<PadServer>, player_index: usize, message: GameMessage) {
    match get_player_by_index(&server, player_index) {
        None => {
            error!("Put message failed : Player index \"{}\" not found!", player_index);
        }
        Some(player) => {
            server.put_msg_to(message, &player);
        }
    }
}

fn get_player_by_index(server: &Arc<PadServer>, index: usize) -> Option<PlayerInfo> {
    let list = server.list_players().unwrap_or(Vec::new());
    let max = list.iter().count();
    let index = if max > 0 { index.clamp(0, max - 1) } else { 0 };

    let result = list.get(index).cloned();
    result
}

fn get_player_by_ban_index(server: &Arc<PadServer>, index: usize) -> Option<PlayerInfo> {
    let list = server.list_players_banned().unwrap_or(Vec::new());
    let max = list.iter().count();
    let index = if max > 0 { index.clamp(0, max - 1) } else { 0 };

    let result = list.get(index).cloned();
    result
}

fn print_player_list(list: Result<Vec<PlayerInfo>, PoisonError<MutexGuard<HashMap<String, PlayerInfo>>>>) {
    let list = list.unwrap_or(Vec::new());
    let mut i = 0;
    for player in list {
        let n = player.customize.nickname;
        info!("({}){} ", i, n);
        i += 1;
    }
}