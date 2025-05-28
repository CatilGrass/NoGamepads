use bincode::config;
use bincode::config::Configuration;

pub mod pad_service;
pub mod pad_data;

pub const DEFAULT_PORT : u16 = 5989;
pub const BINCODE_CONVERT_FAILED : Vec<u8> = Vec::new();
pub const BINCODE_CONFIG : Configuration = config::standard();