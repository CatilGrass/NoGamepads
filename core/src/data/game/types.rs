use std::collections::HashMap;
use std::sync::Mutex;
use crate::data::player::player_data::{Account, Player};

pub(crate) type GameInfo = HashMap<String, String>;

pub(crate) type Players = Mutex<HashMap<Account, Player>>;