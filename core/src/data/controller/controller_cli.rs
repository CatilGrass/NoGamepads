use std::sync::{Arc, Mutex};
use clap::{Args, Parser, Subcommand};
use clearscreen::clear;
use log::info;
use nogamepads::entry_mutex;
use crate::data::controller::controller_runtime::ControllerRuntime;

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

    #[command(about = "Close the controller")]
    Close,

    #[command(about = "Send a message")]
    Message(MessageArgs),

    #[command(about = "Press a button")]
    Press(ButtonArgs),

    #[command(about = "Release a button")]
    Release(ButtonArgs),

    #[command(about = "Change a axis value")]
    Axis(AxisArgs),

    #[command(about = "Change a direction value")]
    Direction(DirectionArgs),

    Pop,

    PopAll
}

#[derive(Args, Debug)]
struct MessageArgs {
    message: String,
}

#[derive(Args, Debug)]
struct ButtonArgs {
    button_key: u8
}

#[derive(Args, Debug)]
struct AxisArgs {
    axis_key: u8,
    axis_value: f64
}

#[derive(Args, Debug)]
struct DirectionArgs {
    dir_key: u8,
    x: f64,
    y: f64
}

pub fn process_controller_cli(runtime: Arc<Mutex<ControllerRuntime>>, cmd: ControllerCli) -> bool {

    match cmd.command {
        Commands::Clear => {
            clear().expect("Failed to clear screen");
        }

        Commands::Close => {
            entry_mutex!(runtime, |guard| {
                guard.close();
            });
            return false;
        }

        Commands::Message(args) => {
            entry_mutex!(runtime, |guard| {
                guard.message(args.message);
            });
        }

        Commands::Press(args) => {
            entry_mutex!(runtime, |guard| {
                guard.press_button(args.button_key);
            });
        }

        Commands::Release(args) => {
            entry_mutex!(runtime, |guard| {
                guard.release_button(args.button_key);
            });
        }

        Commands::Axis(args) => {
            entry_mutex!(runtime, |guard| {
                guard.change_axis(args.axis_key, args.axis_value);
            });
        }

        Commands::Direction(args) => {
            entry_mutex!(runtime, |guard| {
                guard.change_direction(args.dir_key, args.x, args.y);
            });
        }

        Commands::Pop => {
            entry_mutex!(runtime, |guard| {
                if let Some(msg) = guard.pop() {
                    info!("{:?}", msg);
                } else {
                    info!("None");
                }
            });
        }

        Commands::PopAll => {
            entry_mutex!(runtime, |guard| {
                while let Some(msg) = guard.pop() {
                    info!("{:?}", msg);
                }
            });
        }
    }
    true
}