use bincode::config;
use bincode::config::Configuration;

pub const BINCODE_CONVERT_FAILED : Vec<u8> = Vec::new();
pub const BINCODE_CONFIG : Configuration = config::standard();

pub mod controller;
pub mod game;
pub mod message;
pub mod player;