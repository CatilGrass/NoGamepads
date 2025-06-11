use bincode::config;
use bincode::config::Configuration;

pub mod data;
pub mod service;

pub const BINCODE_CONVERT_FAILED : Vec<u8> = Vec::new();
pub const BINCODE_CONFIG : Configuration = config::standard();
