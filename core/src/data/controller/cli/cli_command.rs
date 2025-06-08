use std::sync::{Arc, Mutex};
use clap::{Parser, Subcommand};
use nogamepads::entry_mutex;
use crate::data::controller::runtime::structs::ControllerRuntime;
use crate::data::message::enums::ControlMessage;
use crate::data::message::traits::MessageManager;
use crate::service::service_types::ServiceType::TCPConnection;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ControllerCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {

    #[command(about = "Clean the screen")]
    Clear,

    #[command(about = "Send a message")]
    Message,

    #[command(about = "Close the controller")]
    Close
}

pub fn process_controller_cli(runtime: Arc<Mutex<ControllerRuntime>>, cmd: ControllerCli) {

    match cmd.command {
        Commands::Clear => {

        }

        Commands::Message => {
            entry_mutex!(runtime, |guard| {
                guard.send(ControlMessage::Msg("fuck".to_string()), 0, TCPConnection);
            });
        }

        Commands::Close => {
            entry_mutex!(runtime, |guard| {
                guard.close();
            });
        }
    }
}