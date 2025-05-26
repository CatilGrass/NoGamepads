use crate::pad_io::client::nogamepads_client::PadClient;
use crate::pad_data::pad_messages::nogamepads_messages::{ControlMessage, GameMessage};
use clap::{Args, Parser, Subcommand};
use std::sync::Arc;

/// NoGamePads Client - Cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Pcc {
    #[command(subcommand)]
    command: Commands,
}

/// 主要命令
#[derive(Subcommand, Debug)]
enum Commands {

    // 清屏
    #[command(about = "Clean the screen")]
    Clear,

    // 断开当前连接
    #[command(about = "Exit from server")]
    Exit,

    // 检查收到的消息
    #[command(about = "Check received")]
    Received(ReceivedArgs),

    // 取出一条消息
    #[command(about = "Pop a message")]
    Pop(PopArgs),

    // 发送消息
    #[command(about = "Send Message")]
    Msg(MsgArgs),
}

#[derive(Args, Debug)]
struct ReceivedArgs {

    #[arg(long)]
    list: bool
}

/// 发送消息 参数
#[derive(Args, Debug)]
struct MsgArgs {

    // 消息内容
    #[arg(value_name = "CONTENT")]
    message: String,
}

#[derive(Args, Debug)]
struct PopArgs {  }

pub fn process_debug_cmd (cmd: Pcc, client: Arc<PadClient>) {
    match cmd.command {
        Commands::Clear => {
            clearscreen::clear().expect("Failed to clear screen");
        }

        Commands::Exit => {
            client.exit_server();
        }

        Commands::Received(args) => {
            if args.list {
                for msg in client.list_received() {
                    println!("{:?}", msg);
                }
            } else {
                println!("Total {} messsage(s)!", client.list_received().iter().count());
            }
        }

        Commands::Pop(_args) => {
            println!("{:?}", client.pop_msg_or(GameMessage::Err));
        }

        Commands::Msg(args) => {
            client.put_msg(ControlMessage::Msg(args.message));
        }
    }
}

#[allow(dead_code)]
fn put_to_list(client: Arc<PadClient>, message: ControlMessage) {
    client.put_msg(message);
}