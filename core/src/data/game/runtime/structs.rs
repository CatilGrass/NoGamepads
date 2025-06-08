use std::collections::{HashMap, VecDeque};
use std::sync::atomic::AtomicBool;
use crate::data::game::structs::GameControlData;
use crate::data::game::types::{GameInfo, Players};
use crate::data::message::enums::{ControlMessage, GameMessage};
use crate::data::player::structs::Account;
use crate::service::service_types::ServiceType;

/// Game pad_client runtime
/// Stores the game state, player information, and all data involved in controller-side interactions during runtime
pub struct GameRuntime {

    pub info: GameInfo,
    pub data: GameRuntimeData,
    pub control: GameControlRuntime,

    pub writer_count: i32,
    pub reader_count: i32,
}

pub struct GameRuntimeData {

    pub(crate) received: HashMap<(ServiceType, Account), VecDeque<(Account, ControlMessage)>>,
    pub(crate) send: HashMap<(ServiceType, Account), VecDeque<(Account, GameMessage)>>,

    pub(crate) players_online: Players,
    pub(crate) players_banned: Players,

    pub locked: AtomicBool,
    pub close: AtomicBool,
}

#[derive(Default)]
pub struct GameControlRuntime {
    pub(crate) keys: GameControlData,
    pub(crate) directions : HashMap<u8, HashMap<Account, (f64, f64)>>,
    pub(crate) axes : HashMap<u8, HashMap<Account, f64>>,
    pub(crate) button : HashMap<u8, HashMap<Account, bool>>,
    pub(crate) events : VecDeque<(Account, ControlMessage)>
}